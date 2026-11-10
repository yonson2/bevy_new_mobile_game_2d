# [010] Issue board (tracker/)

**Status:** done
**Milestone:** M0
**Depends on:** [001]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
The tracker is Markdown files. The owner wants to see the milestones and their progress at a glance, on a phone too. Not every game wants it, so it must be easy to delete.

## Approach
`tracker/`: a small Astro site (one dependency) that reads the index and the issue files as `tools/issues.py` writes them. One board page (the milestones with their progress and goal; the active, open and done lists) and one page for each issue (its fields, its dependencies both ways as links, its Markdown). `ISSUES_DIR` picks the folder (default `issues`). `just board` starts it. It reads only: the files are the source. The interview asks if the game keeps it.

## Verification
`cd tracker && npm install && npm run build` builds a page for each issue of `issues/`; `ISSUES_DIR=meta/issues npm run build` builds the template's log.

## Notes
- 2026-11-10: built for `issues/` (17 pages) and for `meta/issues` (13 pages); the counts equal `tools/issues.py check`; no broken internal link (72 checked).
- 2026-11-10: shots with headless Chrome at 500 px: the board and an issue page fit the width. Headless Chrome has no window narrower than 500 px, so a real phone width is not checked.
- 2026-11-10: links start at the site's root: a deploy under a sub-path needs Astro's `base`.
- 2026-11-10: a sub-agent wrote it from a brief; the session checked the build and the pages.
