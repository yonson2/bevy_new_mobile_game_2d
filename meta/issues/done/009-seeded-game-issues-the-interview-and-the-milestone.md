# [009] Seeded game issues: the interview and the milestone spine

**Status:** done
**Milestone:** M0
**Depends on:** [008]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
A clone must know what to do first. The owner's flow: clone, start an agent, the agent interviews the owner, sets the repository up and files the first milestones and issues. Without seeded issues, each game starts from a blank tracker and forgets the steps that every game needs.

## Approach
`issues/` ships with a milestone spine and 16 issues:
- M0 Foundations: [001] `Gate: the game` (the interview: pitch, pillars, name and id, pixel art or smooth, sound, platforms, scope, determinism, the board, languages; it writes `docs/game.md` and splits M3 Content into the game's milestones), [002] make the template yours (identity, style, `docs/template.md` with the template commit, delete `meta/`, the board or not), [003] prove the builds and the keys.
- M1 Greybox loop: intake, `Gate: the loop is fun on a phone`, consolidation.
- M2 Style lock: intake, the model investigation, `Gate: the look`, `Gate: the sound`, consolidation.
- M3 Content: empty; the interview fills M3 to M7.
- M8 Polish: intake, the UI and UX audit, consolidation. M9 Release: intake, `Gate: send it to review`.
- `issues/README.md` has the rules: one issue one commit, each milestone starts with its intake and ends with its consolidation, gates, owner-run, runnable verification.

## Verification
`python3 tools/issues.py check` passes on `issues/`; `tools/check-repo.sh` checks both trackers.

## Notes
- 2026-11-10: the owner's answers behind it: the tracker board and the art style are questions of the interview; code is copied, not shared, and each game records its template commit; determinism is the default.
- 2026-11-10: milestone ids M3 to M7 are free for the game's content, so the interview adds milestones with no renumbering.
- 2026-11-10: [011] tests the interview on a clone.
