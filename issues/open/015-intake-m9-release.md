# [015] Intake: M9 Release

**Status:** open
**Milestone:** M9
**Depends on:** [014]
**Created:** 2026-11-10
**Closed:** -

## Problem
A release is a list of things that each stop the launch if one is missing.

## Approach
The intake steps ([004] lists them) on M9's goal. File an issue for each of: the final
app icon (`tools/icon.py`); the store listing (name, texts, key art, screenshots from
`GAME_SHOT` at the stores' sizes); the privacy answers of both stores and
`mobile/ios/ios-src/PrivacyInfo.xcprivacy`; the signed
builds (`just ios-upload`, `just android-release`; owner-run); the test tracks
(TestFlight, Play internal testing); `tools/ledger.py check --final` (every asset
approved, every licence known, including the music terms); a last pass on real phones.
Start the branch `m9`. File no issue for the version: the test tracks get 0.x builds, and
the close of [016], M9's last issue, makes it 1.0.0. Keep [016] last.

## Verification
Notes map each clause of M9's goal to an issue; `just issue check` passes.

## Notes
-
