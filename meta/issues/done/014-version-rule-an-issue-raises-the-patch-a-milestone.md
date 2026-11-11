# [014] Version rule: an issue raises the patch, a milestone the minor, the release is 1.0.0

**Status:** done
**Milestone:** M1
**Depends on:** -
**Created:** 2026-11-11
**Closed:** 2026-11-11

## Problem
A game's version changed only when someone remembered to edit `Cargo.toml`. The owner wants the version to follow the work, with 1.0.0 as the store release.

## Approach
- `tools/issues.py close` raises the version in `Cargo.toml` and `Cargo.lock`, for the game's tracker only (not with `--dir`): the patch for an issue; the minor (patch 0) for the issue that completes its milestone; 1.0.0 when that milestone is the one the Milestones list names `Release` and the version is 0.x.
- The apps already read `Cargo.toml` (the `justfile`, Gradle): no change there.
- The rule is in `AGENTS.md` (hard rule 11), `issues/README.md` ("Rules") and `docs/mobile.md`.
- Seeded issues: [015] files no version issue; [016], M9's last issue, closes on the owner's go, which gives 1.0.0, merges `m9`, and the owner builds 1.0.0 from `main`.
- Template version 2026.11.1.

## Verification
`just check` passes. In a copy, closing the game's issues 1 to 16 in order gives 0.1.1, 0.1.2, 0.2.0 (M0 done) .. 0.5.1, 1.0.0 (M9 done); two more M9 issues give 1.0.1 and 1.1.0; `Cargo.lock` follows.

## Notes
- 2026-11-11: the owner's decision: `Cargo.toml` is the source, the apps follow, the version rises with issues and milestones, 1.0.0 is the store release.
- 2026-11-11: the release is found by the milestone's name (`Release`), not by "the last milestone" or "no issue left open": both of those break when the owner files work for after the launch before the launch.
- 2026-11-11: the test tracks in M9 get 0.x builds; the 1.0.0 build is made after the gate closes, by the owner. A review that asks for a fix gives 1.0.1 in the store: accepted.
- 2026-11-11: the build number stays major x 10000 + minor x 100 + patch. 100 closes in one milestone would repeat a number; `docs/mobile.md` says to split the milestone. Not guarded in code.
- 2026-11-11: not verified: a phone build after a bump (the build files did not change).
