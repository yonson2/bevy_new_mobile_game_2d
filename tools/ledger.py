#!/usr/bin/env python3
"""The asset ledger: every file under assets/ has a row with its hash, its source and its
licence terms. An asset with an unknown licence does not ship.

Images and fonts are in art-src/ledger.md; sounds are in audio-src/ledger.md. A row's
verdict is `placeholder` (in the game, the owner has not judged it) or `approved` (the
owner's verdict, with where the owner gave it).

  ledger.py add FILE --source TEXT --licence TEXT [--approved REVIEW]
        adds the file's row, or replaces it; the hash is the file's hash now
  ledger.py check [--final]
        every file under assets/ has a row with its hash of now; --final also refuses
        placeholders (the release milestone)

SOURCE says how the file was made and names its sidecar, for example
"PixelLab animate-pixminimax, art-src/sprites/player-walk.json".
"""

import argparse
import hashlib
import re
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
AUDIO = {".ogg", ".wav", ".mp3"}
HEAD = "| File | Hash | Verdict | Source | Licence | Review | Date |\n|---|---|---|---|---|---|---|\n"
ROW = re.compile(r"^\| `([^`]+)` \| `([0-9a-f]{16})` \| (approved|placeholder) \|")


def ledger(path: Path) -> Path:
    return ROOT / ("audio-src" if path.suffix in AUDIO else "art-src") / "ledger.md"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()[:16]


def add(args) -> int:
    path = args.file.resolve()
    rel = path.relative_to(ROOT).as_posix()
    if not rel.startswith("assets/"):
        sys.exit(f"{rel} is not under assets/")
    cell = lambda text: text.replace("|", "\\|").replace("\n", " ")
    row = (f"| `{rel}` | `{digest(path)}` | {'approved' if args.approved else 'placeholder'} | "
           f"{cell(args.source)} | {cell(args.licence)} | {cell(args.approved or '-')} | "
           f"{time.strftime('%Y-%m-%d')} |")
    file = ledger(path)
    text = file.read_text()
    if HEAD not in text:
        sys.exit(f"{file.relative_to(ROOT)} has no table")
    lines = [line for line in text.splitlines() if not line.startswith(f"| `{rel}` |")]
    file.write_text("\n".join(lines + [row]) + "\n")
    print(row)
    return 0


def check(args) -> int:
    rows = {}
    for kind in ("art-src", "audio-src"):
        for line in (ROOT / kind / "ledger.md").read_text().splitlines():
            if m := ROW.match(line):
                rows[m[1]] = (m[2], m[3])
    errors = []
    files = sorted(p for p in (ROOT / "assets").rglob("*") if p.is_file() and p.name != ".gitkeep")
    for path in files:
        rel = path.relative_to(ROOT).as_posix()
        row = rows.pop(rel, None)
        if row is None:
            errors.append(f"{rel}: no row in {ledger(path).relative_to(ROOT)}")
        elif row[0] != digest(path):
            errors.append(f"{rel}: changed since its row ({row[0]}, now {digest(path)})")
        elif args.final and row[1] != "approved":
            errors.append(f"{rel}: still a placeholder")
    errors += [f"{rel}: a row, but no file" for rel in rows]
    for error in errors:
        print(error)
    print(f"ledger: {len(files)} files, {len(errors)} problems")
    return 1 if errors else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawTextHelpFormatter)
    sub = parser.add_subparsers(dest="command", required=True)
    a = sub.add_parser("add")
    a.add_argument("file", type=Path)
    a.add_argument("--source", required=True)
    a.add_argument("--licence", required=True)
    a.add_argument("--approved", metavar="REVIEW")
    a.set_defaults(run=add)
    c = sub.add_parser("check")
    c.add_argument("--final", action="store_true")
    c.set_defaults(run=check)
    args = parser.parse_args()
    return args.run(args)


if __name__ == "__main__":
    sys.exit(main())
