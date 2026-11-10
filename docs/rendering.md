# Rendering

`src/canvas.rs` decides how the world reaches the screen.

## Units

- **Canvas pixel**: the unit of the art and the fonts. The screen is about 360 canvas
  pixels wide on every phone; its height follows the phone's shape.
- **World unit**: the unit of gameplay. One world unit is `ZOOM` = 2 canvas pixels, so the
  world is about 180 units wide. The origin is the canvas's bottom-left corner.
- A sprite's `custom_size` is in world units: a 32 px frame is 16 units.
- `Canvas::world_size()` is the visible world; `Canvas::window_to_world(delta)` turns a
  drag on the window into a move in the world. Gameplay never reads the window's pixels.

## Two styles

`pub const STYLE: Style` picks one for the game (the interview asks; M2 confirms).
`GAME_STYLE=pixel|smooth` overrides it for one dev run.

| | `Style::Pixel` | `Style::Smooth` |
|---|---|---|
| For | pixel art | any other 2D art |
| How | the world camera draws into a low-resolution image; the screen camera draws that image at a whole-number scale with nearest sampling, then the UI | one camera draws the world and the UI at the screen's resolution |
| Canvas width | the screen's width divided by the nearest whole scale: near 360, not always 360 | 360 exactly |
| Images | `Canvas::load` sets nearest sampling | linear sampling |
| Movement | positions snap to canvas pixels on the screen | positions are smooth |

Load every world image with `Canvas::load(&assets, "images/x.png")`, so the sampling
follows the style.

## Fonts and UI

The UI's `Rem` is one canvas pixel, so `font::SMALL`, `LARGE` and `HUGE` are sizes in
canvas pixels and a UI scales with the phone as the world does. In `Style::Pixel` a pixel
font stays sharp only at whole multiples of its em (Departure Mono: 11). Lengths in `px`
are logical pixels.

## The safe area

`platform::SafeArea` has the insets of the notch, the bars and the home indicator. Put
`InSafeArea` (all sides) or `InSafeTop` on a UI root and it gets the padding. The world
draws under the insets: keep what the player must see or touch out of them. The
`safe-area` scene shows a phone with a notch.
