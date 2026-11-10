# Dev harness

How an agent sees and plays the game with no window. All of it is in `src/dev_tools.rs`
and compiles only with the `dev_tools` feature (`just dev`, `just shot`).

## A picture of a scene

```sh
just shot                                          # the title, target/shots/title.png
GAME_SHOT_SCENE=gameplay just shot target/shots/play.png
GAME_STYLE=smooth GAME_SHOT_SCENE=gameplay just shot target/shots/smooth.png
```

A shot run opens no window and does not take the focus. Each frame advances time by a
fixed step, so the same seed gives the same picture. Read the PNG to see the result.

| Variable | Meaning |
|---|---|
| `GAME_SHOT=<file.png>` | run a scene, save one frame, exit (`just shot` sets it) |
| `GAME_SHOT_SCENE=<scene>` | the scene of the shot (default `title`) |
| `GAME_SCENE=<scene>` | a run with a window starts in the scene (`GAME_SCENE=gameplay just dev`) |
| `GAME_SEED=<u64>` | the run's seed (`just shot` uses 1) |
| `GAME_STYLE=pixel\|smooth` | the canvas style for this run |
| `GAME_SWIPE=<x0>,<y0>,<x1>,<y1>,<ms>[,hold][;..]` | touch swipes in logical pixels (the window is 390 x 844), one after the other; the shot waits for their end |
| `GAME_SHOT_WAIT=<secs>` | game seconds after the swipes before the shot (default 0.5) |
| `GAME_SHOT_FPS=<n>` | frames a second of the shot run (default 60) |
| `GAME_SHOT_SIZE=<w>x<h>` | the picture's size (default 1080 x 2400) |

Scenes: `title`, `gameplay`, `pause`, `game-over`, `settings`, `canvas` (the pixel grid
and the canvas's corners), `safe-area` (the insets of a phone with a notch), `haptics`
(buttons for each haptic), `stress` (many sprites, for frame times), `replay` (the
scripted run of the replay check).

A scripted swipe goes through the input layer, so gameplay sees it. Bevy's picking does
not see it, so it presses no button: to show a screen, add a scene that sets the states.

**A new screen or a new state of the game gets a scene** in `DevScene`, in the issue that
adds it. A screen without a scene cannot be checked.

## The log

A `dev_tools` build logs each `Screen` change, each gesture, each sound that starts, and
a warning for each update over 16 ms: `slow update: <update> (frame <frame>)`. The update
is the game's logic without the rendering. The Android emulator renders in software: its
times say nothing about a phone.

## The replay check

`just replay` plays the `replay` scene with seeds 1, 2 and 3 at 30, 60 and 120 frames a
second, without the renderer. Each run logs `GAME_STATE step <n>: <hash> (<summary>)`.
The hash must be the same at the three frame rates and equal its line in
`tools/replay-hashes.txt`.

- The hash comes from `state_hash` in the game's module: it must cover everything that
  gameplay changes (positions by their bits, counters, the RNG state).
- A change to the rules moves the hashes. Update `tools/replay-hashes.txt` in the same
  commit and say why in the issue. A change that should not move them, and does, is a bug.
- A difference between frame rates means gameplay depends on the frame: look for
  gameplay outside `FixedUpdate`, a missing order between two systems, or a reaction to a
  state change (it lands only with the next frame).

## On the phones

`just ios-sim --features dev_tools` and `just android --features game/dev_tools` build
with the dev tools. The variables reach the app as `SIMCTL_CHILD_GAME_SCENE=..` on the
simulator and as `-e GAME_SCENE ..` on `adb shell am start` (`docs/mobile.md`).
