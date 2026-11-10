# bevy_new_mobile_game_2d

A template for 2D games on iOS and Android with Rust and [Bevy](https://bevyengine.org),
made to be worked on by an AI agent together with its owner. It follows the layout of
[bevy_new_2d](https://github.com/TheBevyFlock/bevy_new_2d) and adds what a phone game
needs and what an agent needs to work alone between your decisions.

## Start a game

1. Make the game's repository from this template, then put your keys in `.env`
   (`.env.example`) or export them in your shell:

   ```sh
   gh repo create my-game --template yonson2/bevy_new_mobile_game_2d --private --clone
   ```

   "Use this template" on GitHub does the same. The game starts with one commit and no
   link to the template's history: it owns its copy.
2. Start your agent in the folder. `AGENTS.md` sends it to issue [001], `Gate: the game`:
   it interviews you (the pitch, the name, pixel art or not, the platforms, the scope).
3. The agent writes the design and the milestones from your answers, makes the
   repository the game's ([002]), proves the builds and the tools ([003]), and starts the
   first milestone with its intake.

You decide at the gates, play the builds on your phone, judge art and sound on review
pages, and push.

## What is in it

| | |
|---|---|
| **The app** | Bevy 0.20 with BSN scenes: loading, title, settings, pause and game over screens; a theme with widgets; one text table. |
| **Phones** | Xcode and Gradle projects; safe areas, suspend and resume, haptics, the share sheet, Android's Back; the name and the id in one place. |
| **Input** | touches and the mouse as gestures: drag, flick, swipe, tap, double tap, long press. |
| **Two looks** | `Style::Pixel` (a low-resolution canvas at a whole-number scale) or `Style::Smooth`. |
| **The same run from a seed** | fixed steps, seeded streams, portable float math, and a replay check that proves it at 30, 60 and 120 fps. |
| **Eyes for the agent** | pictures of any scene with no window, scripted touches, a slow-update log, a stress scene. |
| **A demo** | a small dodge game that uses each part once. The first milestone replaces it. |
| **An issue tracker** | Markdown files under `issues/`, one commit an issue, milestones that start with an intake and end with a consolidation; an optional web board in `tracker/`. |
| **An asset pipeline** | OpenRouter for images, PixelLab for pixel art and animation, ElevenLabs for effects, music and voices; a sidecar for each generation, a ledger with licence terms, review pages. |
| **Checks** | `just check` and the same in CI. |

## Requirements

- Rust (stable) and [`just`](https://github.com/casey/just).
- Python 3.11 or newer with Pillow; `ffmpeg` and `oggenc` (vorbis-tools) for the audio.
- iOS: Xcode, and `rustup target add aarch64-apple-ios aarch64-apple-ios-sim`.
- Android: the SDK with NDK 26.1.10909125, Android Studio (its JDK runs Gradle),
  `cargo install cargo-ndk`, `rustup target add aarch64-linux-android`.
- The board: Node.

## Commands

```sh
just dev                           # run on the desktop with the dev tools
just check                         # every check
GAME_SHOT_SCENE=gameplay just shot # a picture of a scene, no window
just ios-sim                       # build, install and start on the iOS simulator
just android-run                   # build, install and start on an Android device
just issue new "Title" -m M1       # the tracker
just board                         # the tracker as a web page
```

`just --list` shows them all.

## Read next

- `AGENTS.md`: the rules and the map for an agent.
- `ARCHITECTURE.md`: how the code is built.
- `issues/README.md`: the tracker and the milestones.
- `docs/`: the dev harness, rendering, phones, assets, the model investigation.

## Licence

The code and the docs: MIT No Attribution (`LICENSE`). A game made from the template
owes no notice and no credit.

Not under that licence:

- **The demo's art and sound** (`assets/images`, `assets/audio`, `art-src/sprites`,
  `art-src/icon`, `audio-src`, the app icons under `mobile/`). They are generated, they
  show the asset pipeline, and they are not licensed for reuse. M1 and M2 replace them
  with the game's own. Their terms are in `art-src/ledger.md` and `audio-src/ledger.md`.
- Departure Mono: SIL OFL 1.1 (`art-src/fonts/DepartureMono-OFL.txt`).
- The Gradle wrapper: Apache 2.0.

The layout and parts of the code come from bevy_new_2d (CC0-1.0, MIT or Apache-2.0).
