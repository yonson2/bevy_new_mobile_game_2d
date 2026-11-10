//! The demo game: drag the player away from what falls; each enemy that passes is a
//! point. It is here so the template's checks have something to check (the shots, the
//! replay, the phone builds), and to show each pattern once: assets on the loading
//! screen, seeded draws, gameplay in `FixedUpdate`, sounds and haptics as messages, UI in
//! BSN. M1 replaces this folder with the game (`issues/`).

mod enemies;
mod hud;
mod player;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Run>();
    app.add_systems(OnEnter(Screen::Gameplay), |mut run: ResMut<Run>| {
        *run = default();
    });
    // Each gameplay system has its place in a step: a seed then plays the same run.
    app.configure_sets(
        FixedUpdate,
        (Play::Clock, Play::Player, Play::Enemies)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(|run: Res<Run>| !run.over),
    );
    app.add_systems(
        FixedUpdate,
        (|mut run: ResMut<Run>| run.steps += 1).in_set(Play::Clock),
    );
    app.add_plugins((enemies::plugin, hud::plugin, player::plugin));
}

/// The order of the gameplay systems in a fixed step.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Play {
    Clock,
    Player,
    Enemies,
}

/// The run in play.
#[derive(Resource, Default)]
pub struct Run {
    /// The enemies that passed the player.
    pub score: u32,
    /// The fixed steps played.
    steps: u32,
    /// An enemy hit the player. The gameplay stops in that fixed step: the screen changes
    /// only with the next frame, and a frame can hold more fixed steps.
    over: bool,
}

/// The side of a sprite's frame in world units: 32 art pixels.
const FRAME: f32 = 16.;

/// A hash of the run's state for the replay check, and a summary: the run, the player,
/// each enemy and the RNG. Floats go in by their bits.
#[cfg(feature = "dev_tools")]
pub fn state_hash(world: &mut World) -> (u64, String) {
    use std::hash::{DefaultHasher, Hash, Hasher};

    let bits = |t: &Transform| [t.translation.x.to_bits(), t.translation.y.to_bits()];
    let mut hasher = DefaultHasher::new();
    let run = world.resource::<Run>();
    let (score, steps) = (run.score, run.steps);
    (score, steps, run.over).hash(&mut hasher);
    let mut player = world.query_filtered::<&Transform, With<player::Player>>();
    let player: Vec<_> = player.iter(world).map(bits).collect();
    player.hash(&mut hasher);
    let mut enemies = world.query_filtered::<&Transform, With<enemies::Enemy>>();
    let mut enemies: Vec<_> = enemies.iter(world).map(bits).collect();
    enemies.sort();
    enemies.hash(&mut hasher);
    world
        .resource::<crate::rng::GameRng>()
        .state()
        .hash(&mut hasher);
    let screen = **world.resource::<State<Screen>>();
    let summary = format!(
        "score {score} in {steps} steps, {} enemies, {screen:?}",
        enemies.len()
    );
    (hasher.finish(), summary)
}
