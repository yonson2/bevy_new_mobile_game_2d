# Phones

## Identity: one place

```toml
# Cargo.toml
[package]
version = "0.1.0"

[package.metadata.app]
name = "New Game"            # under the icon and on the title screen
id = "com.example.newgame"   # the bundle id (iOS) and the application id (Android)
```

- `build.rs` gives them to the code as `APP_NAME` and `APP_ID` (the title, the desktop
  save folder).
- `mobile/android/app/build.gradle` reads the same lines. The Java package stays
  `mobile.game` for every game; only the application id changes.
- The `justfile` passes them to Xcode as build settings. The Xcode project holds no id
  and no team, so **build with `just`**, not from Xcode's window.
- The version is the `Cargo.toml` version on both. The build number is
  major x 10000 + minor x 100 + patch.
- `just issue close N` raises the version (`issues/README.md`, "Rules"): do not edit it
  by hand. 1.0.0 is the store release. A milestone with 100 issues or more would repeat
  a build number: split it.

Do not rename the package `game` or the library `mobile`: scripts and JNI names use them.

## iOS

```sh
just ios-sim                          # build, install, start on the simulator; no window
just ios-sim --features dev_tools
IOS_SIM="iPhone 16 Pro" just ios-sim
IOS_DEVICE=<id> just ios-device       # a phone; signed by IOS_TEAM
just ios-archive && just ios-upload   # the App Store archive, then TestFlight
```

- `IOS_TEAM` (the Apple developer team id) is in `.env` or the environment.
- A screenshot: `xcrun simctl io <id> screenshot file.png`. Name the simulator's id:
  several can be booted. Stop the app: `xcrun simctl terminate <id> <app id>`.
- A dev tools variable: `SIMCTL_CHILD_GAME_SCENE=gameplay just ios-sim --features dev_tools`.
- The log: `xcrun simctl spawn <id> log show --last 1m --style compact --predicate
  'process == "Game"'`.
- `mobile/ios/ios-src/PrivacyInfo.xcprivacy` declares the APIs the app uses. Update it
  when the game adds a tracker, an ad SDK or a new API category.

## Android

```sh
just android                          # the debug APK
just android --features game/dev_tools
just android-run                      # install and start on ANDROID_SERIAL, or the only device
just android-release                  # the signed bundle for Play
```

- An agent uses the emulator only, with no window:
  `~/Library/Android/sdk/emulator/emulator -avd <name> -no-window -no-audio`, then
  `ANDROID_SERIAL=emulator-5554` for every `adb` and `just android-run`. The owner's phone
  can be paired to the same adb: never send it a command unless the owner asks.
- A screenshot: `adb exec-out screencap -p > file.png`.
- A dev tools variable: `adb shell am start -S -n <app id>/mobile.game.MainActivity -e
  GAME_SCENE gameplay`.
- The log: `adb logcat -d | grep RustStdoutStderr`.
- The release bundle is signed with the upload key: `ANDROID_UPLOAD_KEYSTORE` (the
  keystore's path) and `ANDROID_UPLOAD_PASSWORD`, key alias `upload`.
- `gamesActivity` in `mobile/android/gradle/libs.versions.toml` must match the
  `android-activity` crate in `Cargo.lock`. Check it at each Bevy upgrade.
- `JAVA_HOME` defaults to Android Studio's JDK, `ANDROID_HOME` to
  `~/Library/Android/sdk`.

## The icon

`just icon` (`tools/icon.py`) writes the iOS icon and Android's adaptive icon layers from
`art-src/icon/icon.png`: the subject on a clear ground. `--pixel` for pixel art,
`--background '#rrggbb'` for the colour behind it.

## What the platform code does

- `src/platform.rs`: the safe area (UIKit on iOS; `MainActivity` sends it on Android),
  pause on suspend, Android's Back (it pauses a run; elsewhere the app goes to the
  background).
- `src/haptics.rs`: Core Haptics and UIKit feedback on iOS; the vibrator on Android.
- `src/share.rs`: the share sheet. Android's `FileProvider` serves the `share/` folder of
  the app's data.
- `MainActivity.java`: full screen, edge swipes go to the game, `GAME_*` extras become
  environment variables for the dev tools.
