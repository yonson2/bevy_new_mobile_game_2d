//! The UI's colors. Placeholder values: the art direction replaces them
//! (`docs/ui-style.md`).

use bevy::prelude::*;

/// #1b1f2a, the clear color.
pub const BACKGROUND: Color = Color::srgb(0.106, 0.122, 0.165);
/// #1b1f2a at 90 %, over a paused run.
pub const OVERLAY: Color = Color::srgba(0.106, 0.122, 0.165, 0.9);

/// #f2c14e
pub const HEADER_TEXT: Color = Color::srgb(0.949, 0.757, 0.306);
/// #c8d0e0
pub const LABEL_TEXT: Color = Color::srgb(0.784, 0.816, 0.878);

/// #f5f7fa
pub const BUTTON_TEXT: Color = Color::srgb(0.961, 0.969, 0.980);
/// #2e3648
pub const BUTTON_BACKGROUND: Color = Color::srgb(0.180, 0.212, 0.282);
/// #3c4660
pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.235, 0.275, 0.376);
/// #151922
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.082, 0.098, 0.133);
/// #5a6785
pub const BUTTON_EDGE: Color = Color::srgb(0.353, 0.404, 0.522);
