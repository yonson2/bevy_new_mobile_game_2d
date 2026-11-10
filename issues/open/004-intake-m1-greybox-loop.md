# [004] Intake: M1 Greybox loop

**Status:** open
**Milestone:** M1
**Depends on:** [003]
**Created:** 2026-11-10
**Closed:** -

## Problem
M1's goal is the core loop in plain shapes. The loop is in `docs/game.md`; the issues
that build it do not exist yet.

## Approach
The intake steps, as for every milestone:
1. Read the milestone's goal (`issues/README.md`) and `docs/game.md`.
2. File the issues that reach the goal: small, each with a runnable Verification. For
   M1: the game's states and data, the input, the loop's rules in `FixedUpdate` with
   seeded draws, the HUD, a first tuning pass, the replay scene for the game's own loop
   (`tools/replay-hashes.txt`), and the removal of `src/demo/`.
3. Map each clause of the goal to its issues in Notes. File what is missing.
4. Ask the owner what only the owner can answer; record the answers.
5. Order the milestone in the Open list. [005] and [006] stay last.
6. Start the milestone's branch: `git switch -c m1`.

Greybox means no generated art and no generated sound: shapes, the theme's colours, the
placeholder font. The question is whether the loop is fun, and art hides the answer.

## Verification
Notes map each clause of M1's goal to an issue; `just issue check` passes; the Open
list has M1's issues in order.

## Notes
-
