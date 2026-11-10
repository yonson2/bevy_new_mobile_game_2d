# [013] Public repository: licence, version stamp, GitHub template flow

**Status:** done
**Milestone:** M1
**Depends on:** -
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
The template goes public as a GitHub template repository. It has no licence, and a game made with "Use this template" has no git history of the template, so the commit that [001] recorded as the game's origin does not exist there.

## Approach
- `LICENSE`: MIT No Attribution, so a game made from the template owes no notice. `README.md` says what is not under it: the demo's generated art and sound (not licensed for reuse), the font, the Gradle wrapper.
- A version stamp in `docs/template.md` (repository and `Version: year.month.count`), raised with each change that a game would want and tagged on `main`. [002] keeps the two lines as the game's origin; [001] no longer records a commit.
- `README.md`, "Start a game": `gh repo create my-game --template yonson2/bevy_new_mobile_game_2d --private --clone`.
- [002] deletes the template's `LICENSE` in a game.
- `docs/research/models.md`: the owner's confirmation of the image terms, and the limit on giving generated files to others.

## Verification
`just check` passes; `LICENSE` exists; `docs/template.md` has the Repository and Version lines.

## Notes
- 2026-11-10: the owner's decisions: MIT-0 (not MIT: no notice in each game), the copyright line "Pedro Ramos (yonson2)", a GitHub template repository, the demo's assets stay in the repository and outside the licence, and the `wip` commit rule of [011] stays.
- 2026-11-10: the owner confirmed the OpenRouter and OpenAI terms for the demo's images in a public repository.
- 2026-11-10: bevy_new_2d's licence read from its README: CC0-1.0, MIT or Apache-2.0 at the user's option.
- 2026-11-10: the repository URL assumes the owner's GitHub login (`yonson2`) and this folder's name. The repository does not exist on GitHub yet: the owner creates it, pushes, sets the template flag and pushes the tag `v2026.11.0`.
- 2026-11-10: not read: the PixelLab and ElevenLabs terms for redistribution of generated files.
