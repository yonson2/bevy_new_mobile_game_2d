#!/usr/bin/env python3
"""Call one PixelLab REST v2 endpoint: pixel art stills, animations, tiles, edits.

Content time only: the game never calls PixelLab. ENDPOINT is a role of
`tools/models.toml` (`animation`, `image`, `pixelize`, `palette`..) or an endpoint's path.
The endpoints and their request shapes: https://api.pixellab.ai/v2/llms.txt and
https://api.pixellab.ai/v2/openapi.json.

POST https://api.pixellab.ai/v2/ENDPOINT with the JSON from --args. --image KEY=FILE sets a
base64 image argument; KEY[]=FILE appends one to a list; a dotted KEY (`style_image.image`,
`reference_images.0.image`) sets one inside an object or a list of --args. An answer with a
`background_job_id` is polled until the job completes. Each image in the answer is saved as
NAME.png (NAME-<n>.png for several, in the answer's order); --drop-first skips the first
one (an animation's answer starts with its input frame). The sidecar NAME.json keeps the
endpoint, the request (images by file name), the rest of the answer, the usage and the
date.

`tools/pixellab.py balance` prints the generations and the credits that remain.

Key: PIXELLAB_TOKEN (the environment, or `.env`).

Usage: tools/pixellab.py ENDPOINT --out DIR/NAME [--args JSON_OR_FILE]
       [--image KEY=FILE] [--image KEY[]=FILE ...] [--drop-first]
"""

import argparse
import base64
import json
import time
import urllib.error
import urllib.request
from pathlib import Path

from common import key, models, sidecar

API = "https://api.pixellab.ai/v2"


def call(method: str, path: str, body: dict | None = None) -> dict:
    headers = {"Authorization": f"Bearer {key('PIXELLAB_TOKEN')}",
               "Content-Type": "application/json"}
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(API + path, data=data, headers=headers, method=method)
    try:
        with urllib.request.urlopen(req, timeout=600) as res:
            return json.loads(res.read())
    except urllib.error.HTTPError as err:
        raise SystemExit(f"{method} {path}: HTTP {err.code}: {err.read()[:500]!r}")


def take_images(node, found: list):
    """Replaces each base64 image in the answer by its index in `found`."""
    if isinstance(node, dict):
        if node.get("type") == "base64" and "base64" in node:
            found.append(node)
            return {"saved_image": len(found) - 1}
        return {k: take_images(v, found) for k, v in node.items()}
    if isinstance(node, list):
        return [take_images(v, found) for v in node]
    return node


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("endpoint")
    ap.add_argument("--out", type=Path)
    ap.add_argument("--args", default="{}")
    ap.add_argument("--image", action="append", default=[])
    ap.add_argument("--drop-first", action="store_true")
    args = ap.parse_args()
    if args.endpoint == "balance":
        print(json.dumps(call("GET", "/balance"), indent=2))
        return
    if not args.out:
        ap.error("--out is required")
    endpoint = models("pixellab").get(args.endpoint, args.endpoint).lstrip("/")

    # A JSON string, or the path of a file that holds it.
    text = args.args if args.args.lstrip().startswith("{") else Path(args.args).read_text()
    body = json.loads(text)
    request = json.loads(text)
    for spec in args.image:
        name, file = spec.split("=", 1)
        image = {"type": "base64", "base64": base64.b64encode(Path(file).read_bytes()).decode(),
                 "format": "png"}
        *parents, name = name.split(".")
        node, logged = body, request
        for parent in parents:
            if isinstance(node, list):
                node, logged = node[int(parent)], logged[int(parent)]
            else:
                node, logged = node.setdefault(parent, {}), logged.setdefault(parent, {})
        if name.endswith("[]"):
            node.setdefault(name[:-2], []).append(image)
            logged.setdefault(name[:-2], []).append(file)
        else:
            node[name] = image
            logged[name] = file

    started = time.time()
    answer = call("POST", "/" + endpoint, body)
    job = answer.get("background_job_id")
    while job:
        status = call("GET", f"/background-jobs/{job}")
        if status["status"] == "completed":
            answer = {"submitted": answer, "job": status}
            break
        if status["status"] == "failed":
            raise SystemExit(f"job {job} failed: {json.dumps(status)[:500]}")
        time.sleep(5)

    found = []
    result = take_images(answer, found)
    saved = found[1:] if args.drop_first else found
    args.out.parent.mkdir(parents=True, exist_ok=True)
    files = []
    for i, image in enumerate(saved, 1):
        stem = args.out.name if len(saved) == 1 else f"{args.out.name}-{i}"
        out = args.out.with_name(f"{stem}.{image.get('format', 'png')}")
        out.write_bytes(base64.b64decode(image["base64"]))
        files.append(out.name)
    usage = answer.get("usage") or (answer.get("job") or {}).get("usage")
    sidecar(args.out.with_name(args.out.name + ".json"),
            {"provider": "pixellab", "endpoint": f"{API}/{endpoint}", "request": request,
             "answer": result, "usage": usage, "files": files,
             "elapsed_s": round(time.time() - started, 1)})
    print(f"{args.out}: {', '.join(files) or 'no images'} (usage {usage})")


if __name__ == "__main__":
    main()
