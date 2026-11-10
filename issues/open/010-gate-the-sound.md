# [010] Gate: the sound

**Status:** open
**Milestone:** M2
**Depends on:** [008]
**Created:** 2026-11-10
**Closed:** -

## Problem
Sound made piece by piece does not fit together. The owner decides the direction on
real takes.

## Approach
1. Two or three takes of: the main music loop, three sound effects of the core loop,
   and a voice line if the game has voices (`tools/audio.py`). Measure the takes
   (`tools/encode.py measure`); a silent or clipped take does not go to the owner.
2. The owner listens on a review page and picks.
3. Write `docs/audio-style.md`: the music's mood, tempo and key, the effects' character,
   the levels (`--peak`), the prompts that worked, the voice ids.
4. Put the picks in the game with their ledger rows.

## Verification
The owner's verdicts are in Notes; `docs/audio-style.md` exists; `tools/ledger.py check`
passes.

## Notes
-
