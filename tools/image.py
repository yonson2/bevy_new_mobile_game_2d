#!/usr/bin/env python3
"""Generate images with OpenRouter: concept art, style references, key art, store art.

Content time only: the game never calls OpenRouter. Each call writes NAME.png (NAME-<n>.png
for several images) and one sidecar NAME.json: the request, the cost that OpenRouter
reports, the files and the date.

A preset of `tools/models.toml` gives the model and its settings; the options replace a
preset's fields. The models and their parameters:
GET https://openrouter.ai/api/v1/images/models.

Key: OPENROUTER_API_KEY (the environment, or `.env`).

Usage: tools/image.py PROMPT --out DIR/NAME [--preset key_art] [--model SLUG]
       [--quality max] [--aspect 9:16] [--resolution 1K] [--transparent] [--format png]
       [--seed N] [--n N] [--ref FILE_OR_URL ...]
"""

import argparse
import base64
import json
import mimetypes
import time
import urllib.error
import urllib.request
from pathlib import Path

from common import key, models, sidecar

API = "https://openrouter.ai/api/v1/images"
EXT = {"image/png": ".png", "image/jpeg": ".jpg", "image/webp": ".webp", "image/gif": ".gif"}


def reference(ref: str) -> dict:
    url = ref
    if not ref.startswith(("http://", "https://", "data:")):
        mime = mimetypes.guess_type(ref)[0] or "image/png"
        url = f"data:{mime};base64,{base64.b64encode(Path(ref).read_bytes()).decode()}"
    return {"type": "image_url", "image_url": {"url": url}}


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("prompt")
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--preset", default="key_art")
    ap.add_argument("--model")
    ap.add_argument("--quality")
    ap.add_argument("--aspect")
    ap.add_argument("--resolution")
    ap.add_argument("--transparent", action="store_true")
    ap.add_argument("--format")
    ap.add_argument("--seed", type=int)
    ap.add_argument("--n", type=int)
    ap.add_argument("--ref", action="append", default=[])
    args = ap.parse_args()

    body = {**models("openrouter")[args.preset], "prompt": args.prompt}
    options = {"model": args.model, "quality": args.quality, "aspect_ratio": args.aspect,
               "resolution": args.resolution, "output_format": args.format, "seed": args.seed,
               "n": args.n, "background": "transparent" if args.transparent else None}
    body.update({k: v for k, v in options.items() if v is not None})
    request = dict(body)
    if args.ref:
        body["input_references"] = [reference(r) for r in args.ref]
        # The sidecar keeps the references by name, not as data URLs.
        request["input_references"] = args.ref

    headers = {"Authorization": f"Bearer {key('OPENROUTER_API_KEY')}",
               "Content-Type": "application/json"}
    req = urllib.request.Request(API, data=json.dumps(body).encode(), headers=headers)
    started = time.time()
    try:
        with urllib.request.urlopen(req, timeout=900) as res:
            result = json.loads(res.read())
    except urllib.error.HTTPError as err:
        raise SystemExit(f"HTTP {err.code}: {err.read()[:500]!r}")

    args.out.parent.mkdir(parents=True, exist_ok=True)
    files = []
    for i, image in enumerate(result["data"], 1):
        stem = args.out.name if len(result["data"]) == 1 else f"{args.out.name}-{i}"
        out = args.out.with_name(stem + EXT.get(image.get("media_type"), ".png"))
        out.write_bytes(base64.b64decode(image["b64_json"]))
        files.append(out.name)
    usage = result.get("usage") or {}
    sidecar(args.out.with_name(args.out.name + ".json"),
            {"provider": "openrouter", "endpoint": API, "request": request, "usage": usage,
             "cost_usd": usage.get("cost"), "files": files,
             "elapsed_s": round(time.time() - started, 1)})
    print(f"{args.out}: {', '.join(files)} (${usage.get('cost')})")


if __name__ == "__main__":
    main()
