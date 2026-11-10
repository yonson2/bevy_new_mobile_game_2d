#!/usr/bin/env python3
"""Generate sound effects, music and voices with ElevenLabs.

Content time only: the game never calls ElevenLabs. Each call writes its audio and a
sidecar JSON beside it (NAME.json): the request, the output format the plan accepted, the
billing headers and the timing. The models come from `tools/models.toml`.
`tools/encode.py` makes the game's `.ogg` from a pick.

Key: ELEVENLABS_TOKEN (the environment, or `.env`).

Usage:
  tools/audio.py sfx PROMPT --out DIR/NAME [--seconds S] [--influence P] [--loop] [--takes N]
  tools/audio.py music PROMPT --out DIR/NAME --seconds S [--mode loop|track|ambience]
                 [--vocals] [--takes N]
  tools/audio.py design DESCRIPTION --out DIR/NAME [--text TEXT]    three voice previews
  tools/audio.py voice GENERATED_VOICE_ID --name NAME --description DESCRIPTION
                 [--out DIR/NAME]                                   keeps a preview's voice
  tools/audio.py say TEXT --voice VOICE_ID --out DIR/NAME [--stability S] [--takes N]
  tools/audio.py credits
"""

import argparse
import base64
import json
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

from common import key, models, sidecar

API = "https://api.elevenlabs.io"
# Best first; the plan or the endpoint refuses a format, and the call moves down.
FORMATS = ["wav_48000", "wav_44100", "mp3_44100_192", "mp3_44100_128"]
KEPT_HEADERS = ("character-cost", "request-id", "history-item-id", "content-type")


def call(path, body=None, method="POST"):
    """One API call, retried with backoff on 429 and 5xx."""
    data = json.dumps(body).encode() if body is not None else None
    headers = {"xi-api-key": key("ELEVENLABS_TOKEN"), "Content-Type": "application/json"}
    for attempt in range(6):
        req = urllib.request.Request(f"{API}{path}", data=data, headers=headers, method=method)
        try:
            with urllib.request.urlopen(req, timeout=600) as res:
                return res.status, {k.lower(): v for k, v in res.headers.items()}, res.read()
        except urllib.error.HTTPError as err:
            status, raw = err.code, err.read()
            if status != 429 and status < 500:
                return status, {k.lower(): v for k, v in err.headers.items()}, raw
            wait = float(err.headers.get("retry-after") or 2**attempt)
            print(f"  HTTP {status}, retrying in {wait:.0f} s", file=sys.stderr)
            time.sleep(wait)
    raise SystemExit(f"{path}: still failing after retries")


def call_json(path, body=None, method="POST"):
    status, _, raw = call(path, body, method)
    if status != 200:
        raise SystemExit(f"{path}: HTTP {status} {raw.decode(errors='replace')[:500]}")
    return json.loads(raw)


def takes(args, path, body, formats=FORMATS):
    """Calls `path` once a take, each in the best output format the plan accepts."""
    for take in range(1, args.takes + 1):
        stem = args.out if args.takes == 1 else args.out.with_name(f"{args.out.name}-{take}")
        started = time.time()
        for fmt in formats:
            status, headers, raw = call(f"{path}?output_format={fmt}", body)
            if status == 200:
                break
            print(f"  {fmt}: HTTP {status} {raw.decode(errors='replace')[:200]}", file=sys.stderr)
            if status not in (400, 401, 403, 422):
                raise SystemExit(f"{path}: HTTP {status}")
        else:
            raise SystemExit(f"{path}: no output format accepted")
        master = stem.with_name(f"{stem.name}.{fmt[:3]}")
        master.write_bytes(raw)
        sidecar(stem.with_name(stem.name + ".json"),
                {"provider": "elevenlabs", "endpoint": path, "request": body,
                 "output_format": fmt, "file": master.name,
                 "headers": {k: headers[k] for k in KEPT_HEADERS if k in headers},
                 "elapsed_s": round(time.time() - started, 1)})
        print(f"{master}  ({headers.get('character-cost', '?')} credits)")


def sfx(args):
    body = {"text": args.prompt, "model_id": models("elevenlabs")["sfx"],
            "prompt_influence": args.influence, "loop": args.loop}
    if args.seconds is not None:
        body["duration_seconds"] = args.seconds
    takes(args, "/v1/sound-generation", body)


def music(args):
    body = {"prompt": args.prompt, "model_id": models("elevenlabs")["music"],
            "music_length_ms": round(args.seconds * 1000), "force_instrumental": not args.vocals,
            "generation_mode": args.mode}
    # The music endpoint has no WAV.
    takes(args, "/v1/music", body, FORMATS[2:])


def design(args):
    body = {"voice_description": args.description, "model_id": models("elevenlabs")["voice_design"]}
    if args.text:
        body["text"] = args.text
    else:
        body["auto_generate_text"] = True
    answer = call_json("/v1/text-to-voice/design?output_format=mp3_44100_128", body)
    previews = []
    for i, preview in enumerate(answer["previews"], 1):
        out = args.out.with_name(f"{args.out.name}-{i}.mp3")
        out.write_bytes(base64.b64decode(preview["audio_base_64"]))
        previews.append({"file": out.name, "generated_voice_id": preview["generated_voice_id"]})
        print(f"{out}  generated_voice_id {preview['generated_voice_id']}")
    sidecar(args.out.with_name(args.out.name + ".json"),
            {"provider": "elevenlabs", "endpoint": "/v1/text-to-voice/design", "request": body,
             "text": answer.get("text"), "previews": previews})


def voice(args):
    body = {"voice_name": args.name, "voice_description": args.description,
            "generated_voice_id": args.generated_voice_id}
    answer = call_json("/v1/text-to-voice", body)
    print(f"voice_id {answer['voice_id']}")
    if args.out:
        sidecar(args.out.with_name(args.out.name + ".json"),
                {"provider": "elevenlabs", "endpoint": "/v1/text-to-voice", "request": body,
                 "voice_id": answer["voice_id"]})


def say(args):
    body = {"text": args.text, "model_id": models("elevenlabs")["speech"]}
    if args.stability is not None:
        body["voice_settings"] = {"stability": args.stability}
    takes(args, f"/v1/text-to-speech/{args.voice}", body)


def credits(args):
    plan = call_json("/v1/user/subscription", method="GET")
    print(f"{plan['tier']}: {plan['character_count']} of {plan['character_limit']} credits used")


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = ap.add_subparsers(dest="command", required=True)

    p = sub.add_parser("sfx")
    p.add_argument("prompt")
    p.add_argument("--seconds", type=float)
    p.add_argument("--influence", type=float, default=0.3)
    p.add_argument("--loop", action="store_true")
    p.set_defaults(run=sfx)

    p = sub.add_parser("music")
    p.add_argument("prompt")
    p.add_argument("--seconds", type=float, required=True)
    p.add_argument("--mode", default="loop", choices=["loop", "track", "ambience"])
    p.add_argument("--vocals", action="store_true")
    p.set_defaults(run=music)

    p = sub.add_parser("design")
    p.add_argument("description")
    p.add_argument("--text")
    p.set_defaults(run=design)

    p = sub.add_parser("voice")
    p.add_argument("generated_voice_id")
    p.add_argument("--name", required=True)
    p.add_argument("--description", required=True)
    p.set_defaults(run=voice)

    p = sub.add_parser("say")
    p.add_argument("text")
    p.add_argument("--voice", required=True)
    p.add_argument("--stability", type=float)
    p.set_defaults(run=say)

    sub.add_parser("credits").set_defaults(run=credits)
    for name, p in sub.choices.items():
        if name != "credits":
            p.add_argument("--out", type=Path, required=name != "voice")
        if name in ("sfx", "music", "say"):
            p.add_argument("--takes", type=int, default=1)
    args = ap.parse_args()
    if getattr(args, "out", None):
        args.out.parent.mkdir(parents=True, exist_ok=True)
    args.run(args)


if __name__ == "__main__":
    main()
