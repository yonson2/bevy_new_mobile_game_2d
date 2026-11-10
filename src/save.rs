//! The save file: the settings and the best score, as JSON in the app's data folder. It
//! loads when the app starts (no file or a bad one: the defaults) and is written at each
//! change, through a temporary file, so a kill mid-write keeps the old one. A shot run
//! (`dev_tools`) neither reads nor writes it, so a seed plays the same run.

use std::path::{Path, PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    let file = data_file("save.json");
    let save = file.as_deref().map(load).unwrap_or_default();
    app.insert_resource(save)
        .insert_resource(SaveFile(file))
        .add_systems(Last, write.run_if(changed));
}

/// `#[serde(default)]`: a save from an older version fills in what it lacks.
#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Save {
    pub settings: Settings,
    /// The demo's best score.
    pub best: u32,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub sound: bool,
    pub haptics: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sound: true,
            haptics: true,
        }
    }
}

#[derive(Resource)]
struct SaveFile(Option<PathBuf>);

/// A file in the app's data folder; none in a shot run, which leaves the disk alone.
pub fn data_file(name: &str) -> Option<PathBuf> {
    #[cfg(feature = "dev_tools")]
    if crate::dev_tools::is_shot_run() {
        return None;
    }
    Some(data_dir()?.join(name))
}

#[cfg(target_os = "android")]
fn data_dir() -> Option<PathBuf> {
    bevy::android::ANDROID_APP.get()?.internal_data_path()
}

/// iOS gives each app its own `HOME`.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn data_dir() -> Option<PathBuf> {
    let home = PathBuf::from(std::env::var_os("HOME")?);
    Some(
        home.join("Library/Application Support")
            .join(env!("CARGO_PKG_NAME")),
    )
}

#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "android")))]
fn data_dir() -> Option<PathBuf> {
    let data = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| Some(PathBuf::from(std::env::var_os("HOME")?).join(".local/share")))?;
    Some(data.join(env!("CARGO_PKG_NAME")))
}

fn load(path: &Path) -> Save {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Save::default();
    };
    match serde_json::from_str(&text) {
        Ok(save) => {
            info!("save loaded from {}", path.display());
            save
        }
        Err(err) => {
            warn!("save {} unreadable, using defaults: {err}", path.display());
            Save::default()
        }
    }
}

/// A change, but not the load: a loaded save is already on disk.
fn changed(save: Res<Save>) -> bool {
    save.is_changed() && !save.is_added()
}

fn write(save: Res<Save>, file: Res<SaveFile>) {
    let Some(path) = &file.0 else {
        return;
    };
    let result = serde_json::to_string_pretty(&*save)
        .map_err(Into::into)
        .and_then(|text| write_file(path, &text));
    match result {
        Ok(()) => info!("save written to {}", path.display()),
        Err(err) => warn!("save {} not written: {err}", path.display()),
    }
}

/// Writes `text` to `path` through a temporary file, so a kill mid-write keeps the old one.
fn write_file(path: &Path, text: &str) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, text)?;
    std::fs::rename(&tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_save_survives_a_round_trip_and_a_partial_file_fills_in_defaults() {
        let save = Save {
            settings: Settings {
                sound: false,
                haptics: false,
            },
            best: 12,
        };
        let text = serde_json::to_string_pretty(&save).unwrap();
        assert_eq!(serde_json::from_str::<Save>(&text).unwrap(), save);

        let old: Save = serde_json::from_str(r#"{"settings": {"haptics": false}}"#).unwrap();
        assert_eq!(
            old,
            Save {
                settings: Settings {
                    haptics: false,
                    ..default()
                },
                best: 0,
            }
        );
    }

    #[test]
    fn a_file_is_written_on_a_change_and_read_back_and_a_bad_one_gives_defaults() {
        let dir = std::env::temp_dir().join(format!("game-save-{}", std::process::id()));
        let path = dir.join("nested/save.json");
        let mut app = App::new();
        app.insert_resource(Save::default())
            .insert_resource(SaveFile(Some(path.clone())))
            .add_systems(Update, write.run_if(changed));
        app.update();
        assert!(!path.exists(), "a loaded save is not written again");
        app.world_mut().resource_mut::<Save>().best = 7;
        app.update();
        assert_eq!(load(&path).best, 7);

        std::fs::write(&path, "not json").unwrap();
        assert_eq!(load(&path), Save::default());
        assert_eq!(load(&dir.join("missing.json")), Save::default());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
