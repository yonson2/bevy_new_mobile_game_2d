//! The first screen: it waits until every asset collection (`LoadResource`) has loaded,
//! then shows the title.

use bevy::prelude::*;

use super::Screen;
use crate::{asset_tracking::ResourceHandles, strings::strings, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen.spawn());
    app.add_systems(
        Update,
        enter_title.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn spawn_loading_screen() -> impl Scene {
    bsn! {
        @widget::ui_root("Loading Screen")
        DespawnOnExit<Screen>(Screen::Loading)
        Children [
            @widget::label(strings().loading)
        ]
    }
}

fn enter_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
