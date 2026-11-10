# [004] Phones: iOS and Android projects, the app's identity in one place

**Status:** done
**Milestone:** M0
**Depends on:** [003]
**Created:** 2026-11-10
**Closed:** 2026-11-10

## Problem
The game must build and run on iOS and Android from the first day. A new game needs its own name and id on both, and a change in three project files is easy to get wrong.

## Approach
- `mobile/ios`: the Xcode project `Game`. Its one build phase runs `build_rust_deps.sh`, which builds the `game` binary for the target and puts it in the app.
- `mobile/android`: the Gradle project. `MainActivity` (package `mobile.game`, the same for every game) starts the `mobile` library, hides the system bars, sends the safe area and Back to the game, and gives it haptics and the share sheet.
- Identity in one place: `name` and `id` under `[package.metadata.app]` in `Cargo.toml`. `build.rs` gives them to the code (`APP_NAME`, `APP_ID`), Gradle reads the two lines, and the `justfile` passes them to Xcode as build settings. The version is the `Cargo.toml` version on both.
- `justfile`: `android`, `android-release`, `android-run`, `ios-sim`, `ios-device`, `ios-archive`, `ios-upload`, `icon`. `.env` fills in the variables that the environment does not set.
- `tools/icon.py` writes the app icon of both projects from `art-src/icon/icon.png`.

## Verification
`just ios-sim --features dev_tools` builds, installs and starts the game on the simulator with no window, and a screenshot shows the run. `just android --features game/dev_tools` makes the APK; the emulator (no window) installs and starts it, and a screenshot shows the run.

## Notes
- 2026-11-10: iOS simulator (iPhone 16): gameplay scene on screen, the HUD under the notch; the built app's `Info.plist` has the name `New Game`, the id `com.example.newgame`, version 0.1.0, build 100.
- 2026-11-10: Android emulator (`Medium_Phone_API_36`, serial `emulator-5554`): gameplay scene on screen; the package has versionName 0.1.0 and versionCode 100. The emulator renders in software: an update there takes 20 to 60 ms, which says nothing about a phone.
- 2026-11-10: not checked here: a build on a phone, the release bundle, the archive and the upload. They need the owner's signing team (`IOS_TEAM`) and upload key (`ANDROID_UPLOAD_KEYSTORE`, `ANDROID_UPLOAD_PASSWORD`).
- 2026-11-10: the Xcode project holds no bundle id and no team. Build with `just`: a build from Xcode's window has no id.
- 2026-11-10: the save's folder on a desktop is the app id, so two games in development do not share a save.
