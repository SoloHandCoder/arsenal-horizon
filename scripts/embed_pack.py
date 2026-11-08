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


def main() -> None:
    source = RUNTIME.read_text(encoding="utf-8")
    start = source.index("// BEGIN PACK\n") + len("// BEGIN PACK\n")
    end = source.index("// END PACK")
    RUNTIME.write_text(source[:start] + pack_source() + "\n" + source[end:], encoding="utf-8", newline="\n")
    print("embedded", len(list(PACK.rglob("*.*"))), "files")


if __name__ == "__main__":
    main()
