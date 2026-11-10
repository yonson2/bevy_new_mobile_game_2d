//! The screens and the state that picks one.

mod game_over;
mod gameplay;
mod loading;
mod settings;
mod title;

use bevy::prelude::*;

pub use gameplay::Pause;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_plugins((
        game_over::plugin,
        gameplay::plugin,
        loading::plugin,
        settings::plugin,
        title::plugin,
    ));
}

/// The screen on show. Each root entity of a screen carries `DespawnOnExit<Screen>`.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Loading,
    Title,
    Settings,
    Gameplay,
    GameOver,
}
