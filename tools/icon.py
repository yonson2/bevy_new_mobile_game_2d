#!/usr/bin/env python3
"""Write the app icon into the phone projects from one source picture.

The source, `art-src/icon/icon.png`: the icon's subject on a clear ground, square, 1024 px
or more. The background is one flat colour.

- iOS: the 1024 px icon of the asset catalog, the subject on the background, a full square
  with no alpha (iOS rounds the corners itself).
- Android: the adaptive icon's foreground and monochrome layers for each density, and the
  background colour. A launcher's mask always keeps the middle 66 dp circle of the 108 dp
  layer: the subject goes into that circle's square.

Usage: tools/icon.py [--background '#1b1f2a'] [--share 0.8] [--pixel]

--pixel: the source is pixel art; it scales with hard pixels.
Needs Pillow.
"""
import argparse
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "art-src/icon/icon.png"
IOS = ROOT / "mobile/ios/ios-src/Assets.xcassets/AppIcon.appiconset/icon-1024.png"
ANDROID_RES = ROOT / "mobile/android/app/src/main/res"
# Android densities and the pixel size of a 108 dp layer there.
DENSITIES = {"mdpi": 108, "hdpi": 162, "xhdpi": 216, "xxhdpi": 324, "xxxhdpi": 432}


def layer(subject, px, share, sampling):
    """The subject in the middle of a clear `px` square, `share` of its side."""
    inner = round(px * share)
    out = Image.new("RGBA", (px, px), (0, 0, 0, 0))
    out.alpha_composite(subject.resize((inner, inner), sampling), ((px - inner) // 2,) * 2)
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--background", default="#1b1f2a", help="the colour behind the subject")
    parser.add_argument("--share", type=float, default=0.8, help="the subject's share of the iOS icon's side")
    parser.add_argument("--pixel", action="store_true", help="scale with hard pixels")
    args = parser.parse_args()
    sampling = Image.NEAREST if args.pixel else Image.LANCZOS
    ground = tuple(bytes.fromhex(args.background.lstrip("#")))
    subject = Image.open(SOURCE).convert("RGBA")
    if subject.width != subject.height:
        raise SystemExit(f"{SOURCE.relative_to(ROOT)} is not square: {subject.size}")

    ios = Image.new("RGBA", (1024, 1024), ground + (255,))
    ios.alpha_composite(layer(subject, 1024, args.share, sampling))
    IOS.parent.mkdir(parents=True, exist_ok=True)
    ios.convert("RGB").save(IOS)

    # The square inside the 66 dp circle of the 108 dp layer.
    safe = 66 / 108 / 2**0.5
    white = Image.new("RGBA", subject.size, (255, 255, 255, 255))
    white.putalpha(subject.getchannel("A"))
    for density, px in DENSITIES.items():
        folder = ANDROID_RES / f"mipmap-{density}"
        folder.mkdir(parents=True, exist_ok=True)
        layer(subject, px, safe, sampling).save(folder / "ic_launcher_foreground.png")
        layer(white, px, safe, sampling).save(folder / "ic_launcher_monochrome.png")
    values = ANDROID_RES / "values/ic_launcher_background.xml"
    values.parent.mkdir(parents=True, exist_ok=True)
    values.write_text(
        '<?xml version="1.0" encoding="utf-8"?>\n'
        "<!-- Written by tools/icon.py. -->\n"
        "<resources>\n"
        f'    <color name="ic_launcher_background">{args.background}</color>\n'
        "</resources>\n"
    )
    print(f"icon written from {SOURCE.relative_to(ROOT)} on {args.background}")


if __name__ == "__main__":
    main()
