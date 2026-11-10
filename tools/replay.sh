#!/usr/bin/env bash
# Replay check: seeds 1, 2 and 3 play the scripted run (GAME_SHOT_SCENE=replay) at 30, 60
# and 120 frames per second, without the renderer. Each run's GAME_STATE hash must be the
# same at every frame rate and equal its line in tools/replay-hashes.txt. A change that
# moves the hashes on purpose updates that file in the same commit. PROFILE picks the
# cargo profile (default dev).
set -euo pipefail
cd "$(dirname "$0")/.."
profile=${PROFILE:-dev}
if [ "$profile" = dev ]; then dir=debug; else dir=$profile; fi
crate=$(grep -m1 '^name = ' Cargo.toml | cut -d'"' -f2)
cargo build --locked --profile "$profile" --features dev_tools
out=target/replay
rm -rf "$out" && mkdir -p "$out"
for seed in 1 2 3; do
    for fps in 30 60 120; do
        GAME_SEED=$seed GAME_SHOT_FPS=$fps GAME_SHOT_SCENE=replay GAME_SHOT="$out/unused.png" \
            BEVY_ASSET_ROOT="$PWD" "target/$dir/$crate" 2>&1 \
            | grep -o 'GAME_STATE.*' > "$out/$fps.txt" || true
    done
    test -s "$out/30.txt"
    diff "$out/30.txt" "$out/60.txt"
    diff "$out/30.txt" "$out/120.txt"
    echo "seed $seed: $(cat "$out/30.txt")"
    echo "$seed $(cut -d' ' -f4 "$out/30.txt")" >> "$out/hashes.txt"
done
grep -v '^#' tools/replay-hashes.txt | diff - "$out/hashes.txt"
echo "replay: the same state at 30, 60 and 120 fps, as in tools/replay-hashes.txt"
