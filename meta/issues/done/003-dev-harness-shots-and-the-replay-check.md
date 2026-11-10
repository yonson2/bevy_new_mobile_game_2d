# [003] Dev harness: shots and the replay check

**Status:** done
**Milestone:** M0
**Depends on:** [002]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
An agent cannot look at a window or play a run. It needs a picture of each screen
without a window, and a check that a seed plays the same run at any frame rate.

## Approach
- `src/dev_tools.rs` (feature `dev_tools`): `GAME_SHOT` runs a scene with no window and
  a fixed time step, saves one frame and exits; `GAME_SCENE` starts a windowed run in a
  scene; `GAME_SWIPE` plays touches through the input layer; `GAME_SEED`, `GAME_STYLE`.
  Scenes: title, gameplay, pause, game-over, settings, canvas, safe-area, haptics,
  stress, replay.
- `tools/replay.sh` (`just replay`): seeds 1, 2 and 3 at 30, 60 and 120 fps without the
  renderer; each state hash must be the same at the three rates and equal its line in
  `tools/replay-hashes.txt`.
- A `dev_tools` build stops at start on two `FixedUpdate` systems with no order between
  them, and warns on an update over 16 ms.

## Verification
`just shot` saves `target/shots/title.png` and the picture shows the title;
`just replay` passes; `cargo clippy --all-targets --features dev_tools -- -D warnings`
passes.

## Notes
- 2026-11-10: shots of title, gameplay (with a swipe), pause, game-over, settings,
  canvas and safe-area checked by eye, and gameplay with `GAME_STYLE=smooth`.
- 2026-11-10: the replay script's seeds end at a hit (scores 8, 4 and 0). The hash holds
  the run's score, its fixed steps, the player, the enemies and the RNG state.
