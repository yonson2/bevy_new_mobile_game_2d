//! The enemies: they come in at the top at a seeded place and speed, and fall. One that
//! leaves at the bottom is a point; one that touches the player ends the run.

use bevy::prelude::*;

use super::{FRAME, Play, Run, player};
use crate::{
    asset_tracking::LoadResource,
    audio::Sfx,
    canvas::Canvas,
    haptics::Haptic,
    rng::{GameRng, Stream},
    save::Save,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<EnemyArt>();
    app.init_resource::<Spawner>();
    app.add_systems(OnEnter(Screen::Gameplay), |mut spawner: ResMut<Spawner>| {
        spawner.wait = FIRST_SPAWN_SECS;
    });
    app.add_systems(
        FixedUpdate,
        (spawn, fall, hit_player).chain().in_set(Play::Enemies),
    );
}

#[derive(Component, Default, Clone)]
pub struct Enemy {
    /// World units a second, down.
    speed: f32,
}

const RADIUS: f32 = 5.;
const FIRST_SPAWN_SECS: f32 = 1.;
/// The seconds between two enemies at score 0, what each point takes off, and the least.
const SPAWN_SECS: f32 = 0.9;
const SPAWN_SECS_A_POINT: f32 = 0.01;
const SPAWN_SECS_MIN: f32 = 0.3;

/// The seconds until the next enemy.
#[derive(Resource, Default)]
struct Spawner {
    wait: f32,
}

#[derive(Resource, Asset, Clone, TypePath)]
struct EnemyArt {
    #[dependency]
    image: Handle<Image>,
}

impl FromWorld for EnemyArt {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            image: world.resource::<Canvas>().load(assets, "images/enemy.png"),
        }
    }
}

fn spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut spawner: ResMut<Spawner>,
    mut rng: ResMut<GameRng>,
    canvas: Res<Canvas>,
    art: Res<EnemyArt>,
    run: Res<Run>,
) {
    spawner.wait -= time.delta_secs();
    if spawner.wait > 0. {
        return;
    }
    spawner.wait += (SPAWN_SECS - SPAWN_SECS_A_POINT * run.score as f32).max(SPAWN_SECS_MIN);
    // `u64` draws only: other integer sizes draw differently on 32-bit targets.
    let rng = rng.stream(Stream::Spawns);
    let size = canvas.world_size();
    let x = FRAME / 2. + rng.u64(0..=1000) as f32 / 1000. * (size.x - FRAME);
    let enemy = Enemy {
        speed: 40. + rng.u64(0..=40) as f32,
    };
    let image = art.image.clone();
    let transform = Transform::from_xyz(x, size.y + FRAME, 0.);
    commands.spawn_scene(bsn! {
        #Enemy
        enemy
        Sprite { image: {image}, custom_size: {Some(Vec2::splat(FRAME))} }
        transform
        DespawnOnExit<Screen>(Screen::Gameplay)
    });
}

fn fall(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<(Entity, &Enemy, &mut Transform)>,
    mut run: ResMut<Run>,
    mut sounds: MessageWriter<Sfx>,
    mut haptics: MessageWriter<Haptic>,
) {
    for (entity, enemy, mut transform) in &mut enemies {
        transform.translation.y -= enemy.speed * time.delta_secs();
        if transform.translation.y < -FRAME {
            commands.entity(entity).despawn();
            run.score += 1;
            sounds.write(Sfx::Point);
            haptics.write(Haptic::Pulse);
        }
    }
}

fn hit_player(
    player: Single<&Transform, With<player::Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    mut run: ResMut<Run>,
    mut save: ResMut<Save>,
    mut sounds: MessageWriter<Sfx>,
    mut haptics: MessageWriter<Haptic>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let at = player.translation.xy();
    let reach = player::RADIUS + RADIUS;
    if !enemies
        .iter()
        .any(|enemy| enemy.translation.xy().distance_squared(at) < reach * reach)
    {
        return;
    }
    sounds.write(Sfx::Hit);
    haptics.write(Haptic::Rumble);
    run.over = true;
    if run.score > save.best {
        save.best = run.score;
    }
    next_screen.set(Screen::GameOver);
}
