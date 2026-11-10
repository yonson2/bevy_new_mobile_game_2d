//! The look shared by every screen: the fonts, the palette, the press colors and the
//! widgets.

pub mod font;
pub mod interaction;
pub mod palette;
pub mod widget;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((font::plugin, interaction::plugin));
}
