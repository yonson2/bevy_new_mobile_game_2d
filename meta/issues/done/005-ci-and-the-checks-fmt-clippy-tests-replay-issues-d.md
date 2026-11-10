# [005] CI and the checks: fmt, clippy, tests, replay, issues, doc sizes

**Status:** done
**Milestone:** M0
**Depends on:** [003]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
A change can break the format, a lint, a test, the replay or a tracker, and nobody sees it until later. The checks must be one command on the Mac and the same checks on each push.

## Approach
- `just check`: `cargo fmt --check`, `tools/check-repo.sh` (each tracker is valid and its index is in sync), clippy with and without `dev_tools`, the tests, the replay check.
- `.github/workflows/ci.yml`: the same checks on Linux in the `ci` profile (no optimisation, so a cold build is fast).

## Verification
`just check` passes. The test job's commands pass in a Linux container.

## Notes
- 2026-11-10: `just check` passes on the Mac.
- 2026-11-10: the test job in a Linux container (Podman, `rust:latest`, rustc 1.99.1, aarch64, Bevy's Linux packages from the workflow): 24 tests pass, and the replay gives the three pinned hashes. So a seed plays the same run on macOS (optimised) and Linux (not optimised).
- 2026-11-10: not checked: the workflow on GitHub itself (the owner pushes), and x86_64.
- 2026-11-10: the size limits of the docs go into [008] with the docs.
