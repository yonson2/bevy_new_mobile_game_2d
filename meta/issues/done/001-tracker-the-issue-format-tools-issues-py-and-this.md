# [001] Tracker: the issue format, tools/issues.py and this log

**Status:** done
**Milestone:** M0
**Depends on:** -
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
The games before this template kept an index, a done log and a next ID by hand. A close
took four edits in three files, and only the optional board checked the files.

## Approach
- One file per issue in `open/`, `active/`, `done/`, made from `TEMPLATE.md`. The files are
  the only source.
- `tools/issues.py`: `new`, `start`, `close`, `sync`, `check`. It writes the marked blocks
  of `README.md` (next ID, Active, Open, the milestone boxes) and `done-log.md`. The order
  of the Open list stays by hand.
- Two trackers with one tool: `issues/` for the game, `meta/issues/` for the template.

## Verification
`python3 tools/issues.py --dir meta/issues check` passes after a `new`, a `start` and a
`close` made with the tool.

## Notes
- 2026-11-10: the board (`tracker/`, [010]) reads the same `README.md` sections
  (`## Open`, `## Milestones`, `## Milestone goals`), so their line formats are fixed:
  `- [NNN] ...`, `- [x] M0 Name`, `- **M0 Name:** goal`.
- 2026-11-10: a milestone's box is ticked when it has issues and all are done. No hand
  edit.
