//! The title screen: the name, the way into a run, and the settings.

use bevy::prelude::*;

use super::Screen;
use crate::{strings::strings, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen.spawn());
}

fn spawn_title_screen() -> impl Scene {
    bsn! {
        @widget::ui_root("Title Screen")
        DespawnOnExit<Screen>(Screen::Title)
        Children [
            @widget::header(strings().game_name)
            --
            Node { height: px(40) }
            --
            @widget::button(strings().play, enter_gameplay)
            --
            @widget::button(strings().settings, enter_settings)
        ]
    }
}

fn enter_gameplay(_: On<PointerClick>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn enter_settings(_: On<PointerClick>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Settings);
}
