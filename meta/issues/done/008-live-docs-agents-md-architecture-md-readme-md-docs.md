# [008] Live docs: AGENTS.md, ARCHITECTURE.md, README.md, docs/

**Status:** done
**Milestone:** M0
**Depends on:** [005]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
An agent starts each session with no memory. It needs one short entry file with the rules and a map, and one description of how the code is built. Both go stale unless something checks them, and both grow until nobody reads them.

## Approach
- `AGENTS.md` (not `CLAUDE.md`): start here, the hard rules, the commands, a table of where to read more. Short on purpose.
- `ARCHITECTURE.md`, after architecture.md and fitted to a game: the structure, the app and its states, the modules and how they talk, data, outside services, builds, the invariants, testing, a glossary.
- `README.md` for a person: what it is, how to start a game, what is in it, requirements, commands.
- `docs/`: `dev-harness.md`, `rendering.md`, `mobile.md`, `assets.md`, `template.md`, `research/models.md`; stubs for `game.md`, `art-style.md`, `ui-style.md`, `audio-style.md` that name the issue that writes each.
- Live: a rule in `AGENTS.md` (a change that makes a doc wrong fixes it in the same commit), each consolidation checks the docs, and `tools/check-docs.py` in `just check`: a size limit for each doc (the sign to split one into `docs/`) and every repository path named in backticks must exist.

## Verification
`just check` passes with the docs check; a doc that names a path that does not exist fails it.

## Notes
- 2026-11-10: limits: `AGENTS.md` and `README.md` 120 lines, `ARCHITECTURE.md` and each doc 250. Now: 79, 70, 138, and 102 for the longest doc.
- 2026-11-10: the check found one wrong path while the docs were written. Paths in fenced blocks are examples and are not checked.
- 2026-11-10: checked against the code: the fixed step (Bevy's default, 64 a second), linear sampling in `Style::Smooth` (no `ImagePlugin` override), the scenes and variables of `src/dev_tools.rs`.
- 2026-11-10: no licence file: the owner decides (`README.md`, "Licence").
