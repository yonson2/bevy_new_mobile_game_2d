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
   holds them; `docs/mobile.md` shows them in an example, change that too.
2. `src/canvas.rs`: `STYLE` from [001].
3. Record where the game came from: replace `docs/template.md` with the template's
   repository and the commit this clone started from ([001]'s Notes have both). A bug
   found in the template's code is fixed here, and `docs/template.md` gets a line so the
   owner can take the fix back to the template.
4. Delete `meta/` (the template's own work log) and its entry in the loop of
   `tools/check-repo.sh`.
5. The board: if [001] said to drop it, delete `tracker/`, the `board` recipe in the
   `justfile`, and its lines in `AGENTS.md`, `ARCHITECTURE.md` and `README.md`.
6. `README.md`: the game's, not the template's: what the game is, how to run it.
7. `just check` names what still points at a deleted path.

The demo, its icon and its sounds stay until M1 replaces the demo and M2 decides the
look: the checks need a game to check. The title shows the game's name over the demo.

## Verification
`just check` passes; `grep -rn "New Game\|com.example.newgame" Cargo.toml src` finds
nothing; `meta/` is gone; `just shot` shows the title with the game's name.

## Notes
-
