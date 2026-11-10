//! Music and sound effects. The game writes an [`Sfx`] at each event with a sound. The
//! music follows the screen and crossfades when the track changes. All sound stops while
//! the app is suspended, and the Sound setting mutes it.

use bevy::{audio::Volume, prelude::*, window::AppLifecycle};

use crate::{asset_tracking::LoadResource, save::Save, screens::Screen};

const CROSSFADE_SECS: f32 = 1.;
/// The music's level under the sound effects.
const MUSIC_GAIN: f32 = 0.5;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<Sfx>()
        .init_resource::<MusicTrack>()
        .load_resource::<SfxAssets>()
        .load_resource::<MusicAssets>()
        .add_systems(
            Update,
            (
                play_sfx.run_if(resource_exists::<SfxAssets>),
                (
                    choose_music.run_if(resource_exists::<MusicAssets>),
                    switch_music.run_if(resource_changed::<MusicTrack>),
                    fade_music,
                )
                    .chain(),
                apply_sound_setting.run_if(resource_changed::<Save>),
                apply_global_volume.run_if(resource_changed::<GlobalVolume>),
                pause_on_suspend,
                log_started,
            ),
        );
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Track {
    Title,
    Game,
}

impl Track {
    const ALL: [Track; 2] = [Track::Title, Track::Game];

    /// Its file, `assets/audio/music/<file>.ogg`: a loop.
    fn file(self) -> &'static str {
        match self {
            Track::Title => "title",
            Track::Game => "game",
        }
    }
}

fn music_for(screen: Screen) -> Option<Track> {
    match screen {
        Screen::Loading => None,
        Screen::Gameplay => Some(Track::Game),
        Screen::Title | Screen::Settings | Screen::GameOver => Some(Track::Title),
    }
}

/// The music, loaded once on the loading screen, in `Track` order.
#[derive(Resource, Asset, Clone, TypePath)]
struct MusicAssets {
    #[dependency]
    tracks: [Handle<AudioSource>; 2],
}

impl FromWorld for MusicAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            tracks: Track::ALL.map(|t| assets.load(format!("audio/music/{}.ogg", t.file()))),
        }
    }
}

/// The track that should play; the old one fades out as the new one fades in.
#[derive(Resource, Default, PartialEq)]
struct MusicTrack(Option<Handle<AudioSource>>);

/// A playing track and its fade: `gain` moves toward `target` in `CROSSFADE_SECS`.
#[derive(Component)]
struct Music {
    gain: f32,
    target: f32,
}

fn choose_music(
    screen: Res<State<Screen>>,
    music: Res<MusicAssets>,
    mut track: ResMut<MusicTrack>,
) {
    let handle = music_for(*screen.get()).map(|t| music.tracks[t as usize].clone());
    track.set_if_neq(MusicTrack(handle));
}

fn switch_music(
    mut commands: Commands,
    track: Res<MusicTrack>,
    mut playing: Query<(&AudioPlayer, &mut Music)>,
) {
    let mut found = false;
    for (player, mut music) in &mut playing {
        let current = track.0.as_ref() == Some(&player.0);
        music.target = if current { 1. } else { 0. };
        found |= current;
    }
    if let (false, Some(handle)) = (found, &track.0) {
        commands.spawn((
            Music {
                gain: 0.,
                target: 1.,
            },
            AudioPlayer(handle.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::SILENT),
        ));
    }
}

/// Real time: the music goes on while the game is paused.
fn fade_music(
    mut commands: Commands,
    time: Res<Time<Real>>,
    global_volume: Res<GlobalVolume>,
    mut tracks: Query<(Entity, &mut Music, Option<&mut AudioSink>)>,
) {
    let step = time.delta_secs() / CROSSFADE_SECS;
    for (entity, mut music, sink) in &mut tracks {
        music.gain = if music.gain < music.target {
            (music.gain + step).min(music.target)
        } else {
            (music.gain - step).max(music.target)
        };
        if music.gain == 0. && music.target == 0. {
            commands.entity(entity).despawn();
        } else if let Some(mut sink) = sink {
            sink.set_volume(global_volume.volume * Volume::Linear(MUSIC_GAIN * music.gain));
        }
    }
}

/// A sound effect that plays once, then despawns.
fn sound_effect(handle: Handle<AudioSource>) -> impl Scene {
    bsn! {
        AudioPlayer(handle)
        PlaybackSettings::DESPAWN
    }
}

/// A game event with a sound. A message, not an observer's event: the gameplay systems
/// that write it get no `Commands`, so the schedule keeps its sync points and a seed
/// plays as before.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sfx {
    /// A press on a button.
    Press,
    /// An enemy passes the player: a point.
    Point,
    /// An enemy hits the player.
    Hit,
    /// The game-over screen: a voice line.
    GameOver,
}

impl Sfx {
    const ALL: [Sfx; 4] = [Sfx::Press, Sfx::Point, Sfx::Hit, Sfx::GameOver];

    /// Its file, `assets/audio/sfx/<file>.ogg`.
    fn file(self) -> &'static str {
        match self {
            Sfx::Press => "press",
            Sfx::Point => "point",
            Sfx::Hit => "hit",
            Sfx::GameOver => "game-over",
        }
    }
}

/// The sounds, loaded once on the loading screen, in `Sfx` order.
#[derive(Resource, Asset, Clone, TypePath)]
struct SfxAssets {
    #[dependency]
    sounds: [Handle<AudioSource>; 4],
}

impl FromWorld for SfxAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            sounds: Sfx::ALL.map(|s| assets.load(format!("audio/sfx/{}.ogg", s.file()))),
        }
    }
}

fn play_sfx(mut commands: Commands, mut sounds: MessageReader<Sfx>, audio: Res<SfxAssets>) {
    for &sfx in sounds.read() {
        commands.spawn_scene(sound_effect(audio.sounds[sfx as usize].clone()));
    }
}

fn apply_sound_setting(save: Res<Save>, mut global_volume: ResMut<GlobalVolume>) {
    let volume = if save.settings.sound {
        Volume::Linear(1.)
    } else {
        Volume::SILENT
    };
    if global_volume.volume != volume {
        global_volume.volume = volume;
    }
}

/// [`GlobalVolume`] does not change sounds that already play, so this system does.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink), Without<Music>>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

fn pause_on_suspend(mut lifecycle: MessageReader<AppLifecycle>, sinks: Query<&AudioSink>) {
    for event in lifecycle.read() {
        match event {
            AppLifecycle::Suspended => sinks.iter().for_each(|sink| sink.pause()),
            AppLifecycle::Running => sinks.iter().for_each(|sink| sink.play()),
            _ => {}
        }
    }
}

/// Each sound that starts, for checks without speakers (emulator, simulator, shot runs).
fn log_started(started: Query<&AudioPlayer, Added<AudioSink>>) {
    for player in &started {
        info!(
            "audio: {} starts",
            player.0.path().map_or("?".into(), |p| p.to_string())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_sound_and_track_has_its_file() {
        let sounds = Sfx::ALL.map(|s| format!("assets/audio/sfx/{}.ogg", s.file()));
        let tracks = Track::ALL.map(|t| format!("assets/audio/music/{}.ogg", t.file()));
        for path in sounds.iter().chain(&tracks) {
            assert!(std::path::Path::new(path).exists(), "{path}");
        }
    }

    #[test]
    fn the_music_follows_the_screen() {
        assert_eq!(music_for(Screen::Loading), None);
        assert_eq!(music_for(Screen::Title), Some(Track::Title));
        assert_eq!(music_for(Screen::GameOver), Some(Track::Title));
        assert_eq!(music_for(Screen::Gameplay), Some(Track::Game));
    }
}
