//! The gameplay screen's pause and its menu. The game draws the rest (`demo`).

use bevy::prelude::*;

use super::Screen;
use crate::{
    platform::InSafeArea,
    strings::strings,
    theme::{palette, widget},
};

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<Pause>();
    app.add_systems(OnEnter(Pause::Paused), spawn_pause_menu.spawn());
}

/// Whether the run plays. A sub-state of `Screen::Gameplay`: it exists only during a run
/// and starts as `Running` in each one.
#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(Screen = Screen::Gameplay)]
pub enum Pause {
    #[default]
    Running,
    Paused,
}

/// The pause menu. Its root covers the screen and takes every press, so the gameplay
/// screen under it gets none.
fn spawn_pause_menu() -> impl Scene {
    bsn! {
        #PauseMenu
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
        }
        GlobalZIndex(5)
        BackgroundColor(palette::OVERLAY)
        InSafeArea
        DespawnOnExit<Pause>(Pause::Paused)
        Children [
            @widget::header(strings().paused)
            --
            @widget::button(strings().resume, resume)
            --
            @widget::button(strings().to_title, quit_to_title)
        ]
    }
}

fn resume(_: On<PointerClick>, mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause::Running);
}

fn quit_to_title(_: On<PointerClick>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
