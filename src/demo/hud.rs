//! The run's UI: the score and the pause button at the top, and a hint until the first
//! drag.

use bevy::prelude::*;

use super::Run;
use crate::{
    input::Gesture,
    platform::InSafeTop,
    screens::{Pause, Screen},
    strings::strings,
    theme::{font, palette, widget},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_hud.spawn());
    app.add_systems(
        Update,
        (show_score.run_if(resource_changed::<Run>), hide_hint).run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Default, Clone)]
struct ScoreText;

#[derive(Component, Default, Clone)]
struct Hint;

fn spawn_hud() -> impl Scene {
    let hint = strings().hint;
    bsn! {
        #Hud
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
        }
        Pickable::IGNORE
        InSafeTop
        DespawnOnExit<Screen>(Screen::Gameplay)
        Children [
            #Bar
            Node {
                width: percent(100),
                padding: {UiRect::all(px(12))},
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
            }
            Pickable::IGNORE
            Children [
                #ScoreText
                ScoreText
                Text("0")
                TextFont { font_size: {font::HUGE} }
                TextColor(palette::HEADER_TEXT)
                --
                @widget::chip("II", pause)
            ]
            --
            #Hint
            Hint
            Node { margin: {UiRect::bottom(px(220))} }
            Pickable::IGNORE
            Children [
                Text(hint)
                TextFont { font_size: {font::SMALL} }
                TextColor(palette::LABEL_TEXT)
            ]
        ]
    }
}

fn show_score(run: Res<Run>, mut text: Single<&mut Text, With<ScoreText>>) {
    let score = run.score.to_string();
    if text.0 != score {
        text.0 = score;
    }
}

fn hide_hint(
    mut commands: Commands,
    mut gestures: MessageReader<Gesture>,
    hint: Query<Entity, With<Hint>>,
) {
    if gestures
        .read()
        .any(|g| matches!(g, Gesture::DragStart { .. }))
    {
        for hint in &hint {
            commands.entity(hint).despawn();
        }
    }
}

fn pause(_: On<PointerClick>, mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause::Paused);
}
