# [002] Skeleton and the demo game

**Status:** done
**Milestone:** M0
**Depends on:** [001]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
A new game starts from the same parts each time: the app with its plugins and states,
the phone platform code, the input layer, the save, the theme. Without a small game on top,
the template's checks would pass on an empty app and prove nothing.

## Approach
- The bevy_new_2d layout on Bevy 0.20.1, with BSN scenes: a library crate
  (`app()`), a thin binary, `mobile/` for Android; one plugin per module.
- `src/`: `asset_tracking`, `audio` (sounds as messages, music per screen with a
  crossfade), `canvas` (two styles: pixel and smooth), `input` (gestures), `platform`
  (safe area, pause on suspend, Android Back), `haptics`, `share`, `save`, `rng`
  (seeded streams), `strings`, `theme`, `screens` (loading, title, settings, pause, game
  over).
- `src/demo/`: drag the player away from what falls. It uses each part once.
- Determinism: gameplay in `FixedUpdate` on one thread, seeded `u64` draws, portable
  libm (`clippy.toml` forbids the `f32` float methods).

## Verification
`cargo build`, `cargo test` and `cargo clippy --all-targets -- -D warnings` pass with no
warning; `cargo run` shows the title and plays the demo.

## Notes
- 2026-11-10: Bevy 0.20.1 is the newest release (crates.io, checked today). `jni` stays
  at 0.21 and `objc2-ui-kit` at 0.2: the versions that Bevy's winit and android-activity
  use, so the build has one copy of each.
- 2026-11-10: `canvas::STYLE` picks the rig (owner: "we should ask the user" for pixel art
  or another 2D style). `Style::Pixel` draws into a canvas about 360 pixels wide at a
  whole-number scale. `Style::Smooth` draws at the screen's resolution, 360 canvas pixels
  wide. The fonts' `Rem` is a canvas pixel in both.
- 2026-11-10: a state change lands with the next frame, and a frame can hold more than
  one fixed step. The demo stops its gameplay with `Run::over` in the step of the hit; the
  replay check found the difference between 30 and 60 fps before that.
- 2026-11-10: the art and the sounds are placeholders (squares, sine tones) until [007].
- 2026-11-10: no share of the template's licence yet: the owner decides (README).
