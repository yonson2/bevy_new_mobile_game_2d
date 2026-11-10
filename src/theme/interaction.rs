//! Press and hover colors, the press sound and its haptic, from bevy_new_2d
//! (`src/theme/interaction.rs`). On the desktop the mouse stands in for a finger.

use bevy::prelude::*;

use crate::{audio::Sfx, haptics::Haptic};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(apply_interaction_palette_on_press);
    app.add_observer(apply_interaction_palette_on_over);
    app.add_observer(apply_interaction_palette_on_out);
    app.add_observer(apply_interaction_palette_on_release);
}

/// The [`BackgroundColor`] of a pickable entity (a button) for each interaction state.
#[derive(Component, Debug, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette_on_press(
    press: On<PointerPress>,
    mut palette_query: Query<(&InteractionPalette, &mut BackgroundColor)>,
    mut sounds: MessageWriter<Sfx>,
    mut haptics: MessageWriter<Haptic>,
) {
    let Ok((palette, mut bg)) = palette_query.get_mut(press.event_target()) else {
        return;
    };
    *bg = palette.pressed.into();
    sounds.write(Sfx::Press);
    haptics.write(Haptic::Tick);
}

fn apply_interaction_palette_on_release(
    release: On<PointerRelease>,
    mut palette_query: Query<(&InteractionPalette, &mut BackgroundColor)>,
) {
    let Ok((palette, mut bg)) = palette_query.get_mut(release.event_target()) else {
        return;
    };
    *bg = palette.hovered.into();
}

fn apply_interaction_palette_on_over(
    over: On<PointerOver>,
    mut palette_query: Query<(&InteractionPalette, &mut BackgroundColor)>,
) {
    let Ok((palette, mut bg)) = palette_query.get_mut(over.event_target()) else {
        return;
    };
    *bg = palette.hovered.into();
}

fn apply_interaction_palette_on_out(
    out: On<PointerOut>,
    mut palette_query: Query<(&InteractionPalette, &mut BackgroundColor)>,
) {
    let Ok((palette, mut bg)) = palette_query.get_mut(out.event_target()) else {
        return;
    };
    *bg = palette.none.into();
}
