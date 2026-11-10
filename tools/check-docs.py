#!/usr/bin/env python3
"""Check the live docs: each is under its size limit, and each repository path that a
doc names in backticks, outside a fenced block, exists.

A doc over its limit is a sign to split it: move a section into its own file under
docs/ and leave a pointer.
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LIMITS = {"AGENTS.md": 120, "ARCHITECTURE.md": 250, "README.md": 120}
DOCS_LIMIT = 250
PATH = re.compile(r"`((?:src|tools|docs|assets|mobile|issues|tracker|art-src|audio-src|meta)/[^`\s*<>{}]+|[A-Za-z_.-]+\.(?:md|toml))`")
FENCE = re.compile(r"^```.*?^```", re.M | re.S)
# Folders that git ignores.
SKIP = ("art-src/raw", "audio-src/takes")


def main() -> int:
    docs = [ROOT / name for name in LIMITS] + sorted((ROOT / "docs").rglob("*.md"))
    errors = []
    for doc in docs:
        rel = doc.relative_to(ROOT).as_posix()
        if not doc.exists():
            errors.append(f"{rel}: missing")
            continue
        text = doc.read_text()
        lines = text.count("\n")
        limit = LIMITS.get(rel, DOCS_LIMIT)
        if lines > limit:
            errors.append(f"{rel}: {lines} lines, the limit is {limit}: split it")
        # A fenced block is an example: its paths are not checked.
        for path in sorted(set(PATH.findall(FENCE.sub("", text)))):
            if path.startswith(SKIP) or (ROOT / path).exists():
                continue
            errors.append(f"{rel}: `{path}` does not exist")
    for error in errors:
        print(error)
    print(f"docs: {len(docs)} files, {len(errors)} problems")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
