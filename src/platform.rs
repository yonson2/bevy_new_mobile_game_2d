//! The phone platform: the safe area, the pause when the app leaves the
//! foreground, and Android Back. `app()` hides the iOS status bar and home indicator and
//! defers the system edge gestures there; `MainActivity.java` hides the Android bars
//! (sticky immersive), keeps the side edges' swipes for the game, and reports the insets
//! and Back.

use bevy::{
    prelude::*,
    window::{AppLifecycle, WindowFocused},
};

use crate::screens::Pause;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SafeArea>();
    app.add_systems(
        Update,
        (
            pad_to_safe_area,
            pad_to_safe_top,
            pause_on_leave.run_if(in_state(Pause::Running)),
        ),
    );
    #[cfg(target_os = "ios")]
    app.add_systems(PreUpdate, ios::read_safe_area);
    #[cfg(target_os = "android")]
    app.add_plugins(android::plugin);
}

/// The insets of the screen's safe area, in logical pixels: the notch or the camera
/// cutout, the status bar, and the home indicator or the navigation bar lie outside it.
/// Zero on the desktop.
#[derive(Resource, Clone, Copy, PartialEq, Debug, Default)]
pub struct SafeArea {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// A UI root whose content stays inside the safe area: its padding is the insets, and its
/// background still covers the whole screen.
#[derive(Component, Default, Clone)]
pub struct InSafeArea;

/// A root that reaches the screen's top and sides, its content in the safe area: a band
/// behind the status bar.
#[derive(Component, Default, Clone)]
pub struct InSafeTop;

fn pad_to_safe_top(safe: Res<SafeArea>, mut roots: Query<(Ref<InSafeTop>, &mut Node)>) {
    for (root, mut node) in &mut roots {
        if safe.is_changed() || root.is_added() {
            node.padding = UiRect {
                left: px(safe.left),
                right: px(safe.right),
                top: px(safe.top),
                bottom: px(0),
            };
        }
    }
}

fn pad_to_safe_area(safe: Res<SafeArea>, mut roots: Query<(Ref<InSafeArea>, &mut Node)>) {
    for (root, mut node) in &mut roots {
        if safe.is_changed() || root.is_added() {
            node.padding = UiRect {
                left: px(safe.left),
                right: px(safe.right),
                top: px(safe.top),
                bottom: px(safe.bottom),
            };
        }
    }
}

/// A run pauses when the app leaves the foreground or the window loses the focus, and it
/// comes back paused.
fn pause_on_leave(
    mut lifecycle: MessageReader<AppLifecycle>,
    mut focus: MessageReader<WindowFocused>,
    mut next_pause: ResMut<NextState<Pause>>,
) {
    let left = lifecycle
        .read()
        .any(|e| matches!(e, AppLifecycle::WillSuspend | AppLifecycle::Suspended));
    if left || focus.read().any(|e| !e.focused) {
        next_pause.set(Pause::Paused);
    }
}

#[cfg(target_os = "ios")]
mod ios {
    use bevy::{
        ecs::system::NonSendMarker, prelude::*, window::PrimaryWindow, winit::WINIT_WINDOWS,
    };

    use super::SafeArea;

    /// winit gives the iOS safe area as the window's inner position and size, and the
    /// screen as its outer ones.
    pub(super) fn read_safe_area(
        _: NonSendMarker,
        window: Single<Entity, With<PrimaryWindow>>,
        mut safe: ResMut<SafeArea>,
    ) {
        WINIT_WINDOWS.with_borrow(|windows| {
            let Some(window) = windows.get_window(*window) else {
                return;
            };
            let (Ok(inner), Ok(outer)) = (window.inner_position(), window.outer_position()) else {
                return;
            };
            let (inner_size, outer_size) = (window.inner_size(), window.outer_size());
            let scale = window.scale_factor() as f32;
            let (left, top) = ((inner.x - outer.x) as f32, (inner.y - outer.y) as f32);
            safe.set_if_neq(SafeArea {
                top: top / scale,
                right: (outer_size.width as f32 - inner_size.width as f32 - left) / scale,
                bottom: (outer_size.height as f32 - inner_size.height as f32 - top) / scale,
                left: left / scale,
            });
        });
    }
}

/// `MainActivity` calls these on the Java UI thread over JNI; the game reads what they
/// store once a frame.
#[cfg(target_os = "android")]
mod android {
    use std::{
        ffi::c_void,
        sync::atomic::{AtomicI32, AtomicU8, Ordering::SeqCst},
    };

    use bevy::{prelude::*, window::PrimaryWindow};

    use super::SafeArea;
    use crate::screens::Pause;

    pub(super) fn plugin(app: &mut App) {
        app.add_systems(PreUpdate, read_safe_area)
            .add_systems(OnEnter(Pause::Running), || BACK.store(RUNNING, SeqCst))
            .add_systems(OnExit(Pause::Running), || BACK.store(AWAY, SeqCst))
            .add_systems(Update, pause_on_back.run_if(in_state(Pause::Running)));
    }

    /// The insets in physical pixels: left, top, right, bottom.
    static INSETS: [AtomicI32; 4] = [const { AtomicI32::new(0) }; 4];

    /// `MainActivity.onSafeArea`.
    #[unsafe(export_name = "Java_mobile_game_MainActivity_onSafeArea")]
    extern "C" fn on_safe_area(
        _env: *mut c_void,
        _class: *mut c_void,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    ) {
        for (inset, value) in INSETS.iter().zip([left, top, right, bottom]) {
            inset.store(value, SeqCst);
        }
    }

    fn read_safe_area(window: Single<&Window, With<PrimaryWindow>>, mut safe: ResMut<SafeArea>) {
        let [left, top, right, bottom] = INSETS
            .each_ref()
            .map(|i| i.load(SeqCst) as f32 / window.scale_factor());
        safe.set_if_neq(SafeArea {
            top,
            right,
            bottom,
            left,
        });
    }

    const AWAY: u8 = 0;
    const RUNNING: u8 = 1;
    const PRESSED: u8 = 2;

    /// Back in play: the Java UI thread and the game both write it.
    static BACK: AtomicU8 = AtomicU8::new(AWAY);

    /// `MainActivity.onBackInPlay`: true when the game takes Back to pause a running game.
    #[unsafe(export_name = "Java_mobile_game_MainActivity_onBackInPlay")]
    extern "C" fn on_back_in_play(_env: *mut c_void, _class: *mut c_void) -> u8 {
        let taken = BACK.compare_exchange(RUNNING, PRESSED, SeqCst, SeqCst);
        matches!(taken, Ok(_) | Err(PRESSED)).into()
    }

    fn pause_on_back(mut next_pause: ResMut<NextState<Pause>>) {
        if BACK.load(SeqCst) == PRESSED {
            next_pause.set(Pause::Paused);
        }
    }
}
