# [012] Close: consolidation

**Status:** done
**Milestone:** M0
**Depends on:** [011]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
M0 was built in one pass from several sources. Before it becomes `main`, a pass must look for what does not belong: unused code, names from other projects, stale comments, docs that do not match the code.

## Approach
The consolidation steps (`issues/README.md`) on the whole template: search for names and issue numbers from other projects; read the modules for unused code; check each doc claim against the code; run every check; merge `m0` into `main`.

## Verification
`just check` passes; a search for other projects' names finds nothing; `m0` is merged into `main`.

## Notes
- 2026-11-10: the search (project names, old issue numbers, old variable prefixes, the old bundle id and team id) finds nothing in the tree.
- 2026-11-10: removed: a `Serialize` derive on `input::Swipe` that nothing used. Updated the CI file's header.
- 2026-11-10: checked by the builds and runs of M0: desktop, iOS simulator, Android emulator, the Linux container, each asset tool with a real call.
- 2026-11-10: open for M1 Upkeep, not done here: a second trial with the fixed instructions; a build on a real phone and the release paths (they need the owner's team and key); the workflow on GitHub; a licence file; the review page in a browser.
