//! Dev-only tools, compiled with the `dev_tools` feature. Release builds never contain
//! them. `docs/dev-harness.md` shows how to use them.
//!
//! - Always: a log line for each `Screen` change and for each gesture but the drag deltas;
//!   a warning for each update over 16 ms, `slow update: <update> (frame <frame>)`: the
//!   update is the main schedule, the game's logic without the rendering.
//! - `GAME_SEED=<u64>`: the run seed.
//! - `GAME_STYLE=pixel|smooth`: the canvas style for this run (`canvas::STYLE`).
//! - `GAME_SHOT=<file.png>`: run a scene, save one frame to the file and exit. Each frame
//!   advances time by a fixed step, so a seed gives the same picture. Its sounds play
//!   silent; the log has a line for each (`audio: .. starts`).
//! - `GAME_SHOT_SCENE=<scene>`: with `GAME_SHOT`, the scene (default `title`). A scene
//!   sets its states directly. [`DevScene::from_env`] lists them.
//! - `GAME_SCENE=<scene>`: without `GAME_SHOT`, a run with a window starts in the scene,
//!   then the player plays on.
//! - `GAME_SWIPE=<x0>,<y0>,<x1>,<y1>,<ms>[,hold][;<x0>,..]`: with `GAME_SHOT`, straight
//!   touch swipes in logical pixels from the scene's first frame, one after the other,
//!   through the input layer; the shot waits for their end. A swipe with `hold` keeps the
//!   finger down at its end. Bevy's picking does not see them, so they press no button.
//! - `GAME_SHOT_WAIT=<secs>`: with `GAME_SHOT`, the game seconds after the swipes before
//!   the shot (default 0.5).
//! - `GAME_SHOT_FPS=<n>`: with `GAME_SHOT`, the frames a second (default 60).
//! - `GAME_SHOT_SIZE=<w>x<h>`: with `GAME_SHOT`, the image size in pixels (default 1080 x
//!   2400). The UI keeps the desktop window's width of 390 logical pixels at any size, as
//!   on a phone.
//!
//! A `GAME_SHOT` run opens no window and does not take the focus: `app()` leaves out
//! winit, the schedule runner drives the frames, and the screen camera draws into an
//! image.

use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use bevy::{
    app::ScheduleRunnerPlugin,
    asset::RenderAssetUsages,
    audio::Volume,
    camera::{ImageRenderTarget, RenderTarget},
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    input::touch::{TouchInput, TouchPhase},
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
    },
    sprite::Anchor,
    time::TimeUpdateStrategy,
    window::WindowEvent,
};

use crate::{
    AppSystems, PHONE,
    audio::Sfx,
    canvas::{Canvas, ScreenCamera, Style, WorldCamera, ZOOM},
    haptics::Haptic,
    input::Gesture,
    platform::SafeArea,
    rng::GameRng,
    screens::{Pause, Screen},
    theme::{palette, widget},
};

const FRAME: Duration = Duration::from_nanos(1_000_000_000 / 60);
const SHOT_SIZE: UVec2 = UVec2::new(1080, 2400);
/// Frames in the scene's state before the shot: the UI spawns and lays out.
const SETTLE_FRAMES: u32 = 30;
/// Frames of one canvas size before `GAME_SCENE` enters its scene.
const SETTLED_FRAMES: u32 = 10;
/// Real seconds before a shot run gives up, on top of its swipes and its wait.
const GIVE_UP_SECS: f32 = 20.;
/// Real seconds more for the replay: it plays `REPLAY_STEPS`.
const REPLAY_SECS: f32 = 120.;
/// An update over this misses a 60 Hz frame.
const SLOW_UPDATE: Duration = Duration::from_millis(16);
const BOUNCERS: u32 = 200;
/// The stress scene's seconds before it measures (shader warm-up), and its seconds of
/// measure.
const STRESS_WARM_UP: f32 = 5.;
const STRESS_RUN: f32 = 120.;

pub fn is_shot_run() -> bool {
    std::env::var_os("GAME_SHOT").is_some()
}

/// The `replay` scene: `app()` turns the renderer off for it.
pub fn is_replay_run() -> bool {
    is_shot_run() && std::env::var("GAME_SHOT_SCENE").as_deref() == Ok("replay")
}

/// `GAME_STYLE`: the canvas style of this run.
pub fn style() -> Option<Style> {
    match std::env::var("GAME_STYLE").ok()?.as_str() {
        "pixel" => Some(Style::Pixel),
        "smooth" => Some(Style::Smooth),
        _ => None,
    }
}

pub(super) fn plugin(app: &mut App) {
    // Two FixedUpdate systems that touch the same data in no set order stop the start: a
    // new system could reorder them, and a seed would play another way. The order of the
    // sounds and the haptics in a step changes nothing.
    app.edit_schedule(FixedUpdate, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Error,
            ..default()
        });
    });
    app.allow_ambiguous_resource::<Messages<Sfx>>();
    app.allow_ambiguous_resource::<Messages<Haptic>>();
    app.add_systems(Update, log_screens);
    app.add_systems(
        FixedUpdate,
        log_gestures.in_set(AppSystems::Update).ambiguous_with_all(),
    );
    app.insert_resource(UpdateStart(Instant::now()))
        .add_systems(First, start_update)
        .add_systems(Last, log_slow_update);

    if let Some(seed) = std::env::var("GAME_SEED").ok().and_then(|s| s.parse().ok()) {
        info!("run seed {seed} (GAME_SEED)");
        app.insert_resource(GameRng::new(seed));
    }

    let shot = std::env::var("GAME_SHOT").ok();
    let scene = match shot {
        Some(_) => Some(DevScene::from_env("GAME_SHOT_SCENE").unwrap_or(DevScene::Title)),
        None => DevScene::from_env("GAME_SCENE"),
    };
    let Some(scene) = scene else {
        return;
    };
    app.insert_resource(scene)
        .add_systems(
            Startup,
            spawn_safe_area_frame.run_if(resource_equals(DevScene::SafeArea)),
        )
        .add_systems(
            Update,
            (
                spawn_checker.run_if(resource_equals(DevScene::Canvas)),
                show_insets.run_if(resource_equals(DevScene::SafeArea)),
                play_each_haptic
                    .run_if(resource_equals(DevScene::Haptics))
                    .run_if(in_state(Screen::Title)),
            ),
        )
        .add_systems(
            OnEnter(Screen::Title),
            spawn_bouncers.run_if(resource_equals(DevScene::Stress)),
        )
        .add_systems(
            FixedUpdate,
            bounce.in_set(AppSystems::Update).ambiguous_with_all(),
        )
        .add_systems(
            Last,
            measure_stress
                .run_if(resource_equals(DevScene::Stress))
                .run_if(in_state(Screen::Title)),
        );
    let Some(path) = shot else {
        app.add_systems(Update, enter_scene);
        return;
    };
    // The sounds start, and log it, but nobody hears them.
    app.insert_resource(GlobalVolume::new(Volume::SILENT));
    let size = std::env::var("GAME_SHOT_SIZE")
        .ok()
        .and_then(|s| {
            let (w, h) = s.split_once('x')?;
            Some(UVec2::new(w.parse().ok()?, h.parse().ok()?))
        })
        .unwrap_or(SHOT_SIZE);
    let settle = std::env::var("GAME_SHOT_WAIT")
        .ok()
        .and_then(|s| s.parse::<f32>().ok())
        .map_or(SETTLE_FRAMES, |secs| (secs * 60.).round() as u32);
    let swipes = std::env::var("GAME_SWIPE")
        .ok()
        .map(|s| s.split(';').map_while(parse_swipe).collect())
        .unwrap_or_default();
    // The scale factor keeps the UI at the phone's logical width.
    let target = ImageRenderTarget {
        handle: app
            .world_mut()
            .resource_mut::<Assets<Image>>()
            .add(Image::new_target_texture(
                size.x,
                size.y,
                TextureFormat::Rgba8UnormSrgb,
                None,
            )),
        scale_factor: size.x as f32 / PHONE.x as f32,
    };
    let frame = std::env::var("GAME_SHOT_FPS")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .map_or(FRAME, |fps| Duration::from_secs_f64(1. / fps));
    // A replay runs as fast as it can: its time is the frames', not the clock's.
    let wait = if scene == DevScene::Replay {
        Duration::ZERO
    } else {
        frame
    };
    if scene == DevScene::Replay {
        app.insert_resource(Replay {
            touches: replay_script(),
            step: 0,
        })
        // First in each fixed step: the input layer reads its touches in the step, and
        // its hash sees the state the steps before left.
        .add_systems(
            FixedFirst,
            drive_replay.run_if(in_state(Screen::Gameplay).or_else(in_state(Screen::GameOver))),
        );
    }
    app.add_plugins(ScheduleRunnerPlugin::run_loop(wait))
        .insert_resource(TimeUpdateStrategy::ManualDuration(frame))
        .insert_resource(Shot {
            path: path.into(),
            target,
            swipes,
            settle,
            frames: 0,
            requested: false,
        })
        .add_observer(render_offscreen)
        .add_systems(Update, (drive_shot, play_swipe.after(drive_shot)));
}

fn log_screens(mut transitions: MessageReader<StateTransitionEvent<Screen>>) {
    for transition in transitions.read() {
        info!(
            "Screen: {:?} -> {:?}",
            transition.exited, transition.entered
        );
    }
}

#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
enum DevScene {
    Title,
    Gameplay,
    Pause,
    GameOver,
    Settings,
    /// The title over a 1 px checkerboard that fills the canvas (`Style::Pixel`).
    Canvas,
    /// The title under a frame on the safe area's edges, with the insets.
    SafeArea,
    /// The title, then a tick, a pulse and a rumble, a second apart.
    Haptics,
    /// The title over 200 sprites that bounce; it logs the frame times in one `stress:`
    /// line.
    Stress,
    /// A scripted run without the renderer that logs a hash of its state.
    Replay,
}

impl DevScene {
    fn from_env(var: &str) -> Option<Self> {
        Some(match std::env::var(var).ok()?.as_str() {
            "title" => Self::Title,
            "gameplay" => Self::Gameplay,
            "pause" => Self::Pause,
            "game-over" => Self::GameOver,
            "settings" => Self::Settings,
            "canvas" => Self::Canvas,
            "safe-area" => Self::SafeArea,
            "haptics" => Self::Haptics,
            "stress" => Self::Stress,
            "replay" => Self::Replay,
            _ => return None,
        })
    }
}

/// Moves the states one step toward the scene's; true once they are the scene's.
fn reach(
    scene: DevScene,
    screen: Screen,
    pause: Option<Pause>,
    next_screen: &mut NextState<Screen>,
    next_pause: &mut NextState<Pause>,
) -> bool {
    match (scene, screen, pause) {
        (
            DevScene::Title
            | DevScene::Canvas
            | DevScene::SafeArea
            | DevScene::Haptics
            | DevScene::Stress,
            Screen::Title,
            _,
        )
        | (DevScene::Gameplay | DevScene::Replay, Screen::Gameplay, _)
        | (DevScene::Pause, Screen::Gameplay, Some(Pause::Paused))
        | (DevScene::GameOver, Screen::GameOver, _)
        | (DevScene::Settings, Screen::Settings, _) => true,
        (DevScene::Settings, Screen::Title, _) => {
            next_screen.set(Screen::Settings);
            false
        }
        (DevScene::GameOver, Screen::Title, _) => {
            next_screen.set(Screen::GameOver);
            false
        }
        (DevScene::Gameplay | DevScene::Pause | DevScene::Replay, Screen::Title, _) => {
            next_screen.set(Screen::Gameplay);
            false
        }
        (DevScene::Pause, Screen::Gameplay, Some(Pause::Running)) => {
            next_pause.set(Pause::Paused);
            false
        }
        _ => false,
    }
}

/// `GAME_SCENE`: moves to the scene's states once, then leaves the game to the player. It
/// waits until the canvas has kept its size for `SETTLED_FRAMES`: a phone's window has the
/// asked size until the phone reports its own.
fn enter_scene(
    scene: Res<DevScene>,
    screen: Res<State<Screen>>,
    pause: Option<Res<State<Pause>>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_pause: ResMut<NextState<Pause>>,
    canvas: Res<Canvas>,
    mut settled: Local<u32>,
    mut there: Local<bool>,
) {
    *settled = if canvas.is_changed() { 0 } else { *settled + 1 };
    if !*there && *settled >= SETTLED_FRAMES {
        *there = reach(
            *scene,
            *screen.get(),
            pause.map(|p| *p.get()),
            &mut next_screen,
            &mut next_pause,
        );
    }
}

#[derive(Resource)]
struct Shot {
    path: PathBuf,
    /// The image the screen camera draws into.
    target: ImageRenderTarget,
    swipes: Vec<ScriptedSwipe>,
    /// Frames after the swipes before the shot.
    settle: u32,
    /// Frames since the scene's state began.
    frames: u32,
    requested: bool,
}

/// One swipe of `GAME_SWIPE`: touch 0 goes down at `from`, moves once a frame, and lifts
/// at `to` `frames` frames later, unless it holds.
#[derive(Clone, Copy)]
struct ScriptedSwipe {
    from: Vec2,
    to: Vec2,
    frames: u32,
    hold: bool,
}

impl ScriptedSwipe {
    /// Its touch events: a press, the moves and a release, unless it holds.
    fn events(&self) -> u32 {
        self.frames + u32::from(!self.hold)
    }
}

/// `<x0>,<y0>,<x1>,<y1>,<ms>[,hold]`
fn parse_swipe(s: &str) -> Option<ScriptedSwipe> {
    let (s, hold) = match s.strip_suffix(",hold") {
        Some(s) => (s, true),
        None => (s, false),
    };
    let (points, ms) = s.rsplit_once(',')?;
    let p: Vec<f32> = points
        .split(',')
        .map(|n| n.parse().ok())
        .collect::<Option<_>>()?;
    let [x0, y0, x1, y1] = p[..] else {
        return None;
    };
    let ms: u32 = ms.parse().ok()?;
    Some(ScriptedSwipe {
        from: Vec2::new(x0, y0),
        to: Vec2::new(x1, y1),
        frames: (ms * 60).div_ceil(1000).max(1),
        hold,
    })
}

/// The screen camera draws into the shot's image.
fn render_offscreen(add: On<Add<ScreenCamera>>, mut commands: Commands, shot: Res<Shot>) {
    commands
        .entity(add.entity)
        .insert((RenderTarget::Image(shot.target.clone()), IsDefaultUiCamera));
}

/// Moves to the scene's states, plays the swipes, waits `settle` frames, then saves the shot
/// and exits.
fn drive_shot(
    mut commands: Commands,
    mut shot: ResMut<Shot>,
    scene: Res<DevScene>,
    screen: Res<State<Screen>>,
    pause: Option<Res<State<Pause>>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_pause: ResMut<NextState<Pause>>,
    mut game_time: ResMut<Time<Virtual>>,
    real_time: Res<Time<Real>>,
    mut exit: MessageWriter<AppExit>,
) {
    let swipes: u32 = shot.swipes.iter().map(ScriptedSwipe::events).sum();
    let replay = *scene == DevScene::Replay;
    let give_up =
        GIVE_UP_SECS + if replay { REPLAY_SECS } else { 0. } + (shot.settle + swipes) as f32 / 60.;
    if real_time.elapsed_secs() > give_up {
        error!("GAME_SHOT: no shot after {give_up} s, giving up");
        exit.write(AppExit::error());
        return;
    }
    // The replay ends itself (`drive_replay`).
    if shot.requested || (replay && shot.frames > 0) {
        return;
    }
    let there = reach(
        *scene,
        *screen.get(),
        pause.map(|p| *p.get()),
        &mut next_screen,
        &mut next_pause,
    );
    if !there {
        return;
    }
    shot.frames += 1;
    if replay || shot.frames < shot.settle + swipes {
        return;
    }
    shot.requested = true;
    // The capture can land a frame or two later: game time stops, so it shows this frame.
    game_time.pause();
    info!("GAME_SHOT: saving {}", shot.path.display());
    commands
        // Not `Screenshot::image`: it takes scale factor 1, another target than the
        // camera's, and saves a black image.
        .spawn(Screenshot(RenderTarget::Image(shot.target.clone())))
        .observe(save_to_disk(shot.path.clone()))
        .observe(
            |_: On<ScreenshotCaptured>, mut exit: MessageWriter<AppExit>| {
                exit.write(AppExit::Success);
            },
        );
}

/// A 1 px checkerboard over the whole canvas, white and black: each canvas pixel shows on
/// the shot as one square of the scale's size.
fn spawn_checker(mut commands: Commands, canvas: Res<Canvas>, mut images: ResMut<Assets<Image>>) {
    let size = canvas.fit.size.as_uvec2();
    if !canvas.is_changed() || size.min_element() == 0 {
        return;
    }
    let data = (0..size.y)
        .flat_map(|y| (0..size.x).map(move |x| x + y))
        .flat_map(|n| if n % 2 == 0 { [255; 4] } else { [0, 0, 0, 255] })
        .collect();
    let image = images.add(Image::new(
        Extent3d {
            width: size.x,
            height: size.y,
            ..default()
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    ));
    let custom_size = Some(size.as_vec2() / ZOOM);
    commands.spawn_scene(bsn! {
        #Checker
        Sprite { image: {image}, custom_size: {custom_size} }
        Anchor::BOTTOM_LEFT
    });
}

/// Plays `GAME_SWIPE` into the window events that the input layer reads, one touch event a
/// frame from the scene's first frame, one swipe after the other.
fn play_swipe(
    shot: Res<Shot>,
    mut events: MessageWriter<WindowEvent>,
    mut at: Local<(usize, u32)>,
) {
    let (index, played) = &mut *at;
    if shot.frames == 0 {
        return;
    }
    let Some(swipe) = shot.swipes.get(*index) else {
        return;
    };
    let phase = match *played {
        0 => TouchPhase::Started,
        f if f < swipe.frames => TouchPhase::Moved,
        _ => TouchPhase::Ended,
    };
    events.write(WindowEvent::TouchInput(TouchInput {
        phase,
        position: swipe
            .from
            .lerp(swipe.to, *played as f32 / swipe.frames as f32),
        window: Entity::PLACEHOLDER,
        force: None,
        id: 0,
    }));
    *played += 1;
    if *played == swipe.events() {
        *index += 1;
        *played = 0;
    }
}

/// The replay's length: it logs its state after this many fixed steps from the run's start.
const REPLAY_STEPS: u32 = 1500;

/// The replay's script and its fixed steps so far.
#[derive(Resource)]
struct Replay {
    /// Each touch event: the fixed step it plays in, its phase and where, in logical pixels.
    touches: Vec<(u32, TouchPhase, Vec2)>,
    step: u32,
}

/// The replay's touches, in fixed steps from the run's first: drags that take the player
/// left, right and up. A game writes its own script, one that reaches each mechanic.
fn replay_script() -> Vec<(u32, TouchPhase, Vec2)> {
    let mut touches = vec![];
    let mut drag = |at: u32, from: Vec2, to: Vec2, steps: u32| {
        for k in 0..=steps {
            let phase = match k {
                0 => TouchPhase::Started,
                k if k == steps => TouchPhase::Ended,
                _ => TouchPhase::Moved,
            };
            touches.push((at + k, phase, from.lerp(to, k as f32 / steps as f32)));
        }
    };
    drag(60, Vec2::new(195., 600.), Vec2::new(60., 600.), 40);
    drag(200, Vec2::new(60., 600.), Vec2::new(330., 560.), 80);
    drag(420, Vec2::new(330., 560.), Vec2::new(195., 300.), 30);
    drag(700, Vec2::new(195., 300.), Vec2::new(100., 640.), 100);
    drag(1000, Vec2::new(100., 640.), Vec2::new(300., 640.), 12);
    touches
}

/// Plays the replay's touches into the window events that the input layer reads, each in
/// its fixed step, so every frame rate gives the same input to the same step. After
/// `REPLAY_STEPS` it logs `GAME_STATE` and exits.
fn drive_replay(world: &mut World) {
    let mut replay = world.resource_mut::<Replay>();
    replay.step += 1;
    let step = replay.step;
    let now: Vec<_> = replay
        .touches
        .iter()
        .filter(|t| t.0 == step)
        .map(|&(_, phase, position)| {
            WindowEvent::TouchInput(TouchInput {
                phase,
                position,
                window: Entity::PLACEHOLDER,
                force: None,
                id: 0,
            })
        })
        .collect();
    for event in now {
        world.write_message(event);
    }
    if step == REPLAY_STEPS + 1 {
        let (hash, summary) = crate::demo::state_hash(world);
        info!("GAME_STATE step {REPLAY_STEPS}: {hash:016x} ({summary})");
        world.write_message(AppExit::Success);
    }
}

fn log_gestures(mut gestures: MessageReader<Gesture>) {
    for gesture in gestures.read() {
        if !matches!(gesture, Gesture::Drag { .. }) {
            info!("gesture: {gesture:?}");
        }
    }
}

#[derive(Component, Default, Clone)]
struct InsetsLabel;

/// The safe area's frame over the title: a border on the safe area's edges and a label in
/// each corner, so a screenshot shows whether the notch or the home indicator covers any.
fn spawn_safe_area_frame(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        @widget::ui_root("Safe Area Frame")
        GlobalZIndex(2)
        Children [
            #Frame
            Node {
                width: percent(100),
                height: percent(100),
                border: {UiRect::all(px(2))},
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
            }
            BorderColor::all(palette::HEADER_TEXT)
            Pickable::IGNORE
            Children [
                Node { flex_direction: FlexDirection::Column }
                Pickable::IGNORE
                Children [
                    Node { justify_content: JustifyContent::SpaceBetween }
                    Pickable::IGNORE
                    Children [ @widget::label("top left") -- @widget::label("top right") ]
                    --
                    @widget::label("")
                    InsetsLabel
                ]
                --
                Node { justify_content: JustifyContent::SpaceBetween }
                Pickable::IGNORE
                Children [ @widget::label("bottom left") -- @widget::label("bottom right") ]
            ]
        ]
    });
}

fn show_insets(safe: Res<SafeArea>, mut label: Single<&mut Text, With<InsetsLabel>>) {
    if safe.is_changed() {
        label.0 = format!(
            "insets {:.0} {:.0} {:.0} {:.0}",
            safe.top, safe.right, safe.bottom, safe.left
        );
    }
}

/// The `haptics` scene: each kind once, a second apart, from the title on.
fn play_each_haptic(
    time: Res<Time>,
    mut since: Local<f32>,
    mut played: Local<usize>,
    mut haptics: MessageWriter<Haptic>,
) {
    const KINDS: [Haptic; 3] = [Haptic::Tick, Haptic::Pulse, Haptic::Rumble];
    *since += time.delta_secs();
    if *played < KINDS.len() && *since >= (*played + 1) as f32 {
        haptics.write(KINDS[*played]);
        *played += 1;
    }
}

/// When the main schedule started this frame.
#[derive(Resource)]
struct UpdateStart(Instant);

fn start_update(mut start: ResMut<UpdateStart>) {
    start.0 = Instant::now();
}

fn log_slow_update(start: Res<UpdateStart>, time: Res<Time<Real>>) {
    let update = start.0.elapsed();
    if update > SLOW_UPDATE {
        warn!("slow update: {update:.1?} (frame {:.1?})", time.delta());
    }
}

/// A stress sprite's velocity, in world units a second.
#[derive(Component, Default, Clone)]
struct Bouncer(Vec2);

/// Far more sprites than the demo shows, spread over the canvas without the RNG. They are
/// the world camera's children, so they stay on the screen if the camera moves.
fn spawn_bouncers(
    mut commands: Commands,
    canvas: Res<Canvas>,
    camera: Single<Entity, With<WorldCamera>>,
) {
    let size = canvas.world_size();
    for i in 0..BOUNCERS {
        let f = i as f32;
        let transform = Transform::from_xyz((f * 37.) % size.x, (f * 53.) % size.y, 0.);
        let bouncer = Bouncer(Vec2::from_angle(f * 2.4) * 40.);
        commands
            .spawn_scene(bsn! {
                #Bouncer
                bouncer
                Sprite { color: {palette::HEADER_TEXT}, custom_size: {Some(Vec2::splat(3.))} }
                transform
                DespawnOnExit<Screen>(Screen::Title)
            })
            .insert(ChildOf(*camera));
    }
}

fn bounce(
    time: Res<Time>,
    canvas: Res<Canvas>,
    mut bouncers: Query<(&mut Bouncer, &mut Transform)>,
) {
    let size = canvas.world_size();
    for (mut bouncer, mut transform) in &mut bouncers {
        let mut at = transform.translation.xy() + bouncer.0 * time.delta_secs();
        if !(0. ..=size.x).contains(&at.x) {
            bouncer.0.x = -bouncer.0.x;
            at.x = at.x.clamp(0., size.x);
        }
        if !(0. ..=size.y).contains(&at.y) {
            bouncer.0.y = -bouncer.0.y;
            at.y = at.y.clamp(0., size.y);
        }
        transform.translation = at.extend(0.);
    }
}

/// From the warm-up's end, keeps each frame time and the longest update; after the run,
/// logs them once.
fn measure_stress(
    start: Res<UpdateStart>,
    time: Res<Time<Real>>,
    mut since: Local<f32>,
    mut frames: Local<Vec<f32>>,
    mut longest: Local<Duration>,
) {
    *since += time.delta_secs();
    if *since < STRESS_WARM_UP {
        return;
    }
    if *since < STRESS_WARM_UP + STRESS_RUN {
        frames.push(time.delta_secs() * 1000.);
        *longest = (*longest).max(start.0.elapsed());
        return;
    }
    if frames.is_empty() {
        return;
    }
    frames.sort_by(f32::total_cmp);
    let at = |q: f32| frames[((frames.len() - 1) as f32 * q) as usize];
    info!(
        "stress: {} frames in {STRESS_RUN} s; frame median {:.1} ms, 99th {:.1} ms, max {:.1} ms; longest update {:.1?}",
        frames.len(),
        at(0.5),
        at(0.99),
        at(1.),
        *longest
    );
    frames.clear();
}
