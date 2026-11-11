# [016] Gate: send it to review

**Status:** open
**Milestone:** M9
**Depends on:** [015]
**Owner-run:** yes
**Created:** 2026-11-10
**Closed:** -

## Problem
Only the owner sends a game to the stores.

## Approach
The agent gives the owner one page: what is in the build, what was tested and on which
phones, what is known to be wrong, the ledger's state, and the stores' open questions.
The owner says go, or names what must change first.

On go: close this issue. It is the last of M9, so the version becomes 1.0.0. Commit,
merge the branch (`git switch main && git merge --no-ff m9`). The owner builds 1.0.0
from `main` (`just ios-upload`, `just android-release`) and sends it to review.

## Verification
Notes hold the owner's decision with its date; `Cargo.toml` has version 1.0.0.

## Notes
-
