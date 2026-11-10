//! The player: a drag moves it by the finger's travel.

use bevy::{image::TextureAtlasTemplate, prelude::*};

use super::{FRAME, Play};
use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource, canvas::Canvas, input::Gesture,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerArt>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);
    app.add_systems(FixedUpdate, follow_drag.in_set(Play::Player));
    // Drawing only: `Update`.
    app.add_systems(
        Update,
        animate.in_set(AppSystems::Update).in_set(PausableSystems),
    );
}

#[derive(Component, Default, Clone)]
pub struct Player;

/// The player's body in world units. The sprite is larger, so an enemy hits only once the
/// two clearly overlap.
pub const RADIUS: f32 = 5.;

/// The idle clip: the frames of the sheet's one row, and their rate, a second.
const IDLE_FRAMES: u32 = 8;
const IDLE_RATE: f32 = 8.;

/// The player's sheet, loaded on the loading screen.
#[derive(Resource, Asset, Clone, TypePath)]
struct PlayerArt {
    #[dependency]
    sheet: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for PlayerArt {
    fn from_world(world: &mut World) -> Self {
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), IDLE_FRAMES, 1, None, None);
        let layout = world
            .resource_mut::<Assets<TextureAtlasLayout>>()
            .add(layout);
        let assets = world.resource::<AssetServer>();
        Self {
            sheet: world
                .resource::<Canvas>()
                .load(assets, "images/player-sheet.png"),
            layout,
        }
    }
}

fn spawn_player(mut commands: Commands, art: Res<PlayerArt>, canvas: Res<Canvas>) {
    let image = art.sheet.clone();
    let atlas = TextureAtlasTemplate {
        layout: art.layout.clone().into(),
        index: 0,
    };
    let transform = Transform::from_xyz(canvas.world_size().x / 2., 60., 1.);
    commands.spawn_scene(bsn! {
        #Player
        Player
        Sprite {
            image: {image},
            texture_atlas: {atlas},
            custom_size: {Some(Vec2::splat(FRAME))},
        }
        transform
        DespawnOnExit<Screen>(Screen::Gameplay)
    });
}

/// The drag's deltas move the player, inside the canvas.
fn follow_drag(
    mut gestures: MessageReader<Gesture>,
    canvas: Res<Canvas>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    for gesture in gestures.read() {
        if let Gesture::Drag { delta } = *gesture {
            let at = player.translation.xy() + canvas.window_to_world(delta);
            let at = at.clamp(Vec2::ZERO, canvas.world_size());
            player.translation = at.extend(player.translation.z);
        }
    }
}

fn animate(time: Res<Time>, mut player: Single<&mut Sprite, With<Player>>) {
    let frame = (time.elapsed_secs() * IDLE_RATE) as usize % IDLE_FRAMES as usize;
    if let Some(atlas) = &mut player.texture_atlas
        && atlas.index != frame
    {
        atlas.index = frame;
    }
}
