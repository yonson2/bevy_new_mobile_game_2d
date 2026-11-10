"""What the asset tools share: the API keys, the model defaults and the sidecar.

A key comes from the environment; `.env` at the repo root gives the keys that the
environment does not set (`.env.example`). A tool never prints a key and never writes one
to a file.
"""

import json
import os
import time
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def key(name: str) -> str:
    if value := os.environ.get(name):
        return value
    env = ROOT / ".env"
    if env.exists():
        for line in env.read_text().splitlines():
            line = line.strip().removeprefix("export ")
            if line.startswith(name + "="):
                return line.split("=", 1)[1].strip().strip("'\"")
    raise SystemExit(f"{name} is not set: export it, or put it in .env (see .env.example)")


def models(provider: str) -> dict:
    """The provider's table in `tools/models.toml`."""
    return tomllib.loads((ROOT / "tools/models.toml").read_text())[provider]


def sidecar(path: Path, meta: dict) -> None:
    """Writes the record of a generation beside its file: enough to make it again."""
    meta["created"] = time.strftime("%Y-%m-%dT%H:%M:%S%z")
    path.write_text(json.dumps(meta, indent=2) + "\n")
