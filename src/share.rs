//! The share sheet: a text and a file, over JNI to
//! `MainActivity` on Android and with UIKit on iOS. The desktop only logs the file's path.
//! UIKit works only on the main thread, so `run` (`NonSendMarker`) does the work.

use std::path::{Path, PathBuf};

use bevy::{ecs::system::NonSendMarker, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Request>()
        .add_observer(|share: On<Share>, mut request: ResMut<Request>| {
            request.0 = Some((share.text.clone(), share.file.clone()));
        })
        .add_systems(Update, run);
}

/// Shares a text and a file through the share sheet.
#[derive(Event)]
pub struct Share {
    pub text: String,
    pub file: PathBuf,
}

#[derive(Resource, Default)]
struct Request(Option<(String, PathBuf)>);

fn run(_: NonSendMarker, mut request: ResMut<Request>) {
    if let Some((text, file)) = request.0.take() {
        share(&text, &file);
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn share(_: &str, file: &Path) {
    info!("share: {}", file.display());
}

/// The file must lie in what `MainActivity`'s `FileProvider` serves
/// (`res/xml/share_paths.xml`).
#[cfg(target_os = "android")]
fn share(text: &str, file: &Path) {
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
        let text = env.new_string(text)?;
        let file = env.new_string(file.to_string_lossy())?;
        let called = env.call_method(
            &activity,
            "share",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[JValue::Object(&text), JValue::Object(&file)],
        );
        if called.is_err() {
            env.exception_clear()?;
        }
        called.map(drop)
    });
    if let Err(err) = result {
        warn!("share failed: {err}");
    }
}

#[cfg(target_os = "ios")]
fn share(text: &str, file: &Path) {
    use objc2::rc::Retained;
    use objc2_foundation::{MainThreadMarker, NSArray, NSString, NSURL};
    use objc2_ui_kit::{UIActivityViewController, UIApplication};

    let Some(mtm) = MainThreadMarker::new() else {
        warn!("share: not on the main thread");
        return;
    };
    // `keyWindow` is deprecated for apps with many scenes; the game has one.
    #[allow(deprecated)]
    let root = unsafe { UIApplication::sharedApplication(mtm).keyWindow() }
        .and_then(|window| window.rootViewController());
    let Some(root) = root else {
        warn!("share: no root view controller");
        return;
    };
    let path = NSString::from_str(&file.to_string_lossy());
    let url = unsafe { NSURL::fileURLWithPath(&path) };
    let items = NSArray::from_vec(vec![
        Retained::into_super(Retained::into_super(NSString::from_str(text))),
        Retained::into_super(Retained::into_super(url)),
    ]);
    unsafe {
        let sheet = UIActivityViewController::initWithActivityItems_applicationActivities(
            mtm.alloc(),
            &items,
            None,
        );
        // An iPad shows the sheet as a popover, which needs a source view.
        if let Some(popover) = sheet.popoverPresentationController() {
            popover.setSourceView(root.view().as_deref());
        }
        root.presentViewController_animated_completion(&sheet, true, None);
    }
}
