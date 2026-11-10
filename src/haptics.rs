//! Touch feedback on phones: gameplay writes a `Haptic`, and
//! `play_haptics` plays the strongest of a frame. iOS uses the UIKit feedback generators;
//! Android calls `MainActivity.haptic` over JNI, which runs the vibrator. The desktop only
//! logs.

use bevy::{ecs::system::NonSendMarker, prelude::*};

use crate::save::Save;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<Haptic>()
        .add_systems(Update, play_haptics);
}

/// A tick for a press, a pulse for a point, a rumble for a hit. A stronger kind sorts
/// later.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Haptic {
    Tick,
    Pulse,
    Rumble,
}

/// The strongest of a frame plays, so the steps of one frame buzz once. On the main
/// thread: UIKit only works there. A setting turns them off.
fn play_haptics(_: NonSendMarker, mut haptics: MessageReader<Haptic>, save: Res<Save>) {
    let strongest = haptics.read().copied().max();
    let Some(kind) = strongest.filter(|_| save.settings.haptics) else {
        return;
    };
    info!("haptic {kind:?}");
    play(kind);
}

#[cfg(target_os = "android")]
fn play(kind: Haptic) {
    use jni::{
        JavaVM,
        objects::{JObject, JValue},
    };

    let Some(app) = bevy::android::ANDROID_APP.get() else {
        return;
    };
    let result = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) }.and_then(|vm| {
        let mut env = vm.attach_current_thread_permanently()?;
        let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };
        let called = env.call_method(&activity, "haptic", "(I)V", &[JValue::Int(kind as i32)]);
        if called.is_err() {
            env.exception_clear()?;
        }
        called.map(drop)
    });
    if let Err(err) = result {
        warn!("haptic {kind:?} failed: {err}");
    }
}

#[cfg(target_os = "ios")]
fn play(kind: Haptic) {
    use objc2_foundation::MainThreadMarker;
    use objc2_ui_kit::{
        UIImpactFeedbackGenerator, UINotificationFeedbackGenerator, UINotificationFeedbackType,
        UISelectionFeedbackGenerator,
    };

    let Some(mtm) = MainThreadMarker::new() else {
        warn!("haptic {kind:?}: not on the main thread");
        return;
    };
    // A new impact generator has the medium style; the error notification taps three times,
    // where Core Haptics cannot buzz.
    unsafe {
        match kind {
            Haptic::Tick => UISelectionFeedbackGenerator::new(mtm).selectionChanged(),
            Haptic::Pulse => UIImpactFeedbackGenerator::new(mtm).impactOccurred(),
            Haptic::Rumble => {
                if let Err(err) = rumble() {
                    info!("haptic Rumble: three taps ({err})");
                    UINotificationFeedbackGenerator::new(mtm)
                        .notificationOccurred(UINotificationFeedbackType::Error);
                }
            }
        }
    }
}

/// One 400 ms buzz, as Android's `createOneShot(400, ..)`.
#[cfg(target_os = "ios")]
const RUMBLE_SECS: f64 = 0.4;

/// The Rumble through Core Haptics. One engine for the app's life, made at the first Rumble;
/// none where the hardware has no Core Haptics (the simulator). Each Rumble starts the
/// engine again, because iOS stops it when the app goes to the background.
#[cfg(target_os = "ios")]
fn rumble() -> Result<(), String> {
    use std::cell::OnceCell;

    use objc2_06::{AnyThread, rc::Retained};
    use objc2_core_haptics::{
        CHHapticDeviceCapability, CHHapticEngine, CHHapticEvent, CHHapticEventParameter,
        CHHapticEventParameterIDHapticIntensity, CHHapticEventParameterIDHapticSharpness,
        CHHapticEventTypeHapticContinuous, CHHapticPattern, CHHapticPatternPlayer,
        CHHapticTimeImmediate,
    };
    use objc2_foundation_03::{NSArray, NSError};

    thread_local! {
        static ENGINE: OnceCell<Option<Retained<CHHapticEngine>>> = const { OnceCell::new() };
    }
    let text = |err: Retained<NSError>| err.localizedDescription().to_string();
    ENGINE.with(|cell| unsafe {
        let engine = cell.get_or_init(|| {
            CHHapticEngine::capabilitiesForHardware()
                .supportsHaptics()
                .then(|| CHHapticEngine::initAndReturnError(CHHapticEngine::alloc()).ok())
                .flatten()
        });
        let engine = engine.as_ref().ok_or("no Core Haptics")?;
        engine.startAndReturnError().map_err(text)?;
        let parameter = |id, value| {
            CHHapticEventParameter::initWithParameterID_value(
                CHHapticEventParameter::alloc(),
                id,
                value,
            )
        };
        // Full strength, low sharpness: a dull rumble, not a click.
        let parameters = NSArray::from_retained_slice(&[
            parameter(CHHapticEventParameterIDHapticIntensity, 1.0),
            parameter(CHHapticEventParameterIDHapticSharpness, 0.3),
        ]);
        let event = CHHapticEvent::initWithEventType_parameters_relativeTime_duration(
            CHHapticEvent::alloc(),
            CHHapticEventTypeHapticContinuous,
            &parameters,
            0.0,
            RUMBLE_SECS,
        );
        let pattern = CHHapticPattern::initWithEvents_parameters_error(
            CHHapticPattern::alloc(),
            &NSArray::from_retained_slice(&[event]),
            &NSArray::new(),
        )
        .map_err(text)?;
        let player = engine
            .createPlayerWithPattern_error(&pattern)
            .map_err(text)?;
        player
            .startAtTime_error(CHHapticTimeImmediate)
            .map_err(text)
    })
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn play(_: Haptic) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stronger_kind_wins_the_frame() {
        let frame = [Haptic::Pulse, Haptic::Rumble, Haptic::Tick, Haptic::Pulse];
        assert_eq!(frame.into_iter().max(), Some(Haptic::Rumble));
        assert_eq!(
            [Haptic::Tick, Haptic::Pulse].into_iter().max(),
            Some(Haptic::Pulse)
        );
    }
}
