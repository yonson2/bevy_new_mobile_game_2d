# [003] Prove the builds and the tools on this machine

**Status:** open
**Milestone:** M0
**Depends on:** [002]
**Owner-run:** yes
**Created:** 2026-11-10
**Closed:** -

## Problem
A build or a key that fails in M5 costs a day in M5. Find it now, while nothing
depends on it.

## Approach
1. `just check` on the desktop.
2. `just ios-sim --features dev_tools`, then a screenshot of the simulator; `just android
   --features game/dev_tools` and the emulator with no window (`docs/mobile.md`). An agent
   uses the simulator and the emulator only.
3. The keys: `tools/audio.py credits`, `tools/pixellab.py balance`, and one draft image
   (`tools/image.py .. --preset draft`). Record what remains.
4. Owner-run: `just ios-device` and `just android-run` on the owner's phones (`IOS_TEAM`
   in `.env`); the upload keystore for Android (`docs/mobile.md`).

## Verification
A shot from the simulator and one from the emulator show the title with the game's
name; the three tools answer; Notes say which phones ran the game.

## Notes
-
