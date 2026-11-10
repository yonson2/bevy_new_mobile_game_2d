# [002] Make the template yours

**Status:** open
**Milestone:** M0
**Depends on:** [001]
**Created:** 2026-11-10
**Closed:** -

## Problem
The repository still has the template's name, its own work log and its README. The
game's history must start clean, and a later fix to the template must be possible to find.

## Approach
1. `Cargo.toml`, `[package.metadata.app]`: the name and the id from [001]. Nothing else
   holds them (`docs/mobile.md`).
2. `src/canvas.rs`: `STYLE` from [001].
3. Record where the game came from: `docs/template.md` with the template's repository
   and the commit this clone started from (`git log -1` before the first game commit).
   A bug found in the template's code is fixed here, and `docs/template.md` gets a line
   so the owner can take the fix back to the template.
4. Delete `meta/` (the template's own work log) and remove it from `tools/check-repo.sh`'s
   list.
5. The board: delete `tracker/` if [001] said so.
6. `README.md`: the game's, not the template's: what the game is, how to run it.
7. `AGENTS.md` and `ARCHITECTURE.md`: replace what describes the template as a template.
   The demo stays until M1 replaces it: the checks need a game to check.
8. Make the app icon's source a neutral placeholder if the demo's robot is not wanted
   (`tools/icon.py`).

## Verification
`just check` passes; `grep -ri "new game\|com.example" Cargo.toml src mobile` finds
nothing; `meta/` is gone; `just shot` shows the title with the game's name.

## Notes
-
