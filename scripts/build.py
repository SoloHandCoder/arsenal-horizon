#!/usr/bin/env python3
"""Builds Horizon's game with Arsenal in it, the way Horizon's launcher builds native mods (its docs/native-mods.md):
Horizon's source at a commit (GitHub's archive of it), horizon-mixins at a pinned commit, the bundle in
mixins/arsenal applied, and horizon-client compiled the way Horizon's own release is (game-build.yml).

    python scripts/build.py --commit <sha> --version nightly-2026-11-03 --nightly "2026-11-03 2944480" \
        --asset arsenal-horizon-windows-x64.zip

Leaves the zip (the horizon-client program only) in dist/. Everything else goes under work/.
"""
import argparse
import hashlib
import io
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
import urllib.request
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WORK = ROOT / "work"
DIST = ROOT / "dist"
COMPAT = json.loads((ROOT / "compatibility.json").read_text())
EXE = ".exe" if os.name == "nt" else ""


def log(*args):
    print(*args, flush=True)


def run(cmd, **kw):
    log("+", " ".join(str(c) for c in cmd))
    subprocess.run([str(c) for c in cmd], check=True, **kw)


def source(commit):
    out = WORK / "horizon" / commit
    if (out / ".horizon-revision").is_file():
        return out
    log(f"Downloading Horizon {commit[:7]}")
    request = urllib.request.Request(f"https://codeload.github.com/horizon-rust/horizon/tar.gz/{commit}",
                                     headers={"User-Agent": "arsenal-horizon"})
    with urllib.request.urlopen(request, timeout=600) as response:
        data = response.read()
    partial = WORK / "horizon" / f".{commit}.partial"
    shutil.rmtree(partial, ignore_errors=True)
    partial.mkdir(parents=True)
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        # Files and folders only: horizon-mixins refuses links, and none are needed to build.
        members = [m for m in archive.getmembers() if m.isfile() or m.isdir()]
        archive.extractall(partial, members=members, filter="data")
    top = next(partial.iterdir())
    (top / ".horizon-revision").write_text(commit)
    shutil.rmtree(out, ignore_errors=True)
    out.parent.mkdir(parents=True, exist_ok=True)
    top.rename(out)
    shutil.rmtree(partial)
    return out


def mixins_tool():
    rev = COMPAT["horizon_mixins_revision"]
    root = WORK / "horizon-mixins" / rev
    binary = root / "bin" / f"horizon-mixins{EXE}"
    if not binary.is_file():
        run(["cargo", "install", "--locked", "--git", COMPAT["horizon_mixins_repository"], "--rev", rev, "--root", root,
             "horizon-mixins-cli"])
    return binary


def has_mods_feature(stage):
    text = (stage / "crates/horizon-client/Cargo.toml").read_text()
    features = re.search(r"^\[features\]\s*$(.*?)(?=^\[|\Z)", text, re.M | re.S)
    return bool(features and re.search(r"^mods\s*=", features.group(1), re.M))


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--commit", required=True)
    p.add_argument("--version", required=True, help="HORIZON_VERSION, e.g. nightly-2026-11-03")
    p.add_argument("--nightly", default="", help='HORIZON_NIGHTLY, e.g. "2026-11-03 2944480"')
    p.add_argument("--asset", required=True)
    p.add_argument("--check", action="store_true", help="cargo check only")
    a = p.parse_args()
    if not re.fullmatch(r"[0-9a-f]{40}", a.commit):
        sys.exit("--commit must be a full commit SHA")

    tree = source(a.commit)
    bundles = WORK / "mods"
    shutil.rmtree(bundles, ignore_errors=True)
    shutil.copytree(ROOT / "mixins" / "arsenal", bundles / "arsenal")
    toml = bundles / "arsenal" / "horizon-mixin.toml"
    toml.write_text(re.sub(r'horizon_revision = "[0-9a-f]{40}"', f'horizon_revision = "{a.commit}"', toml.read_text()))
    stage = WORK / "stage"
    shutil.rmtree(stage, ignore_errors=True)
    run([mixins_tool(), "apply", "--workspace", tree, "--mods", bundles, "--out", stage])

    env = dict(os.environ, CARGO_TARGET_DIR=str(WORK / "target"), HORIZON_VERSION=a.version, HORIZON_NIGHTLY=a.nightly)
    command = ["cargo", "check" if a.check else "build", "--locked", "--release", "-p", "horizon-client", "--bin", "horizon-client"]
    if has_mods_feature(stage):
        command += ["--features", "horizon-client/mods"]
    run(command, cwd=stage, env=env)
    if a.check:
        return

    binary = WORK / "target" / "release" / f"horizon-client{EXE}"
    help_text = subprocess.run([str(binary), "--help"], capture_output=True, text=True, timeout=120).stdout
    if "--resources" not in help_text:
        sys.exit("The built game doesn't answer --help as Horizon's does.")
    DIST.mkdir(exist_ok=True)
    out = DIST / a.asset
    with zipfile.ZipFile(out, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as z:
        info = zipfile.ZipInfo(f"horizon-client{EXE}", (2026, 1, 1, 0, 0, 0))
        info.compress_type = zipfile.ZIP_DEFLATED
        info.external_attr = 0o100755 << 16
        z.writestr(info, binary.read_bytes())
    digest = hashlib.sha256(out.read_bytes()).hexdigest()
    log(f"{out.name} {out.stat().st_size} bytes sha256 {digest}")


if __name__ == "__main__":
    main()
