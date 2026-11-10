mod asset_tracking;
mod audio;
mod canvas;
mod demo;
#[cfg(feature = "dev_tools")]
mod dev_tools;
mod haptics;
mod input;
mod platform;
mod rng;
mod save;
mod screens;
mod share;
mod strings;
mod theme;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*, window::WindowResolution};

use screens::Pause;

/// A phone in portrait, in logical pixels: the desktop window stands in for one.
const PHONE: UVec2 = UVec2::new(390, 844);

/// The game app, shared by the desktop binary and the mobile library.
pub fn app() -> App {
    let mut app = App::new();
    let plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: strings::strings().game_name.into(),
            resolution: WindowResolution::new(PHONE.x, PHONE.y),
            // A sized window on iOS draws a surface larger than the screen.
            #[cfg(target_os = "ios")]
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            // Swipes in from the screen edges reach the game first.
            #[cfg(target_os = "ios")]
            preferred_screen_edges_deferring_system_gestures: bevy::window::ScreenEdge::All,
            #[cfg(target_os = "ios")]
            prefers_status_bar_hidden: true,
            #[cfg(target_os = "ios")]
            prefers_home_indicator_hidden: true,
            ..default()
        }),
        ..default()
    });
    // The Android emulator's Vulkan driver crashes on debug object names.
    #[cfg(target_os = "android")]
    let plugins = {
        use bevy::render::settings::{InstanceFlags, WgpuSettings};
        let mut wgpu = WgpuSettings::default();
        wgpu.instance_flags.remove(InstanceFlags::DEBUG);
        plugins.set(bevy::render::RenderPlugin {
            render_creation: wgpu.into(),
            ..default()
        })
    };
    // winit's iOS event loop warns about three times a frame, also in release builds.
    #[cfg(target_os = "ios")]
    let plugins = plugins.set(bevy::log::LogPlugin {
        filter: format!(
            "{},winit::platform_impl::ios::app_state=error",
            bevy::log::DEFAULT_FILTER
        ),
        ..default()
    });
    // A shot run draws into an image and opens no window; a replay run draws nothing, so
    // it needs no GPU and CI runs it.
    #[cfg(feature = "dev_tools")]
    let plugins = if dev_tools::is_replay_run() {
        plugins
            .disable::<bevy::winit::WinitPlugin>()
            .set(bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::WgpuSettings {
                    backends: None,
                    ..default()
                }
                .into(),
                ..default()
            })
    } else if dev_tools::is_shot_run() {
        plugins.disable::<bevy::winit::WinitPlugin>()
    } else {
        plugins
    };
    app.add_plugins(plugins);
    app.insert_resource(ClearColor(theme::palette::BACKGROUND));
    for schedule in [Update.intern(), FixedUpdate.intern()] {
        app.configure_sets(
            schedule,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );
        app.configure_sets(schedule, PausableSystems.run_if(in_state(Pause::Running)));
    }
    app.add_plugins((
        asset_tracking::plugin,
        audio::plugin,
        canvas::plugin,
        demo::plugin,
        haptics::plugin,
        input::plugin,
        platform::plugin,
        rng::plugin,
        save::plugin,
        share::plugin,
        theme::plugin,
        screens::plugin,
    ));
    #[cfg(feature = "dev_tools")]
    app.add_plugins(dev_tools::plugin);
    app
}

/// The order of the app's systems in `Update` and in `FixedUpdate`: timers tick, then input
/// is read, then the rest reacts to it in the same step. Gameplay runs in `FixedUpdate`;
/// `Update` keeps what only draws.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppSystems {
    TickTimers,
    RecordInput,
    Update,
}

/// Gameplay systems: they run only while a run plays (`Pause::Running`), so they stop while
/// paused and outside `Screen::Gameplay`.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct PausableSystems;
