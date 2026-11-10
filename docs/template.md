# The template

This repository is `bevy_new_mobile_game_2d`, the template itself.

In a game made from it, issue [002] replaces this file with the game's origin: the
template's repository and the commit the game started from. Code is copied, not shared:
a game owns its copy and changes it freely. A fix that the template should also have gets
a line here, so the owner can take it back.

## Keeping the template current

The template's own work log is `meta/issues/` (`python3 tools/issues.py --dir meta/issues
..`), milestone M1 Upkeep. Do there:

- a new Bevy release: upgrade, run `just check` and both phone builds;
- a new model investigation (`docs/research/models.md`);
- what the games learned: a pattern or a fix that three games needed belongs here.
