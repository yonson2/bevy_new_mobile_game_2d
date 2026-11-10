# [009] Gate: the look

**Status:** open
**Milestone:** M2
**Depends on:** [008]
**Created:** 2026-11-10
**Closed:** -

## Problem
Without one decided style, each generated asset looks like a different game. The
style is the owner's decision, made on real candidates.

## Approach
1. Concepts: two to four directions for one key scene of the game (`tools/image.py`),
   from [001]'s references. The owner picks one on a review page (`tools/review.py`).
2. A pilot in the picked direction with the game's real subjects: the main character, one
   enemy or item, one background or tile, one UI panel. By both routes where both make
   sense: PixelLab directly, and OpenRouter then PixelLab (`docs/assets.md`). For an
   animation, compare the `animation` and `animation_v3` roles.
3. The owner approves or re-rolls each candidate on a review page.
4. Write `docs/art-style.md`: the sizes (frame, tile, canvas), the palette, the outline
   and shading rules, the prompts and settings that worked, the style reference files,
   and what failed. Write `docs/ui-style.md`: the palette in `src/theme/palette.rs`, the
   font, the button and panel language.
5. Put the approved pilot assets in the game with their ledger rows.

## Verification
The owner's verdicts are in Notes; `docs/art-style.md` and `docs/ui-style.md` exist;
a shot of the game shows the pilot assets; `tools/ledger.py check` passes.

## Notes
-
