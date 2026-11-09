"""Copies shaderpack/arsenal into the PACK list in mixins/arsenal/runtime.rs.

The runtime is built inside Horizon's own crate, so it cannot include_str! files
from this repository; the pack goes in as text instead. Run it after changing
the pack (a test fails while the two differ).
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PACK = ROOT / "shaderpack" / "arsenal"
RUNTIME = ROOT / "mixins" / "arsenal" / "runtime.rs"


def pack_source() -> str:
    lines = ["pub const PACK: &[(&str, &str)] = &["]
    for path in sorted(p for p in PACK.rglob("*") if p.is_file()):
        name = path.relative_to(PACK).as_posix()
        text = path.read_text(encoding="utf-8").replace("\r\n", "\n")
        if '"####' in text:
            raise SystemExit(f"{name} holds the raw string end mark")
        lines.append(f'    ("{name}", r####"{text}"####),')
    lines.append("];")
    return "\n".join(lines)


def files_source(name: str, folder: Path) -> str:
    lines = [f"pub const {name}: &[(&str, &str)] = &["]
    for path in sorted(p for p in folder.rglob("*") if p.is_file()):
        rel = path.relative_to(folder).as_posix()
        text = path.read_text(encoding="utf-8").replace("\r\n", "\n")
        if '"####' in text:
            raise SystemExit(f"{rel} holds the raw string end mark")
        lines.append(f'    ("{rel}", r####"{text}"####),')
    lines.append("];")
    return "\n".join(lines)


def put(source: str, marker: str, text: str) -> str:
    start = source.index(f"// BEGIN {marker}\n") + len(f"// BEGIN {marker}\n")
    end = source.index(f"// END {marker}")
    return source[:start] + text + "\n" + source[end:]


def main() -> None:
    source = RUNTIME.read_text(encoding="utf-8")
    source = put(source, "PACK", pack_source())
    # Low Fire on the ground's pack (lowfire/: lower fire block models, as the Java mod's pack).
    source = put(source, "LOWFIRE", files_source("LOWFIRE", ROOT / "lowfire"))
    RUNTIME.write_text(source, encoding="utf-8", newline="\n")
    print("embedded", len(list(PACK.rglob("*.*"))), "files")


if __name__ == "__main__":
    main()
