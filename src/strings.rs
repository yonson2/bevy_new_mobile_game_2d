//! Every player-facing string, in one table per language. English only for now: a new
//! language is a new table and a setting that picks one. Dev-tool text stays inline:
//! players never see it.

/// The strings of one language.
pub struct Strings {
    pub game_name: &'static str,
    pub loading: &'static str,
    pub play: &'static str,
    pub settings: &'static str,
    pub back: &'static str,
    pub sound: &'static str,
    pub haptics: &'static str,
    pub on: &'static str,
    pub off: &'static str,
    pub paused: &'static str,
    pub resume: &'static str,
    pub to_title: &'static str,
    pub game_over: &'static str,
    pub score: &'static str,
    pub best: &'static str,
    pub play_again: &'static str,
    pub share: &'static str,
    /// The share sheet's text; `{}` is the score.
    pub share_text: &'static str,
    /// The demo's hint, until the first drag.
    pub hint: &'static str,
}

const ENGLISH: Strings = Strings {
    game_name: env!("APP_NAME"),
    loading: "Loading...",
    play: "Play",
    settings: "Settings",
    back: "Back",
    sound: "Sound",
    haptics: "Haptics",
    on: "On",
    off: "Off",
    paused: "Paused",
    resume: "Resume",
    to_title: "Title",
    game_over: "Game over",
    score: "Score",
    best: "Best",
    play_again: "Play again",
    share: "Share",
    share_text: concat!("I scored {} in ", env!("APP_NAME"), "!"),
    hint: "Drag to move",
};

pub fn strings() -> &'static Strings {
    &ENGLISH
}
