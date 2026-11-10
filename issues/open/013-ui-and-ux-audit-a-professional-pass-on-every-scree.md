# [013] UI and UX audit: a professional pass on every screen

**Status:** open
**Milestone:** M8
**Depends on:** [012]
**Created:** 2026-11-10
**Closed:** -

## Problem
Each screen was approved alone, while it was built. Nobody has judged the whole: how
the elements read together, how they move and feel, and whether a new player knows what
to do. This is the work that a professional team does before a launch.

## Approach
Do it with the most capable model available, at high effort, in its own session.

1. **A reference study**, `docs/research/ui-references.md`: how the best games of this
   genre, and the best portrait phone games, build their interfaces. For each game: what
   it does, why it works, and what this game takes from it.
2. **An audit of each screen and moment**: every screen, every overlay, the HUD in play,
   each transition. Each one against:
   - hierarchy and legibility at phone size, and contrast;
   - thumb reach and hit areas, one-handed;
   - consistency: one button language, one colour for each meaning, the icons;
   - motion and timing: how each element enters and leaves, its easing and duration;
   - feedback: the sound, the haptic and the visual for each action;
   - how the elements work together: overlaps, layers, what hides what;
   - states: the first time, empty, long names, huge numbers, the safe area on each phone;
   - accessibility and the comfort settings.
3. **The clarity of the goals**: what the player must understand at each moment, and how
   the screen says it. The first minute of a new player.
4. **The method**: shots of every scene at two phone sizes (`GAME_SHOT_SIZE`); a screen
   recording of whole runs on a phone, read frame by frame for timing; the owner's notes
   from play.
5. **The result**: `docs/research/ui-audit.md`: the findings ranked by impact and cost,
   each with a shot. Mock-ups of the top fixes on a review page. Each fix the owner
   accepts becomes an M8 issue, or is done here when it is small.

## Verification
`docs/research/ui-references.md` and `docs/research/ui-audit.md` exist, with shots; the
owner approved the top fixes on a review page; each accepted fix has its issue.

## Notes
-
