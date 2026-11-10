# [011] Consolidation: M2

**Status:** open
**Milestone:** M2
**Depends on:** [009], [010]
**Created:** 2026-11-10
**Closed:** -

## Problem
M2 adds pilots, style docs and tool settings. What stays must be in order before the
content milestones multiply it.

## Approach
The consolidation steps ([006] lists them) on what M2 added, and:
- the style docs name each reference file, and each file exists;
- raw takes that were not picked are out of the tree (`art-src/raw/` and
  `audio-src/takes/` are not in git);
- merge `m2` into `main`.

## Verification
`just check` passes; each path named in `docs/art-style.md`, `docs/ui-style.md` and
`docs/audio-style.md` exists.

## Notes
-
