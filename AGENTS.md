# AGENTS.md

A 2D mobile game in Rust and Bevy, for iOS and Android. This file is the entry point for
an agent: the rules, the commands and where to read more. It is a live document: a
change that makes a line here wrong fixes the line in the same commit. Keep it short;
detail goes into `docs/` (`tools/check-docs.py` holds the size limits).

## Start here

1. Run `just issue check` and read `issues/README.md`: Active first, then the top of Open.
2. If `issues/open/001-gate-the-game.md` exists, this clone is still the template. Start
   that issue: interview the owner, then do [002] and [003]. Do not write game code first.
3. Otherwise take the active issue, or the first open issue whose `Depends on` are done.
   Read `docs/game.md` (what the game is) and `ARCHITECTURE.md` (how the code is built).

## Hard rules

1. **One issue, one commit.** `just issue start N`, the work, the issue's Notes,
   `just issue close N`, then one commit `[NNN] short summary`. Stage explicit paths, not
   `git add -A`. No AI attribution in a commit (no `Co-Authored-By`, no "generated with").
2. **The owner pushes.** Never `git push`. Work on the milestone's branch (`m1`, `m2`..);
   the milestone's consolidation merges it into `main` on this machine.
3. **`just check` passes before every commit.**
4. **Milestones** start with their intake and end with their consolidation
   (`issues/README.md`, "Rules"). **A gate is the owner's decision**: prepare it, ask,
   wait. Do not guess an answer and go on.
5. **Keys.** Never print, log or commit a key. Keys come from the environment, then from
   `.env` (git ignores it; `.env.example` has the names).
6. **Assets.** Every file under `assets/` has a ledger row with its source and licence
   terms (`tools/ledger.py`). A file with unknown licence terms does not ship. A generated
   file keeps its sidecar JSON. The owner approves art and sound on a review page.
7. **Check without a window.** Use `just shot`, the iOS simulator and the Android emulator
   with `-no-window`. Never open a window or take the focus. Never touch the owner's phone
   unless asked: set `ANDROID_SERIAL=emulator-5554` for every adb command.
8. **The same run from a seed.** Gameplay runs in `FixedUpdate` with an explicit order;
   random numbers come from `GameRng` streams as `u64`; float math uses `bevy::math::ops`.
   A change that moves the replay hashes on purpose updates `tools/replay-hashes.txt` in
   the same commit and says why in the issue.
9. **Small and plain.** The least code that does the issue. No feature, option or
   abstraction that the issue did not ask for. Match the patterns that are already here.
10. **Never start Docker.** A container runs with Podman.

## Commands

| Command | What it does |
|---|---|
| `just dev` | run on the desktop with the dev tools |
| `just check` | everything CI runs: format, trackers, ledger, docs, clippy, tests, replay |
| `just shot [file]` | a picture of a scene with no window (`GAME_SHOT_SCENE=gameplay just shot`) |
| `just replay` | seeds 1 to 3 at 30, 60 and 120 fps give the pinned hashes |
| `just issue new "Title" -m M1` / `start N` / `close N` / `check` | the tracker |
| `just ios-sim` / `just android` / `just android-run` | phone builds (`docs/mobile.md`) |
| `just icon` | the app icon from `art-src/icon/icon.png` |
| `just board` | the tracker as a web page (`tracker/`) |
| `tools/image.py`, `tools/pixellab.py`, `tools/audio.py` | generate art and sound (`docs/assets.md`) |

## Where to read more

| Topic | File |
|---|---|
| What the game is | `docs/game.md` |
| The code: modules, states, invariants, patterns | `ARCHITECTURE.md` |
| Shots, scenes, scripted touches, the replay check | `docs/dev-harness.md` |
| The canvas, pixel art against smooth art, units | `docs/rendering.md` |
| iOS and Android: builds, identity, signing, the emulator | `docs/mobile.md` |
| Generating art and sound, the ledger, review pages | `docs/assets.md` |
| The model picks and how to investigate | `docs/research/models.md` |
| The styles (M2 writes them) | `docs/art-style.md`, `docs/ui-style.md`, `docs/audio-style.md` |
| The tracker's rules | `issues/README.md` |
| Where this repository came from | `docs/template.md` |

## Working with the owner

- Ask when a choice is the owner's: the design, the look, the sound, money, the stores.
  Give options and a recommendation. Decide alone what a senior engineer would decide alone.
- Report what happened: a failed check with its output, a skipped step as skipped, what
  was not verified as not verified.
- Record each decision of the owner in the issue's Notes with its date.
- Record what a generation cost (the sidecars have it) in the issue's Notes.
