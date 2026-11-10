# Architecture

How the code is built and why. A live document: an issue that changes the structure
changes this file in the same commit, and each milestone's consolidation checks it
against the code. When it passes its size limit, a section moves into `docs/`.

## 1. Project structure

```
Cargo.toml            the package `game`; [package.metadata.app] is the app's name and id
build.rs              gives the name and id to the code (APP_NAME, APP_ID)
src/
  main.rs             the desktop and iOS binary: game::app().run()
  lib.rs              app(): the plugins, the system sets, the window
  screens/            the Screen states: loading, title, settings, gameplay, game_over
  demo/               the demo game (M1 replaces it with the game)
  theme/              palette, font, widgets, press feedback
  canvas.rs           how the world reaches the screen: pixel or smooth
  input.rs            touches and the mouse to gestures
  rng.rs              the seeded random streams
  save.rs             the save file and the app's data folder
  audio.rs            sounds as messages, music for each screen
  haptics.rs          haptics as messages; iOS and Android back ends
  platform.rs         the safe area, suspend and resume, Android's Back
  share.rs            the system share sheet
  strings.rs          every text a player sees
  asset_tracking.rs   assets that load before the title
  dev_tools.rs        shots, scenes, scripted touches (feature `dev_tools`)
mobile/               the Android library crate and both phone projects
assets/               what ships: images, audio, fonts
art-src/ audio-src/   sources, sidecars and the ledgers of what ships
tools/                the tracker, the checks and the asset tools
issues/               the tracker
tracker/              the tracker as a web page
docs/                 the detail behind this file and AGENTS.md
```

## 2. The app

`game::app()` builds one Bevy `App` from plugins. Each module has one
`pub(super) fn plugin(app: &mut App)` that adds its resources, messages and systems;
a folder's `mod.rs` adds its children. Nothing outside a module reaches into its systems.

```
            Loading --> Title <--> Settings
                          |
                          v
        GameOver <--- Gameplay  (sub-state Pause: Running | Paused)
            |  ^          ^
            +--+----------+   play again, or back to Title
```

- `Screen` is the top state. A screen's entities carry `DespawnOnExit(Screen::X)`.
- `Pause` is a sub-state of `Screen::Gameplay`. `PausableSystems` do not run while paused.
- `AppSystems` orders each frame and each fixed step: `TickTimers`, `RecordInput`,
  `Update`.
- UI and entities are BSN scenes (`bsn!`), spawned with `commands.spawn_scene` or as a
  system with `.spawn()`. The widgets are in `src/theme/widget.rs`.

## 3. Core components

| Module | Its job | Talks to others by |
|---|---|---|
| `input` | one touch or the mouse to `Gesture` messages (drag, flick, swipe, tap, double tap, long press) | `MessageReader<Gesture>` |
| `canvas` | cameras, the canvas size for this screen, `Canvas::load` for images | the `Canvas` resource |
| `rng` | `GameRng`: one seed, one stream for each purpose | `rng.stream(Stream::X)` |
| `audio` | `Sfx` messages play sounds; the music follows the `Screen` | `MessageWriter<Sfx>` |
| `haptics` | `Haptic` messages (tick, pulse, rumble) | `MessageWriter<Haptic>` |
| `save` | `Save` resource; a change writes the file at the frame's end | `ResMut<Save>` |
| `platform` | `SafeArea`; `InSafeArea` / `InSafeTop` pad a UI node | components |
| `share` | the share sheet | `commands.trigger(Share { .. })` |
| `strings` | one table of texts for each language | `strings().x` |
| `theme` | colours, font sizes, widgets | `widget::button(..)` in BSN |

## 4. Data

- The save: `save.json` in the app's data folder, written whole to a temporary file and
  then renamed. `#[serde(default)]` on `Save`: an old save loads in a new version.
- No server and no account. The game works with no network.

## 5. Outside services

At content time only, never in the game: OpenRouter (images), PixelLab (pixel art),
ElevenLabs (sound). `docs/assets.md`.

## 6. Builds

- Desktop: `cargo run`. iOS: Xcode's one build phase runs `cargo build` for the `game`
  binary. Android: `cargo ndk` builds the `mobile` library, Gradle packs it.
- The name, the id and the version are in `Cargo.toml` only. `docs/mobile.md`.
- Profiles: `dev` (the game at opt-level 1, dependencies at 3), `ci` (no optimisation),
  `release` (thin LTO, line tables for crash reports).

## 7. Invariants

Break one only in an issue that says why.

1. **A seed plays the same run** on every target and at every frame rate.
   - Gameplay systems run in `FixedUpdate`, in system sets with an explicit order. With
     `dev_tools`, two fixed systems with no order between them stop the app at start.
   - Gameplay draws random numbers only from `GameRng` streams, and only as `u64`.
   - Float functions come from `bevy::math::ops` (one portable libm). `clippy.toml`
     forbids the `f32` methods that call the platform's libm.
   - Entities that a system visits in an order that matters are sorted by place
     (`rng::by_place`), never by entity id.
   - A state change lands with the next frame, and a frame can hold several fixed steps.
     What must stop in the step itself checks a resource (the demo's `Run::over`).
   - `just replay` proves it; `tools/replay-hashes.txt` pins it.
2. **No work in a release build that a player does not need**: `dev_tools` is a feature,
   and `info!` and lower compile out of release builds.
3. **Player text is in `src/strings.rs`**, never inline.
4. **Gameplay does not know the screen's pixels.** It works in world units; `Canvas`
   converts (`docs/rendering.md`).
5. **Sounds and haptics are messages.** Gameplay never touches the audio or the phone.
6. **A system reads its numbers from named constants** beside it, with their unit.
7. **An asset loads through a `LoadResource` on the loading screen**, so the title never
   shows a missing image.

## 8. Development and testing

- `just check` is the gate: format, the trackers, the ledger, the docs, clippy with and
  without `dev_tools` (warnings are errors), the tests, the replay.
- Unit tests sit beside the code they test, for rules that are easy to break.
- An agent sees the game through shots and plays it through scripted touches
  (`docs/dev-harness.md`).

## 9. Glossary

- **Canvas pixel**: the unit of the art and the fonts; the screen is about 360 wide.
- **World unit**: the unit of gameplay; two canvas pixels.
- **Fixed step**: one run of `FixedUpdate`, 64 a second.
- **Stream**: one sequence of random numbers for one purpose, from the run's seed.
- **Scene** (dev tools): a named starting state for a shot or a run.
- **Sidecar**: the JSON beside a generated file that says how it was made.
- **Ledger**: the table of what ships, with hash, source, licence terms and verdict.
- **Gate**: an issue that waits for the owner's decision. **Owner-run**: a step only the
  owner can do.
- **Intake**, **consolidation**: the first and the last issue of a milestone.
