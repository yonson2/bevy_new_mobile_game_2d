# The template

This repository is the template itself.

- Repository: https://github.com/yonson2/bevy_new_mobile_game_2d
- Version: 2026.11.0

In a game made from it, issue [002] replaces the rest of this file with the game's
origin and keeps the two lines above. A game made with "Use this template" has no git
history of the template, so the version line is the record of where it started. Code is
copied, not shared: a game owns its copy and changes it freely. A fix that the template
should also have gets a line here, so the owner can take it back.

## Keeping the template current

The template's own work log is `meta/issues/` (`python3 tools/issues.py --dir meta/issues
..`), milestone M1 Upkeep, branch `m1`. Do there:

- a new Bevy release: upgrade, run `just check` and both phone builds;
- a new model investigation (`docs/research/models.md`);
- what the games learned: a pattern or a fix that three games needed belongs here.

A change that a game would want raises the version (year.month.count) in the same
commit. The owner tags the merge on `main` (`git tag v2026.11.0`) and pushes the tag.
