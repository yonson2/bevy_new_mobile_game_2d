# [003] Prove the builds and the tools on this machine

**Status:** open
**Milestone:** M0
**Depends on:** [002]
**Created:** 2026-11-10
**Closed:** -

## Problem
A build or a key that fails in M5 costs a day in M5. Find it now, while nothing
depends on it.

## Approach
1. `just check` and `just shot` on the desktop.
2. `just ios-sim --features dev_tools`, then a screenshot of the simulator; `just android
   --features game/dev_tools` and the emulator with no window (`docs/mobile.md`). An agent
   uses the simulator and the emulator only. The owner's phones come at M1's gate ([005]).
3. The keys: `tools/audio.py credits`, `tools/pixellab.py balance`, and one draft image
   (`tools/image.py .. --preset draft`). Record what remains. A missing key is a question
   for the owner, not a reason to skip the step.
4. Close M0: merge the branch (`git switch main && git merge --no-ff m0`).

## Verification
`just shot`, the simulator and the emulator each give a picture of the title with the
game's name; the three tools answer; `m0` is merged into `main`.

## Notes
-
