//! The input layer: every touch, and on the desktop the left mouse button, becomes
//! one pointer stream in arrival order, and recognizers turn the stream into [`Gesture`]s.
//!
//! One finger drives the game: the first one down. The layer ignores the others until it
//! lifts. The layer reads the window events in `FixedUpdate` (`AppSystems::RecordInput`),
//! so each event reaches exactly one fixed step, the first one after it arrives, and a
//! press, its moves and its release in one frame stay apart. Gameplay reads the gestures
//! later in the same step. Positions are window coordinates in logical pixels, y down.

use bevy::{
    input::{ButtonState, touch::TouchPhase},
    prelude::*,
    window::WindowEvent,
};

use crate::{AppSystems, canvas::ScreenCamera, theme::interaction::InteractionPalette};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<Gesture>();
    app.init_resource::<InputLayer>();
    app.add_systems(FixedUpdate, record_input.in_set(AppSystems::RecordInput));
}

/// What the finger did. Velocities are in logical pixels per second.
#[derive(Message, Clone, Copy, PartialEq, Debug)]
pub enum Gesture {
    /// The finger moved past the slop from `at`, where it went down: a drag began.
    DragStart {
        at: Vec2,
    },
    /// The drag moved by `delta`. The deltas of a drag add up to its travel.
    Drag {
        delta: Vec2,
    },
    /// The finger lifted and ended the drag: `travel` from where it went down, and
    /// `velocity` over the drag's last moments.
    DragEnd {
        travel: Vec2,
        velocity: Vec2,
    },
    /// The platform took the finger during a drag (a system gesture): the drag ends with
    /// no other gesture.
    DragCancel,
    /// A drag ended faster than the flick speed. It follows the `DragEnd`.
    Flick {
        velocity: Vec2,
    },
    /// A drag of at least the swipe distance, mostly along one axis. It follows the
    /// `DragEnd`.
    Swipe(Swipe),
    Tap {
        at: Vec2,
    },
    /// A tap soon after a tap and near it, instead of a second `Tap`.
    DoubleTap {
        at: Vec2,
    },
    /// The finger stayed down inside the slop for the long-press time. No `Tap` follows.
    LongPress {
        at: Vec2,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Swipe {
    Left,
    Right,
    Up,
    Down,
}

/// The recognizers' thresholds, in logical pixels and seconds.
#[derive(Clone, Copy, Debug)]
struct Tuning {
    /// How far the finger moves before a press becomes a drag.
    slop: f32,
    /// The longest press that is a tap.
    tap_time: f64,
    /// The longest time from a tap to the next press for a double-tap.
    double_tap_gap: f64,
    /// How near the first tap the second one goes down for a double-tap.
    double_tap_reach: f32,
    long_press_time: f64,
    /// The end of a drag that gives its release velocity.
    flick_window: f64,
    flick_speed: f32,
    swipe_distance: f32,
}

impl Default for Tuning {
    fn default() -> Self {
        Self {
            slop: 8.,
            tap_time: 0.3,
            double_tap_gap: 0.3,
            double_tap_reach: 40.,
            long_press_time: 0.5,
            flick_window: 0.1,
            flick_speed: 300.,
            swipe_distance: 60.,
        }
    }
}

#[derive(Resource, Default)]
struct InputLayer {
    pointer: Pointer,
    recognizer: Recognizer,
    tuning: Tuning,
}

/// One step of the pointer stream.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Step {
    Press(Vec2),
    Move(Vec2),
    Release(Vec2),
    /// The platform took the finger.
    Cancel,
}

/// Turns window events into steps of the one pointer: the first touch down, or the left
/// mouse button while no touch is down. A press on a button is not the pointer's: the
/// button takes it, so its moves and its release give no gesture either.
#[derive(Default)]
struct Pointer {
    touch: Option<u64>,
    mouse_down: bool,
    cursor: Option<Vec2>,
}

impl Pointer {
    fn step(&mut self, event: &WindowEvent, on_button: impl Fn(Vec2) -> bool) -> Option<Step> {
        match event {
            WindowEvent::TouchInput(e) => match e.phase {
                TouchPhase::Started
                    if self.touch.is_none() && !self.mouse_down && !on_button(e.position) =>
                {
                    self.touch = Some(e.id);
                    Some(Step::Press(e.position))
                }
                TouchPhase::Moved if self.touch == Some(e.id) => Some(Step::Move(e.position)),
                TouchPhase::Ended if self.touch == Some(e.id) => {
                    self.touch = None;
                    Some(Step::Release(e.position))
                }
                TouchPhase::Canceled if self.touch == Some(e.id) => {
                    self.touch = None;
                    Some(Step::Cancel)
                }
                _ => None,
            },
            WindowEvent::CursorMoved(e) => {
                self.cursor = Some(e.position);
                self.mouse_down.then_some(Step::Move(e.position))
            }
            WindowEvent::MouseButtonInput(e) if e.button == MouseButton::Left => {
                match (e.state, self.cursor) {
                    (ButtonState::Pressed, Some(p))
                        if !self.mouse_down && self.touch.is_none() && !on_button(p) =>
                    {
                        self.mouse_down = true;
                        Some(Step::Press(p))
                    }
                    (ButtonState::Released, Some(p)) if self.mouse_down => {
                        self.mouse_down = false;
                        Some(Step::Release(p))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

/// The finger while it is down.
struct Finger {
    down: Vec2,
    down_at: f64,
    at: Vec2,
    dragging: bool,
    long_pressed: bool,
    /// Where the finger was in the last `flick_window`, oldest first.
    recent: Vec<(f64, Vec2)>,
}

/// The recognizers, fed the pointer steps with their times in seconds.
#[derive(Default)]
struct Recognizer {
    finger: Option<Finger>,
    /// When and where the last tap lifted, for a double-tap.
    last_tap: Option<(f64, Vec2)>,
}

impl Recognizer {
    fn step(&mut self, tuning: &Tuning, t: f64, step: Step, out: &mut Vec<Gesture>) {
        match step {
            Step::Press(p) => {
                self.finger = Some(Finger {
                    down: p,
                    down_at: t,
                    at: p,
                    dragging: false,
                    long_pressed: false,
                    recent: vec![(t, p)],
                });
            }
            Step::Move(p) => self.move_to(tuning, t, p, out),
            Step::Release(p) => {
                self.move_to(tuning, t, p, out);
                let Some(finger) = self.finger.take() else {
                    return;
                };
                if finger.dragging {
                    release_drag(tuning, &finger, out);
                } else if !finger.long_pressed && t - finger.down_at <= tuning.tap_time {
                    let at = finger.down;
                    match self.last_tap.take() {
                        Some((tapped, near))
                            if finger.down_at - tapped <= tuning.double_tap_gap
                                && at.distance(near) <= tuning.double_tap_reach =>
                        {
                            out.push(Gesture::DoubleTap { at });
                        }
                        _ => {
                            out.push(Gesture::Tap { at });
                            self.last_tap = Some((t, at));
                        }
                    }
                }
            }
            Step::Cancel => {
                if self.finger.take().is_some_and(|f| f.dragging) {
                    out.push(Gesture::DragCancel);
                }
            }
        }
    }

    fn move_to(&mut self, tuning: &Tuning, t: f64, p: Vec2, out: &mut Vec<Gesture>) {
        let Some(finger) = &mut self.finger else {
            return;
        };
        finger.recent.retain(|&(s, _)| t - s <= tuning.flick_window);
        finger.recent.push((t, p));
        if !finger.dragging && p.distance(finger.down) > tuning.slop {
            finger.dragging = true;
            out.push(Gesture::DragStart { at: finger.down });
        }
        // `at` stays at the press until the drag starts, so the first delta covers the slop.
        if finger.dragging && p != finger.at {
            out.push(Gesture::Drag {
                delta: p - finger.at,
            });
            finger.at = p;
        }
    }

    /// The long-press timer, once a step.
    fn tick(&mut self, tuning: &Tuning, t: f64, out: &mut Vec<Gesture>) {
        if let Some(finger) = &mut self.finger
            && !finger.dragging
            && !finger.long_pressed
            && t - finger.down_at >= tuning.long_press_time
        {
            finger.long_pressed = true;
            out.push(Gesture::LongPress { at: finger.down });
        }
    }
}

fn release_drag(tuning: &Tuning, finger: &Finger, out: &mut Vec<Gesture>) {
    let travel = finger.at - finger.down;
    let velocity = match (finger.recent.first(), finger.recent.last()) {
        (Some(&(t0, p0)), Some(&(t1, p1))) if t1 > t0 => (p1 - p0) / (t1 - t0) as f32,
        _ => Vec2::ZERO,
    };
    out.push(Gesture::DragEnd { travel, velocity });
    if velocity.length() >= tuning.flick_speed {
        out.push(Gesture::Flick { velocity });
    }
    let (x, y) = (travel.x.abs(), travel.y.abs());
    if travel.length() < tuning.swipe_distance {
        return;
    }
    if x >= 2. * y {
        out.push(Gesture::Swipe(if travel.x < 0. {
            Swipe::Left
        } else {
            Swipe::Right
        }));
    } else if y >= 2. * x {
        out.push(Gesture::Swipe(if travel.y < 0. {
            Swipe::Up
        } else {
            Swipe::Down
        }));
    }
}

/// Feeds this step's window events to the recognizers in arrival order. The events of one
/// step get times spread evenly over it, so a press, moves and a release that arrive
/// together still give the drag a duration and a velocity.
fn record_input(
    mut events: MessageReader<WindowEvent>,
    mut layer: ResMut<InputLayer>,
    time: Res<Time>,
    mut gestures: MessageWriter<Gesture>,
    screen: Query<&Camera, With<ScreenCamera>>,
    buttons: Query<(&ComputedNode, &UiGlobalTransform), With<InteractionPalette>>,
) {
    let layer = &mut *layer;
    // The UI's rectangles are in physical pixels.
    let scale = screen
        .single()
        .ok()
        .and_then(Camera::target_scaling_factor)
        .unwrap_or(1.);
    let on_button = |p: Vec2| {
        buttons
            .iter()
            .any(|(node, at)| node.contains_point(*at, p * scale))
    };
    let steps: Vec<Step> = events
        .read()
        .filter_map(|e| layer.pointer.step(e, on_button))
        .collect();
    let (now, dt) = (time.elapsed_secs_f64(), time.delta_secs_f64());
    let mut out = Vec::new();
    for (k, &step) in steps.iter().enumerate() {
        let t = now - dt + dt * (k + 1) as f64 / steps.len() as f64;
        layer.recognizer.step(&layer.tuning, t, step, &mut out);
    }
    layer.recognizer.tick(&layer.tuning, now, &mut out);
    gestures.write_batch(out);
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::{
        input::{mouse::MouseButtonInput, touch::TouchInput},
        time::TimeUpdateStrategy,
        window::CursorMoved,
    };

    use super::*;

    /// Runs the recognizers over timed steps, with the long-press timer after each.
    fn recognize(steps: &[(f64, Step)]) -> Vec<Gesture> {
        let (tuning, mut recognizer, mut out) = (Tuning::default(), Recognizer::default(), vec![]);
        for &(t, step) in steps {
            recognizer.step(&tuning, t, step, &mut out);
            recognizer.tick(&tuning, t, &mut out);
        }
        out
    }

    fn count(gestures: &[Gesture], f: impl Fn(&Gesture) -> bool) -> usize {
        gestures.iter().filter(|g| f(g)).count()
    }

    fn dragged(gestures: &[Gesture]) -> Vec2 {
        gestures
            .iter()
            .map(|g| match g {
                Gesture::Drag { delta } => *delta,
                _ => Vec2::ZERO,
            })
            .sum()
    }

    #[test]
    fn small_moves_across_the_slop_keep_the_whole_travel() {
        // 2 px moves: the drag starts on the fifth, past the 8 px slop.
        let mut steps = vec![(0., Step::Press(Vec2::new(100., 100.)))];
        steps.extend((1..=50).map(|k| {
            (
                k as f64 * 0.01,
                Step::Move(Vec2::new(100., 100. + 2. * k as f32)),
            )
        }));
        steps.push((0.6, Step::Release(Vec2::new(100., 200.))));
        let gestures = recognize(&steps);
        assert_eq!(dragged(&gestures), Vec2::new(0., 100.));
    }

    #[test]
    fn a_slow_drag_does_not_flick() {
        // 100 px down at 100 px/s, a step every 0.1 s, then a stop before the release.
        let mut steps = vec![(0., Step::Press(Vec2::new(100., 100.)))];
        steps.extend((1..=10).map(|k| {
            (
                k as f64 * 0.1,
                Step::Move(Vec2::new(100., 100. + 10. * k as f32)),
            )
        }));
        steps.push((1.5, Step::Release(Vec2::new(100., 200.))));
        let gestures = recognize(&steps);
        assert_eq!(
            count(&gestures, |g| matches!(g, Gesture::DragStart { .. })),
            1
        );
        assert_eq!(dragged(&gestures), Vec2::new(0., 100.));
        assert!(gestures.contains(&Gesture::DragEnd {
            travel: Vec2::new(0., 100.),
            velocity: Vec2::ZERO,
        }));
        assert_eq!(count(&gestures, |g| matches!(g, Gesture::Flick { .. })), 0);
    }

    #[test]
    fn a_fast_drag_flicks_and_swipes() {
        // 20 px right every 1/60 s: 1200 px/s.
        let mut steps = vec![(0., Step::Press(Vec2::ZERO))];
        steps.extend((1..=10).map(|k| (k as f64 / 60., Step::Move(Vec2::new(20. * k as f32, 0.)))));
        steps.push((11. / 60., Step::Release(Vec2::new(220., 0.))));
        let gestures = recognize(&steps);
        let Some(Gesture::Flick { velocity }) =
            gestures.iter().find(|g| matches!(g, Gesture::Flick { .. }))
        else {
            panic!("no flick: {gestures:?}");
        };
        assert!(
            (velocity.x - 1200.).abs() < 1. && velocity.y == 0.,
            "{velocity}"
        );
        assert_eq!(gestures.last(), Some(&Gesture::Swipe(Swipe::Right)));
    }

    #[test]
    fn taps_and_a_double_tap() {
        let at = Vec2::new(50., 50.);
        let near = at + Vec2::new(10., 0.);
        let gestures = recognize(&[
            (0., Step::Press(at)),
            (0.1, Step::Release(at)),
            (0.3, Step::Press(near)),
            (0.35, Step::Release(near)),
            // A third tap starts again.
            (0.5, Step::Press(at)),
            (0.55, Step::Release(at)),
            // Too late for a double-tap.
            (2., Step::Press(at)),
            (2.05, Step::Release(at)),
        ]);
        assert_eq!(
            gestures,
            [
                Gesture::Tap { at },
                Gesture::DoubleTap { at: near },
                Gesture::Tap { at },
                Gesture::Tap { at },
            ]
        );
    }

    #[test]
    fn a_long_press_fires_once_and_is_no_tap() {
        let at = Vec2::new(50., 50.);
        let mut steps = vec![(0., Step::Press(at))];
        // Inside the slop.
        steps.extend((1..=8).map(|k| (k as f64 * 0.1, Step::Move(at + Vec2::new(3., 0.)))));
        steps.push((1., Step::Release(at)));
        assert_eq!(recognize(&steps), [Gesture::LongPress { at }]);
    }

    #[test]
    fn a_cancel_ends_the_drag_with_nothing_else() {
        let gestures = recognize(&[
            (0., Step::Press(Vec2::ZERO)),
            (0.01, Step::Move(Vec2::new(0., 100.))),
            (0.02, Step::Cancel),
        ]);
        assert_eq!(
            gestures,
            [
                Gesture::DragStart { at: Vec2::ZERO },
                Gesture::Drag {
                    delta: Vec2::new(0., 100.)
                },
                Gesture::DragCancel,
            ]
        );
    }

    fn touch(id: u64, phase: TouchPhase, x: f32, y: f32) -> WindowEvent {
        WindowEvent::TouchInput(TouchInput {
            phase,
            position: Vec2::new(x, y),
            window: Entity::PLACEHOLDER,
            force: None,
            id,
        })
    }

    /// A press on a button (x under 50 here) is the button's: the finger after it drives.
    #[test]
    fn a_press_on_a_button_is_not_the_pointers() {
        let mut pointer = Pointer::default();
        let steps: Vec<Step> = [
            touch(1, TouchPhase::Started, 10., 0.),
            touch(1, TouchPhase::Moved, 100., 0.),
            touch(1, TouchPhase::Ended, 100., 0.),
            touch(2, TouchPhase::Started, 60., 0.),
            touch(2, TouchPhase::Ended, 60., 0.),
        ]
        .iter()
        .filter_map(|e| pointer.step(e, |p| p.x < 50.))
        .collect();
        assert_eq!(
            steps,
            [
                Step::Press(Vec2::new(60., 0.)),
                Step::Release(Vec2::new(60., 0.))
            ]
        );
    }

    #[test]
    fn the_first_finger_drives() {
        let mut pointer = Pointer::default();
        let steps: Vec<Step> = [
            touch(1, TouchPhase::Started, 0., 0.),
            touch(2, TouchPhase::Started, 9., 9.),
            touch(2, TouchPhase::Moved, 8., 8.),
            touch(1, TouchPhase::Moved, 0., 5.),
            touch(1, TouchPhase::Ended, 0., 6.),
            touch(2, TouchPhase::Ended, 7., 7.),
        ]
        .iter()
        .filter_map(|e| pointer.step(e, |_| false))
        .collect();
        assert_eq!(
            steps,
            [
                Step::Press(Vec2::ZERO),
                Step::Move(Vec2::new(0., 5.)),
                Step::Release(Vec2::new(0., 6.)),
            ]
        );
    }

    #[test]
    fn the_mouse_feeds_the_same_stream() {
        let cursor = |x, y| {
            WindowEvent::CursorMoved(CursorMoved {
                window: Entity::PLACEHOLDER,
                position: Vec2::new(x, y),
                delta: None,
            })
        };
        let button = |state| {
            WindowEvent::MouseButtonInput(MouseButtonInput {
                button: MouseButton::Left,
                state,
                window: Entity::PLACEHOLDER,
            })
        };
        let mut pointer = Pointer::default();
        let steps: Vec<Step> = [
            // A move with the button up is no step.
            cursor(1., 1.),
            button(ButtonState::Pressed),
            // A touch while the button is down is ignored.
            touch(1, TouchPhase::Started, 9., 9.),
            cursor(1., 4.),
            button(ButtonState::Released),
        ]
        .iter()
        .filter_map(|e| pointer.step(e, |_| false))
        .collect();
        assert_eq!(
            steps,
            [
                Step::Press(Vec2::new(1., 1.)),
                Step::Move(Vec2::new(1., 4.)),
                Step::Release(Vec2::new(1., 4.)),
            ]
        );
    }

    /// Each gesture with the number of the fixed step that got it.
    #[derive(Resource, Default)]
    struct Seen(Vec<(u32, Gesture)>);

    /// A swipe whose press, moves and release arrive in one frame, with frames of `frame`:
    /// longer than a fixed step (several steps a frame) or shorter (frames with none).
    fn swipe_in_one_frame(frame: Duration) -> Vec<(u32, Gesture)> {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_message::<WindowEvent>()
            .add_plugins(plugin)
            .init_resource::<Seen>()
            .insert_resource(TimeUpdateStrategy::ManualDuration(frame))
            .add_systems(
                FixedUpdate,
                (|mut gestures: MessageReader<Gesture>,
                  mut seen: ResMut<Seen>,
                  mut fixed_step: Local<u32>| {
                    *fixed_step += 1;
                    seen.0.extend(gestures.read().map(|&g| (*fixed_step, g)));
                })
                .after(record_input),
            );
        app.update();
        app.world_mut().write_message_batch([
            touch(0, TouchPhase::Started, 100., 600.),
            touch(0, TouchPhase::Moved, 100., 500.),
            touch(0, TouchPhase::Moved, 100., 400.),
            touch(0, TouchPhase::Ended, 100., 300.),
        ]);
        for _ in 0..20 {
            app.update();
        }
        std::mem::take(&mut app.world_mut().resource_mut::<Seen>().0)
    }

    #[test]
    fn a_swipe_in_one_frame_reaches_one_fixed_step() {
        for frame in [
            Duration::from_secs_f64(1. / 30.),
            Duration::from_secs_f64(1. / 240.),
        ] {
            let seen = swipe_in_one_frame(frame);
            assert!(
                seen.iter().all(|&(step, _)| step == seen[0].0),
                "{frame:?}: {seen:?}"
            );
            let gestures: Vec<Gesture> = seen.iter().map(|&(_, g)| g).collect();
            assert_eq!(
                count(&gestures, |g| matches!(g, Gesture::DragStart { .. })),
                1,
                "{frame:?}"
            );
            assert_eq!(dragged(&gestures), Vec2::new(0., -300.), "{frame:?}");
            assert_eq!(
                count(&gestures, |g| matches!(g, Gesture::DragEnd { .. })),
                1,
                "{frame:?}"
            );
            assert_eq!(
                count(&gestures, |g| matches!(g, Gesture::Flick { .. })),
                1,
                "{frame:?}"
            );
            assert_eq!(
                gestures.last(),
                Some(&Gesture::Swipe(Swipe::Up)),
                "{frame:?}"
            );
        }
    }
}
