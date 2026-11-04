#!/usr/bin/env python3
"""Copies the Java mod's module list (id, name, category, description) into mixins/arsenal/runtime.rs, between its
`// BEGIN JAVA MODULES` and `// END JAVA MODULES` lines, so the Right Shift menu on Horizon lists every module the Java
mod has. Modules Horizon can't run show greyed out.

    python scripts/java_modules.py <path to the Java mod's src/main/java/dev/arsenal>
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RUNTIME = ROOT / "mixins" / "arsenal" / "runtime.rs"
CATEGORIES = ["HUD", "COMBAT", "ENEMIES", "CRYSTAL", "MACE", "WORLD", "CAMERA", "SOCIAL", "UTILITY"]


def main():
    src = Path(sys.argv[1])
    found = {}
    for path in sorted((src / "modules").rglob("*.java")):
        text = path.read_text(encoding="utf8")
        for m in re.finditer(r'super\("([a-z0-9_]+)",\s*"([^"]+)",\s*Category\.([A-Z_]+)', text):
            found[m.group(1)] = (m.group(2), m.group(3))
    info = (src / "modules" / "ModuleInfo.java").read_text(encoding="utf8")
    block = info[info.index("DESCRIPTIONS = Map.ofEntries("):info.index("TAGS = Map.ofEntries(")]
    descriptions = {m.group(1): json.loads('"' + m.group(2) + '"')
                    for m in re.finditer(r'Map\.entry\("([a-z0-9_]+)",\s*"((?:[^"\\]|\\.)*)"\)', block)}
    rows = sorted(found.items(), key=lambda kv: (CATEGORIES.index(kv[1][1]) if kv[1][1] in CATEGORIES else 99, kv[1][0].lower()))
    lines = ["pub const JAVA_MODULES: &[JavaModule] = &["]
    for mid, (name, cat) in rows:
        desc = descriptions.get(mid, "")
        lines.append(f"    JavaModule {{ id: {json.dumps(mid)}, name: {json.dumps(name, ensure_ascii=False)}, "
                     f"category: {CATEGORIES.index(cat) if cat in CATEGORIES else 0}, description: {json.dumps(desc, ensure_ascii=False)} }},")
    lines.append("];")
    text = RUNTIME.read_text(encoding="utf8")
    start, end = "// BEGIN JAVA MODULES\n", "// END JAVA MODULES"
    a, b = text.index(start) + len(start), text.index(end)
    RUNTIME.write_text(text[:a] + "\n".join(lines) + "\n" + text[b:], encoding="utf8")
    print(f"{len(rows)} modules")


if __name__ == "__main__":
    main()
