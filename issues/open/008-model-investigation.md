# [008] Model investigation

**Status:** open
**Milestone:** M2
**Depends on:** [007]
**Created:** 2026-11-10
**Closed:** -

## Problem
The image, pixel art and audio models change every few weeks. `tools/models.toml` holds
the picks of the template's last investigation, which is old by now.

## Approach
Follow `docs/research/models.md`, "How to investigate": list what each provider has
today from its API, read two independent boards for the image models, and compare with
the current picks. Add a dated section at the top of `docs/research/models.md` and update
`tools/models.toml` in the same commit. Record what was not verified. The pilots in [009]
and [010] then test the picks on the game's real subjects.

## Verification
`docs/research/models.md` has a section with today's date and its sources; each model
id in `tools/models.toml` answered a real call (the sidecars are in `art-src/raw/` or
`audio-src/takes/`).

## Notes
-
