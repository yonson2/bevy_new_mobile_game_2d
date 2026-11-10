# `.env` fills in the variables that the environment does not set (`.env.example`).
set dotenv-load

default: dev

# Run with the dev tools and Bevy linked dynamically.
dev:
    cargo run --features dev

# A headless shot of a scene, then exit: no window, a fixed time step, seed 1. The scene
# comes from GAME_SHOT_SCENE (default title; `docs/dev-harness.md` lists the scenes).
shot path="target/shots/title.png":
    mkdir -p "$(dirname '{{path}}')"
    GAME_SEED=1 GAME_SHOT='{{path}}' cargo run --features dev_tools

# The replay check: the scripted run, seeds 1, 2 and 3 at 30, 60 and 120 fps, gives the
# hashes in tools/replay-hashes.txt. CI runs it.
replay:
    tools/replay.sh

# Every check that CI runs. Run it before a commit.
check:
    cargo fmt --check
    tools/check-repo.sh
    cargo clippy --all-targets --features dev_tools -- -D warnings
    cargo clippy --all-targets -- -D warnings
    cargo test
    tools/replay.sh

# The issue tracker: `just issue new "Title" -m M1`, `just issue start 12`,
# `just issue close 12`, `just issue check`.
issue *args:
    python3 tools/issues.py {{args}}

# The app's identity and version: `Cargo.toml` is the one place (`docs/mobile.md`).
app_name := `sed -n '/^\[package.metadata.app\]/,/^\[/s/^name = "\(.*\)"$/\1/p' Cargo.toml`
app_id := `sed -n '/^\[package.metadata.app\]/,/^\[/s/^id = "\(.*\)"$/\1/p' Cargo.toml`
version := `grep -m1 '^version = ' Cargo.toml | cut -d'"' -f2`
# major * 10000 + minor * 100 + patch, on iOS and on Android.
build_number := `grep -m1 '^version = ' Cargo.toml | cut -d'"' -f2 | awk -F. '{print $1 * 10000 + $2 * 100 + $3}'`

# The app icon in both phone projects, from art-src/icon/icon.png.
icon *args:
    python3 tools/icon.py {{args}}

android_sdk := env_var_or_default("ANDROID_HOME", home_directory() / "Library/Android/sdk")
# Gradle runs on Android Studio's JDK; a newer system JDK can be too new for it.
android_jdk := env_var_or_default("JAVA_HOME", "/Applications/Android Studio.app/Contents/jbr/Contents/Home")
android_dir := "mobile/android"
adb := android_sdk / "platform-tools/adb"

# Android debug APK: the Rust library via cargo-ndk into jniLibs, then Gradle. Extra
# arguments go to cargo, for example `--features game/dev_tools`. The old APK goes first:
# Gradle's incremental packaging keeps the replaced library's space, and the APK grows.
android *cargo_args:
    rm -f {{android_dir}}/app/build/outputs/apk/debug/app-debug.apk
    ANDROID_HOME='{{android_sdk}}' cargo ndk -t arm64-v8a --platform 31 -o {{android_dir}}/app/src/main/jniLibs build --package mobile {{cargo_args}}
    cd {{android_dir}} && ANDROID_HOME='{{android_sdk}}' JAVA_HOME='{{android_jdk}}' ./gradlew --quiet assembleDebug

# Android release bundle for Play:
# mobile/android/app/build/outputs/bundle/release/app-release.aab, signed with the upload
# key (ANDROID_UPLOAD_KEYSTORE, ANDROID_UPLOAD_PASSWORD). cargo-ndk leaves the library
# unstripped: Gradle strips it and keeps its symbols for Play's crash reports.
android-release *cargo_args:
    ANDROID_HOME='{{android_sdk}}' cargo ndk -t arm64-v8a --platform 31 --no-strip -o {{android_dir}}/app/src/main/jniLibs build --release --package mobile {{cargo_args}}
    cd {{android_dir}} && ANDROID_HOME='{{android_sdk}}' JAVA_HOME='{{android_jdk}}' ./gradlew --quiet bundleRelease

# Install the debug APK on the device ANDROID_SERIAL names (or the only one) and start it.
android-run *cargo_args: (android cargo_args)
    '{{adb}}' install -r {{android_dir}}/app/build/outputs/apk/debug/app-debug.apk
    '{{adb}}' shell am start -n {{app_id}}/mobile.game.MainActivity

ios_dir := "mobile/ios"
ios_sim := env_var_or_default("IOS_SIM", "iPhone 16")
# The Apple developer team that signs device and release builds.
ios_team := env_var_or_default("IOS_TEAM", "")
ios_build := "xcodebuild -quiet -project " + ios_dir + "/Game.xcodeproj -scheme Game -configuration Debug -derivedDataPath target/ios APP_NAME=" + quote(app_name) + " PRODUCT_BUNDLE_IDENTIFIER=" + app_id + " MARKETING_VERSION=" + version + " CURRENT_PROJECT_VERSION=" + build_number
ios_signed := ios_build + " DEVELOPMENT_TEAM=" + ios_team + " -allowProvisioningUpdates"

# Build, install and start on the iOS simulator `IOS_SIM` (default iPhone 16), no window.
# Extra arguments go to cargo, for example `--features dev_tools`.
ios-sim *cargo_args:
    #!/usr/bin/env bash
    set -euo pipefail
    id=$(xcrun simctl list devices available | grep -F "    {{ios_sim}} (" | tail -1 | grep -Eo '[0-9A-F-]{36}')
    {{ios_build}} CARGO_ARGS='{{cargo_args}}' -destination "id=$id"
    xcrun simctl boot "$id" 2>/dev/null || true
    xcrun simctl install "$id" target/ios/Build/Products/Debug-iphonesimulator/Game.app
    xcrun simctl launch "$id" {{app_id}}

# Build (signed by the team IOS_TEAM), install and start on iPhone `IOS_DEVICE` (`xcrun devicectl list devices`).
ios-device *cargo_args:
    #!/usr/bin/env bash
    set -euo pipefail
    : "${IOS_DEVICE:?set IOS_DEVICE (xcrun devicectl list devices)}"
    {{ios_signed}} CARGO_ARGS='{{cargo_args}}' -destination generic/platform=iOS
    xcrun devicectl device install app --device "$IOS_DEVICE" target/ios/Build/Products/Debug-iphoneos/Game.app
    xcrun devicectl device process launch --device "$IOS_DEVICE" {{app_id}}

# Release archive for the App Store: target/ios/Game.xcarchive, signed by the team IOS_TEAM.
ios-archive *cargo_args:
    {{replace(ios_signed, "Debug", "Release")}} CARGO_ARGS='{{cargo_args}}' archive -destination generic/platform=iOS -archivePath target/ios/Game.xcarchive

# Upload the archive to App Store Connect (TestFlight) with the Apple account in Xcode.
# /usr/bin first: the export runs /usr/bin/rsync, which fails if it finds Homebrew's rsync.
ios-upload *cargo_args: (ios-archive cargo_args)
    PATH="/usr/bin:$PATH" xcodebuild -quiet -exportArchive -archivePath target/ios/Game.xcarchive -exportOptionsPlist {{ios_dir}}/ExportOptions.plist -exportPath target/ios/export -allowProvisioningUpdates
