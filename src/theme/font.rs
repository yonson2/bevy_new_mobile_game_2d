//! The font and its sizes, in canvas pixels (`Rem`: the canvas sets `RemSize` to its
//! pixel's side on the window). In `Style::Pixel` a font pixel then covers the same whole
//! screen pixels as a canvas pixel and stays sharp.
//!
//! The font is compiled in, as Bevy's own default font is: Departure Mono takes the
//! default font's place before the first frame, so every text has it.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let font = include_bytes!("../../assets/fonts/DepartureMono-Regular.otf");
    app.world_mut()
        .resource_mut::<Assets<Font>>()
        .insert(AssetId::default(), Font::from_bytes(font.to_vec()))
        .expect("the default font's id is valid");
}

/// The font's em in canvas pixels: Departure Mono draws one font pixel a canvas pixel at
/// 11. A pixel font stays sharp only at whole multiples of its em.
const EM: f32 = 11.;
/// Most text: the HUD, hints.
pub const SMALL: FontSize = FontSize::Rem(EM);
/// Buttons and labels.
pub const LARGE: FontSize = FontSize::Rem(2. * EM);
/// Headers and the biggest numbers.
pub const HUGE: FontSize = FontSize::Rem(3. * EM);
