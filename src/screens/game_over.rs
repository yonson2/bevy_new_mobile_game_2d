//! The run's end: the score, the best score, and a share of the screen as a picture.

use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};

use super::Screen;
use crate::{
    audio::Sfx,
    demo::Run,
    save::{Save, data_file},
    share::Share,
    strings::strings,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::GameOver),
        (spawn_game_over_screen, |mut sounds: MessageWriter<Sfx>| {
            sounds.write(Sfx::GameOver);
        }),
    );
}

fn spawn_game_over_screen(mut commands: Commands, run: Res<Run>, save: Res<Save>) {
    let s = strings();
    let score = format!("{} {}", s.score, run.score);
    let best = format!("{} {}", s.best, save.best);
    commands.spawn_scene(bsn! {
        @widget::ui_root("Game Over Screen")
        DespawnOnExit<Screen>(Screen::GameOver)
        Children [
            @widget::header(s.game_over)
            --
            @widget::label(score)
            --
            @widget::label(best)
            --
            @widget::button(s.play_again, play_again)
            --
            @widget::button(s.share, share)
            --
            @widget::button(s.to_title, to_title)
        ]
    });
}

fn play_again(_: On<PointerClick>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn to_title(_: On<PointerClick>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

/// Saves the screen as a picture in the app's data folder, then opens the share sheet
/// with it. Android's `FileProvider` serves the `share` folder (`res/xml/share_paths.xml`).
fn share(_: On<PointerClick>, mut commands: Commands, run: Res<Run>) {
    let Some(file) = data_file("share/score.png") else {
        return;
    };
    if let Some(dir) = file.parent()
        && let Err(err) = std::fs::create_dir_all(dir)
    {
        warn!("share: {}: {err}", dir.display());
        return;
    }
    let text = strings().share_text.replace("{}", &run.score.to_string());
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(file.clone()))
        .observe(move |_: On<ScreenshotCaptured>, mut commands: Commands| {
            commands.trigger(Share {
                text: text.clone(),
                file: file.clone(),
            });
        });
}
