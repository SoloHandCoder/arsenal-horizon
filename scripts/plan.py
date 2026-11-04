#!/usr/bin/env python3
"""Which Horizon build to put Arsenal into: the newest dated Nightly (`nightly-YYYY-MM-DD`, kept on GitHub for a week,
so the Arsenal Launcher can still download that exact official build), and whether this repository already has a
release for it with this Arsenal version.

Writes GitHub Actions outputs: build (2026-11-03-2944480), commit, official_tag, nightly ("2026-11-03 2944480"),
arsenal (the bundle's version), tag (this repository's release tag) and skip (true when that release exists).
"""
import json
import os
import re
import sys
import tomllib
import urllib.error
import urllib.request
from pathlib import Path

HORIZON = "horizon-rust/horizon"
ROOT = Path(__file__).resolve().parents[1]


def get(url):
    request = urllib.request.Request(url, headers={"User-Agent": "arsenal-horizon", "Accept": "application/vnd.github+json"})
    token = os.environ.get("GITHUB_TOKEN")
    if token:
        request.add_header("Authorization", f"Bearer {token}")
    with urllib.request.urlopen(request, timeout=60) as response:
        return json.load(response)


def build_id(name, published, commit):
    """As the Arsenal Launcher names a build: "Nightly 2026-11-03 · 2944480" -> "2026-11-03-2944480"."""
    date = next((w for w in name.split() if re.fullmatch(r"\d{4}-\d{2}-\d{2}", w)), published[:10])
    short = name.rsplit("·", 1)[-1].strip() if "·" in name else ""
    if not re.fullmatch(r"[0-9a-f]{7,40}", short):
        short = commit[:7]
    return f"{date}-{short}"


def main():
    releases = get(f"https://api.github.com/repos/{HORIZON}/releases?per_page=30")
    dated = [r for r in releases if re.fullmatch(r"nightly-\d{4}-\d{2}-\d{2}", r["tag_name"]) and not r.get("draft")]
    if not dated:
        sys.exit("Horizon has no dated Nightly release.")
    newest = max(dated, key=lambda r: r["published_at"])
    commit = newest["target_commitish"]
    if not re.fullmatch(r"[0-9a-f]{40}", commit):
        commit = get(f"https://api.github.com/repos/{HORIZON}/commits/{newest['tag_name']}")["sha"]
    build = build_id(newest["name"] or "", newest["published_at"], commit)
    arsenal = tomllib.loads((ROOT / "mixins/arsenal/horizon-mixin.toml").read_text())["mod"]["version"]
    tag = f"{build}-arsenal-{arsenal}"
    repo = os.environ.get("GITHUB_REPOSITORY", "")
    skip = False
    if repo and os.environ.get("FORCE") != "true":
        try:
            get(f"https://api.github.com/repos/{repo}/releases/tags/{tag}")
            skip = True
        except urllib.error.HTTPError as e:
            if e.code != 404:
                raise
    out = {
        "build": build,
        "commit": commit,
        "official_tag": newest["tag_name"],
        "nightly": build.rsplit("-", 1)[0] + " " + build.rsplit("-", 1)[1],
        "arsenal": arsenal,
        "tag": tag,
        "skip": "true" if skip else "false",
    }
    print(json.dumps(out, indent=2))
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as f:
            for k, v in out.items():
                f.write(f"{k}={v}\n")


if __name__ == "__main__":
    main()
