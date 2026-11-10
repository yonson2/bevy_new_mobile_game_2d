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
The agent interviews the owner. One topic at a time; for each, give two or three
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
- The milestones: split M3 Content into M3 to M7 as the scope needs, with a goal for each
  in `issues/README.md`. Each new milestone gets its intake issue first and its
  consolidation issue last (`just issue new "Intake: M4 .." -m M4`).
- The owner reads `docs/game.md` and the milestone goals, and approves them.

## Verification
`docs/game.md` exists and the owner approved it; each topic above has a dated answer in
Notes; `just issue check` passes with the new milestones.

## Notes
-
