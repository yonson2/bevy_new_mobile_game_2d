//! The canvas: how the world reaches the screen. The screen is about [`WIDTH`] canvas
//! pixels wide on every phone, and its height follows the phone's shape. A canvas pixel is
//! the unit of the art and of the fonts (`Rem`, `theme::font`).
//!
//! The world's origin is the canvas's bottom-left corner, and one world unit is [`ZOOM`]
//! canvas pixels. [`STYLE`] picks the rig:
//!
//! - [`Style::Pixel`]: the world camera draws render layer 0 into a low-resolution image;
//!   the screen camera draws that image (render layer 1) at a whole-number scale with
//!   nearest sampling, and the UI.
//! - [`Style::Smooth`]: one camera draws the world and the UI at the screen's resolution.

use bevy::{
    camera::{RenderTarget, ScalingMode, visibility::RenderLayers},
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
    text::RemSize,
};

use crate::AppSystems;

/// How the world reaches the screen.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Style {
    /// Pixel art: each canvas pixel covers the same whole number of screen pixels.
    Pixel,
    /// Any other 2D art: a canvas pixel is a length, not a square of pixels.
    Smooth,
}

/// The game's style. The art direction decides it (`docs/rendering.md`).
pub const STYLE: Style = Style::Pixel;

/// The canvas width that the scale aims for, in canvas pixels.
const WIDTH: u32 = 360;
/// Canvas pixels a world unit.
pub const ZOOM: f32 = 2.;
/// The layer of the canvas sprite: only the screen camera draws it.
const SCREEN_LAYER: usize = 1;

pub(super) fn plugin(app: &mut App) {
    #[cfg(feature = "dev_tools")]
    let style = crate::dev_tools::style().unwrap_or(STYLE);
    #[cfg(not(feature = "dev_tools"))]
    let style = STYLE;
    app.insert_resource(Canvas {
        style,
        image: default(),
        fit: default(),
        target: UVec2::ONE,
        scale_factor: 1.,
    });
    app.add_systems(Startup, spawn_canvas);
    app.add_systems(Update, fit_canvas.in_set(AppSystems::Update));
}

/// The canvas and its fit to the screen.
#[derive(Resource)]
pub struct Canvas {
    pub style: Style,
    /// What the world camera draws into (`Style::Pixel`).
    image: Handle<Image>,
    pub fit: Fit,
    /// The screen's size in physical pixels.
    target: UVec2,
    /// The screen's physical pixels per logical pixel.
    scale_factor: f32,
}

impl Canvas {
    /// The canvas's size in world units.
    pub fn world_size(&self) -> Vec2 {
        self.fit.size / ZOOM
    }

    /// A move on the window, in logical pixels with y down, in world units with y up.
    pub fn window_to_world(&self, delta: Vec2) -> Vec2 {
        Vec2::new(delta.x, -delta.y) / self.world_pixel()
    }

    /// A world unit's side on the window, in logical pixels.
    pub fn world_pixel(&self) -> f32 {
        self.fit.scale * ZOOM / self.scale_factor
    }

    /// Loads an image of the game's art: with nearest sampling in `Style::Pixel`.
    pub fn load(&self, assets: &AssetServer, path: &'static str) -> Handle<Image> {
        let pixel = self.style == Style::Pixel;
        assets
            .load_builder()
            .with_settings(move |settings: &mut ImageLoaderSettings| {
                if pixel {
                    settings.sampler = ImageSampler::nearest();
                }
            })
            .load(path)
    }

    #[cfg(test)]
    pub fn new_for_test(style: Style, target: UVec2, scale_factor: f32) -> Self {
        Self {
            style,
            image: default(),
            fit: fit(style, target),
            target,
            scale_factor,
        }
    }
}

/// The canvas on a screen: the screen pixels a canvas pixel, and the canvas's size in
/// canvas pixels. Whole numbers in `Style::Pixel`.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Fit {
    pub scale: f32,
    pub size: Vec2,
}

/// The fit for a screen in physical pixels. `Style::Pixel`: the whole-number scale that
/// puts the canvas width nearest [`WIDTH`], and the canvas size that fits the screen at
/// that scale; the pixels left over, fewer than `scale` on each axis, are a border in the
/// clear color. `Style::Smooth`: the canvas is [`WIDTH`] wide and fills the screen.
pub fn fit(style: Style, screen: UVec2) -> Fit {
    match style {
        Style::Pixel => {
            let low = (screen.x / WIDTH).max(1);
            let nearer = (screen.x / (low + 1)).abs_diff(WIDTH) < (screen.x / low).abs_diff(WIDTH);
            let scale = if nearer { low + 1 } else { low };
            Fit {
                scale: scale as f32,
                size: (screen / scale).as_vec2(),
            }
        }
        Style::Smooth => {
            let scale = screen.x as f32 / WIDTH as f32;
            Fit {
                scale,
                size: screen.as_vec2() / scale,
            }
        }
    }
}

/// The camera that draws the world. Its translation is the world point at the canvas's
/// bottom-left corner.
#[derive(Component, Default, Clone)]
pub struct WorldCamera;

/// The camera that draws to the screen, and the UI.
#[derive(Component, Default, Clone)]
pub struct ScreenCamera;

#[derive(Component, Default, Clone)]
struct CanvasSprite;

fn spawn_canvas(
    mut commands: Commands,
    mut canvas: ResMut<Canvas>,
    mut images: ResMut<Assets<Image>>,
) {
    if canvas.style == Style::Smooth {
        commands.spawn_scene(smooth_camera());
        return;
    }
    // 1 x 1 until `fit_canvas` knows the screen.
    let mut image = Image::new_target_texture(1, 1, TextureFormat::Rgba8UnormSrgb, None);
    image.sampler = ImageSampler::nearest();
    canvas.image = images.add(image);
    commands.spawn_scene(canvas_camera(canvas.image.clone()));
    commands.spawn_scene(canvas_sprite(canvas.image.clone()));
    commands.spawn_scene(screen_camera());
}

fn canvas_camera(image: Handle<Image>) -> impl Scene {
    let projection = Projection::Orthographic(OrthographicProjection {
        viewport_origin: Vec2::ZERO,
        scale: 1. / ZOOM,
        ..OrthographicProjection::default_2d()
    });
    bsn! {
        #WorldCamera
        WorldCamera
        Camera2d
        // Before the screen camera, which shows the canvas.
        Camera { order: -1 }
        // `RenderTarget`'s template takes no handle: `template` builds the value.
        template(move |_| Ok(RenderTarget::Image(image.clone().into())))
        projection
        Msaa::Off
    }
}

fn canvas_sprite(image: Handle<Image>) -> impl Scene {
    bsn! {
        #CanvasSprite
        CanvasSprite
        Sprite { image: {image} }
        RenderLayers::layer(SCREEN_LAYER)
    }
}

fn screen_camera() -> impl Scene {
    bsn! {
        #ScreenCamera
        ScreenCamera
        Camera2d
        RenderLayers::layer(SCREEN_LAYER)
    }
}

/// `Style::Smooth`: the screen is as wide as the canvas, in world units, at any size.
fn smooth_camera() -> impl Scene {
    let projection = Projection::Orthographic(OrthographicProjection {
        viewport_origin: Vec2::ZERO,
        scaling_mode: ScalingMode::FixedHorizontal {
            viewport_width: WIDTH as f32 / ZOOM,
        },
        ..OrthographicProjection::default_2d()
    });
    bsn! {
        #WorldCamera
        WorldCamera
        ScreenCamera
        Camera2d
        projection
    }
}

/// Fits the canvas to the screen camera's target, the window or a shot's image, and
/// follows its changes (a resize, Android split screen).
fn fit_canvas(
    screen: Single<&Camera, With<ScreenCamera>>,
    mut canvas: ResMut<Canvas>,
    mut images: ResMut<Assets<Image>>,
    sprite: Option<Single<(&mut Sprite, &mut Transform), With<CanvasSprite>>>,
    mut rem: ResMut<RemSize>,
) {
    let (Some(target), Some(scale_factor)) = (
        screen.physical_target_size(),
        screen.target_scaling_factor(),
    ) else {
        return;
    };
    // A minimised window.
    if target.min_element() == 0 {
        return;
    }
    if canvas.scale_factor != scale_factor {
        canvas.scale_factor = scale_factor;
    }
    if canvas.target != target {
        canvas.target = target;
    }
    let fit = fit(canvas.style, target);
    if fit != canvas.fit {
        canvas.fit = fit;
        if let Some(mut image) = images.get_mut(&canvas.image) {
            image.resize(Extent3d {
                width: fit.size.x as u32,
                height: fit.size.y as u32,
                ..default()
            });
        }
    }
    // The fonts' unit: a canvas pixel's side, in logical pixels.
    let pixel = fit.scale / scale_factor;
    if rem.0 != pixel {
        rem.0 = pixel;
    }
    let Some(sprite) = sprite else {
        return;
    };
    // The sprite's edges land on whole physical pixels: the border's odd pixel goes to
    // the right and the bottom.
    let border = target - (fit.size * fit.scale).as_uvec2();
    let size = fit.size * fit.scale / scale_factor;
    let center = Vec2::new(-((border.x % 2) as f32), (border.y % 2) as f32) / (2. * scale_factor);
    let (mut sprite, mut transform) = sprite.into_inner();
    if sprite.custom_size != Some(size) {
        sprite.custom_size = Some(size);
    }
    if transform.translation.xy() != center {
        transform.translation = center.extend(0.);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Common phone screens.
    const SCREENS: [UVec2; 8] = [
        UVec2::new(720, 1600),
        UVec2::new(1080, 2340),
        UVec2::new(1080, 2400),
        UVec2::new(1125, 2436),
        UVec2::new(1170, 2532),
        UVec2::new(1179, 2556),
        UVec2::new(1290, 2796),
        UVec2::new(1440, 3120),
    ];

    #[test]
    fn phones_get_a_pixel_canvas_near_360_wide() {
        for screen in SCREENS {
            let fit = fit(Style::Pixel, screen);
            assert!((320. ..=440.).contains(&fit.size.x), "{screen}: {fit:?}");
            assert_eq!(fit.scale.fract(), 0., "{screen}: {fit:?}");
            assert_eq!(fit.size.fract(), Vec2::ZERO, "{screen}: {fit:?}");
            let border = screen.as_vec2() - fit.size * fit.scale;
            assert!(border.max_element() < fit.scale, "{screen}: {fit:?}");
        }
    }

    #[test]
    fn the_pixel_scale_is_the_nearest() {
        let pixel = |w, h| fit(Style::Pixel, UVec2::new(w, h));
        assert_eq!(
            pixel(1080, 2400),
            Fit {
                scale: 3.,
                size: Vec2::new(360., 800.)
            }
        );
        // 1290 / 4 = 322 is nearer 360 than 1290 / 3 = 430.
        assert_eq!(pixel(1290, 2796).scale, 4.);
        assert_eq!(
            pixel(1125, 2436),
            Fit {
                scale: 3.,
                size: Vec2::new(375., 812.)
            }
        );
        assert_eq!(
            pixel(120, 200),
            Fit {
                scale: 1.,
                size: Vec2::new(120., 200.)
            }
        );
    }

    #[test]
    fn a_smooth_canvas_is_360_wide_and_fills_the_screen() {
        for screen in SCREENS {
            let fit = fit(Style::Smooth, screen);
            assert_eq!(fit.size.x, 360., "{screen}");
            assert!(
                (fit.size * fit.scale - screen.as_vec2())
                    .abs()
                    .max_element()
                    < 1e-3
            );
        }
    }

    #[test]
    fn a_move_on_the_window_is_a_move_in_the_world() {
        for style in [Style::Pixel, Style::Smooth] {
            // 1080 x 2400 at scale 3 is the screen in both styles: no border. A world
            // unit is 2 canvas pixels.
            let canvas = Canvas::new_for_test(style, UVec2::new(1080, 2400), 3.);
            assert_eq!(canvas.world_size(), Vec2::new(180., 400.));
            assert_eq!(canvas.world_pixel(), 2.);
            assert_eq!(
                canvas.window_to_world(Vec2::new(2., 4.)),
                Vec2::new(1., -2.)
            );
        }
    }
}
