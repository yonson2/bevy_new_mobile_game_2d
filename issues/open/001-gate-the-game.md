# [001] Gate: the game

**Status:** open
**Milestone:** M0
**Depends on:** -
**Created:** 2026-11-10
**Closed:** -

## Problem
This repository is a template with a demo in it. Nobody has said yet what the game is.
Everything after this issue depends on the answers.

## Approach
First: write the output of `git log -1 --format=%H` and of `git remote get-url origin`
into Notes (the template's commit and repository; [002] needs them), and start the branch:
`git switch -c m0`.

Then the agent interviews the owner. One topic at a time; for each, give two or three
concrete options and a recommendation, then record the answer in Notes with its date.
Do not start [002] until every topic has an answer or an explicit "later".

1. **The pitch.** The genre, the core loop in one sentence (what the player does each
   10 seconds, each minute, each session), the session length, the games it looks up to,
   and what is new in it.
2. **The pillars.** Three things the game must be, and three it will not be.
3. **The name and the id.** The name under the icon, and the bundle id
   (`com.company.game`). They go into `Cargo.toml` in [002].
4. **The look.** Pixel art or another 2D style (`canvas::STYLE`: `Pixel` or `Smooth`).
   Portrait is the template's default; say so if the game is landscape. Reference
   pictures and a mood. The style itself is M2's work, not this issue's.
5. **The sound.** The music's mood, and voices or none.
6. **The platforms.** iOS, Android or both; the owner's test phones.
7. **The scope of version 1.** The content list (levels, enemies, modes, items), and what
   waits for later. How the game earns: free, paid, ads or purchases (ads and purchases
   need their own issues and store setup).
8. **The same run from a seed.** The template keeps gameplay deterministic (replays,
   daily seeds, the replay check). Keep it unless the owner has a reason.
9. **The issue board.** `tracker/` is a web page of this tracker. Keep it, or delete it.
10. **Languages.** English only, or more (`src/strings.rs` has one table a language).

Then write the result:
- `docs/game.md`: the pitch, the loop, the pillars, the scope, the answers.
- The milestones: M3 to M7 are for the content, as few as the scope needs. For each one
  that the game uses, M3 too: its line in the Milestones list and its goal in
  `issues/README.md`, an intake issue and a consolidation issue
  (`just issue new "Intake: M4 Daily run" -m M4 -d 018`). An intake depends on the
  consolidation of the milestone before it: M3's on [011], and M8's intake [012] on the
  last content milestone's consolidation (edit its `Depends on` line). Put the new issues
  in their place in the Open list.
- The owner reads `docs/game.md` and the milestone goals, and approves them.

## Verification
`docs/game.md` exists and the owner approved it; each topic above has a dated answer in
Notes; `just issue check` passes with the new milestones.

## Notes
-
