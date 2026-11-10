//! The settings screen: a row per setting, its name on the left and its value on the
//! right; a tap steps it to the next value. The save keeps them.

use bevy::prelude::*;

use super::Screen;
use crate::{
    save::{Save, Settings},
    strings::strings,
    theme::widget::{self, RowValue},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn_settings_screen.spawn())
        .add_systems(Update, relabel.run_if(in_state(Screen::Settings)));
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Default)]
enum Setting {
    #[default]
    Sound,
    Haptics,
}

impl Setting {
    fn name(self) -> &'static str {
        match self {
            Setting::Sound => strings().sound,
            Setting::Haptics => strings().haptics,
        }
    }

    fn value(self, settings: &Settings) -> &'static str {
        let on = match self {
            Setting::Sound => settings.sound,
            Setting::Haptics => settings.haptics,
        };
        if on { strings().on } else { strings().off }
    }

    fn step(self, settings: &mut Settings) {
        match self {
            Setting::Sound => settings.sound ^= true,
            Setting::Haptics => settings.haptics ^= true,
        }
    }
}

fn spawn_settings_screen() -> impl Scene {
    bsn! {
        @widget::ui_root("Settings Screen")
        DespawnOnExit<Screen>(Screen::Settings)
        Children [
            @widget::header(strings().settings)
            --
            @widget::row(Setting::Sound.name(), step)
            Setting::Sound
            --
            @widget::row(Setting::Haptics.name(), step)
            Setting::Haptics
            --
            @widget::button(strings().back, to_title)
        ]
    }
}

fn step(click: On<PointerClick>, rows: Query<&Setting>, mut save: ResMut<Save>) {
    if let Ok(setting) = rows.get(click.event_target()) {
        setting.step(&mut save.settings);
    }
}

fn relabel(
    save: Res<Save>,
    rows: Query<(&Setting, &Children)>,
    mut values: Query<&mut Text, With<RowValue>>,
) {
    for (setting, children) in &rows {
        let value = setting.value(&save.settings);
        for &child in children {
            if let Ok(mut text) = values.get_mut(child)
                && text.0 != value
            {
                text.0 = value.into();
            }
        }
    }
}

fn to_title(_: On<PointerClick>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_setting_steps_through_its_values_and_back() {
        let mut settings = Settings::default();
        for row in [Setting::Sound, Setting::Haptics] {
            assert_eq!(row.value(&settings), "On");
            row.step(&mut settings);
            assert_eq!(row.value(&settings), "Off");
            row.step(&mut settings);
        }
        assert_eq!(settings, Settings::default());
    }
}
