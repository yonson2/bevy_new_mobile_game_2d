# [011] Trial: a clone runs the interview

**Status:** done
**Milestone:** M0
**Depends on:** [009]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
The template's instructions were written by the session that built it. A fresh agent with no memory of that session must be able to follow them, or the template fails at its one job.

## Approach
Clone the template into a scratch folder. A fresh agent starts there with only this brief: follow the repository's instructions, use a written set of the owner's answers for the interview (a game called Starfall), do [001] and [002] and the desktop part of [003], and report every place where it had to guess. Then fix what it found, here.

## Verification
In the clone, after [002]: `just check` passes and `just shot` shows the title with the game's name. Each finding is fixed here or answered in Notes.

## Notes
- 2026-11-10: the trial agent closed [001] and [002] in the clone with one commit each, wrote `docs/game.md`, split the content into two milestones with their intakes and consolidations, set the identity, deleted `meta/` and `tracker/`. `just check` passed there and the title shot showed "Starfall".
- 2026-11-10: fixed from its findings:
  - `just issue new "A title"` lost its quotes: the recipe now passes its arguments as they are.
  - `AGENTS.md` named the file of [001], which moves when the issue starts, so the docs check failed. It now tests `docs/game.md`.
  - The share text held the app's name a second time: it now reads `APP_NAME`. [002]'s verification grep was wrong.
  - M0 had no branch rule and no close: [001] starts `m0` and records the template's commit first; [003] merges it. The rules say M0 is the short case.
  - [003] mixed agent steps and owner steps, so it could not close: the owner's phones moved to M1's gate [005]. [004] now depends on [003].
  - The milestone split was not specified: [001] and `issues/README.md` now say which lines are by hand and what each intake depends on. `tools/issues.py check` now refuses a milestone that is not in the list, and a started issue that still has the template's text.
  - [002] listed too little for the board's removal, had a vague step for the docs and an icon step with no recipe: rewritten.
  - No way to park an issue: an issue that must stop stays active with a commit `[NNN] wip: ..`. This bends "one issue, one commit"; the owner can refuse it.
- 2026-11-10: not changed: the demo's sidecar that names its voice "Template announcer" (a record of what was made).
- 2026-11-10: the trial did not build for phones and used no key. The fixed instructions were not run again by a second fresh agent.
