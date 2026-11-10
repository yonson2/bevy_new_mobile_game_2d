default: dev

# Run with the dev tools and Bevy linked dynamically.
dev:
    cargo run --features dev

# A headless shot of a scene, then exit: no window, a fixed time step, seed 1. The scene
# comes from GAME_SHOT_SCENE (default title; `docs/dev-harness.md` lists the scenes).
shot path="target/shots/title.png":
    mkdir -p "$(dirname '{{path}}')"
    GAME_SEED=1 GAME_SHOT='{{path}}' cargo run --features dev_tools

# The replay check: the scripted run, seeds 1, 2 and 3 at 30, 60 and 120 fps, gives the
# hashes in tools/replay-hashes.txt. CI runs it.
replay:
    tools/replay.sh

# The issue tracker: `just issue new "Title" -m M1`, `just issue start 12`,
# `just issue close 12`, `just issue check`.
issue *args:
    python3 tools/issues.py {{args}}
