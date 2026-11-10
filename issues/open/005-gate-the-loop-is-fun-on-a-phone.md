# [005] Gate: the loop is fun on a phone

**Status:** open
**Milestone:** M1
**Depends on:** [004]
**Owner-run:** yes
**Created:** 2026-11-10
**Closed:** -

## Problem
A loop that is not fun in greybox does not become fun with art. This is the cheapest
moment to change the design or to stop.

## Approach
1. The agent builds the game for the owner's phones (`just ios-device`,
   `just android-run`; the first time, the owner puts `IOS_TEAM` in `.env` and pairs the
   phones, `docs/mobile.md`) and gives a short list of what to try and what to watch for.
2. The owner plays several sessions on a phone, not on the desktop.
3. The owner answers: is it fun, what is the best moment, what is the worst, what is
   confusing. The agent records the answers and files the changes as issues.
4. The verdict: go on, change the loop (new issues in M1, then this gate again), or stop.

## Verification
Notes hold the owner's verdict with its date, and each change the owner asked for has
an issue.

## Notes
-
