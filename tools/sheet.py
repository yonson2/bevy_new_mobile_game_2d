#!/usr/bin/env python3
"""Pack animation frames into a sheet: one row a clip, its frames left to right.

Usage: tools/sheet.py OUT.png ROW [ROW..]

Each ROW is a glob of PNG frames, in the order of the number that ends each name
(`player-walk-3.png` is frame 3). Every frame has the first one's size. The sheet is as
wide as the longest row; a shorter row leaves its end clear. It prints each row's frames.
Needs Pillow.
"""

import glob
import re
import sys

from PIL import Image


def frames(pattern: str) -> list[str]:
    def number(path: str) -> int:
        match = re.search(r"(\d+)\.png$", path)
        return int(match.group(1)) if match else 0

    paths = sorted(glob.glob(pattern), key=number)
    if not paths:
        sys.exit(f"sheet: no frames match {pattern}")
    return paths


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__)
        return 2
    rows = [frames(pattern) for pattern in sys.argv[2:]]
    w, h = Image.open(rows[0][0]).size
    sheet = Image.new("RGBA", (w * max(map(len, rows)), h * len(rows)))
    for y, row in enumerate(rows):
        for x, path in enumerate(row):
            frame = Image.open(path).convert("RGBA")
            if frame.size != (w, h):
                sys.exit(f"sheet: {path} is {frame.size[0]} x {frame.size[1]}, not {w} x {h}")
            sheet.paste(frame, (x * w, y * h))
        print(f"row {y}: {len(row)} frames")
    sheet.save(sys.argv[1])
    return 0


if __name__ == "__main__":
    sys.exit(main())
