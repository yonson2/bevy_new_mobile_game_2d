# [006] Consolidation: M1

**Status:** open
**Milestone:** M1
**Depends on:** [005]
**Created:** 2026-11-10
**Closed:** -

## Problem
A milestone leaves duplicates, shortcuts and stale docs behind. The next milestone
builds on them unless a pass removes them now.

## Approach
A pass over everything M1 added. No new feature.
1. Duplicates: one function, one constant, one widget for each thing.
2. Patterns: one plugin a module, gameplay in `FixedUpdate` with an order, sounds and
   haptics as messages, strings in `src/strings.rs`, no magic numbers in systems
   (`ARCHITECTURE.md`, "Invariants").
3. Reading: names that say what a thing is, comments that say why, dead code removed.
4. Performance: the stress scene and `GAME_SCENE` runs with the slow-update log; fix what
   is over budget or file it.
5. Tests: the rules that are easy to break have a test.
6. Docs: `ARCHITECTURE.md`'s code map, `AGENTS.md`, `docs/` say what the code now does.
7. Close the milestone: `just check`, merge the branch into `main`
   (`git switch main && git merge --no-ff m1`). The owner pushes.

## Verification
`just check` passes before and after; the diff adds no feature; `ARCHITECTURE.md`
names every module under `src/`.

## Notes
-
