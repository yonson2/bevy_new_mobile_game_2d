//! Widgets as BSN scenes, from bevy_new_2d (`src/theme/widget.rs`).

use std::borrow::Cow;

use bevy::{ecs::system::IntoObserverSystem, prelude::*};

use super::{font, interaction::InteractionPalette, palette::*};
use crate::platform::InSafeArea;

/// A button's look: a thin edge with a deeper lip below.
const BUTTON_BORDER: UiRect = UiRect {
    left: Val::Rem(2.),
    right: Val::Rem(2.),
    top: Val::Rem(2.),
    bottom: Val::Rem(4.),
};
const BUTTON_RADIUS: Val = Val::Rem(4.);

/// A root UI node that fills the window and centers its content in a column, inside the
/// safe area.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Scene {
    let name = name.into();
    bsn! {
        Name::new(name)
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
        }
        // Other UI roots still get the picking events.
        Pickable::IGNORE
        InSafeArea
    }
}

/// A header, larger than [`label`].
pub fn header(text: impl Into<String>) -> impl Scene {
    bsn! {
        #Header
        Text(text)
        TextFont { font_size: {font::HUGE} }
        TextColor(HEADER_TEXT)
        TextLayout { justify: Justify::Center }
    }
}

/// A text label.
pub fn label(text: impl Into<String>) -> impl Scene {
    bsn! {
        #Label
        Text(text)
        TextFont { font_size: {font::LARGE} }
        TextColor(LABEL_TEXT)
        TextLayout { justify: Justify::Center }
    }
}

/// A small rounded button with text, for a bar: 44 logical pixels tall, the touch
/// minimum; a press and a release on it run `action`, as [`button`].
pub fn chip<E, M, I>(text: impl Into<String>, action: I) -> impl Scene
where
    E: EventPattern<Event: EntityEvent>,
    I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    M: 'static,
{
    let text = text.into();
    bsn! {
        #Chip
        Node {
            height: px(44),
            padding: {UiRect::horizontal(px(14))},
            align_items: AlignItems::Center,
            border: {BUTTON_BORDER},
            border_radius: {BorderRadius::all(BUTTON_RADIUS)},
        }
        BackgroundColor(BUTTON_BACKGROUND)
        BorderColor::all(BUTTON_EDGE)
        InteractionPalette {
            none: BUTTON_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        }
        Children [
            #ChipText
            Text(text)
            TextFont { font_size: {font::LARGE} }
            TextColor(BUTTON_TEXT)
            TextLayout { linebreak: LineBreak::NoWrap }
            Pickable::IGNORE
        ]
        @on(action)
    }
}

/// The value text of a [`row`].
#[derive(Component, Default, Clone)]
pub struct RowValue;

/// A wide button with a name on the left and a value on the right, a [`RowValue`] that
/// starts empty: a row of a settings list. A press and a release on it run `action`, as
/// [`button`].
pub fn row<E, M, I>(name: impl Into<String>, action: I) -> impl Scene
where
    E: EventPattern<Event: EntityEvent>,
    I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    M: 'static,
{
    let name = name.into();
    bsn! {
        #Row
        Node {
            width: px(340),
            height: px(60),
            padding: {UiRect::horizontal(px(22))},
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            border: {BUTTON_BORDER},
            border_radius: {BorderRadius::all(BUTTON_RADIUS)},
        }
        BackgroundColor(BUTTON_BACKGROUND)
        BorderColor::all(BUTTON_EDGE)
        InteractionPalette {
            none: BUTTON_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        }
        Children [
            #RowName
            Text(name)
            TextFont { font_size: {font::LARGE} }
            TextColor(BUTTON_TEXT)
            Pickable::IGNORE
            --
            #RowValue
            RowValue
            Text("")
            TextFont { font_size: {font::LARGE} }
            TextColor(HEADER_TEXT)
            Pickable::IGNORE
        ]
        @on(action)
    }
}

/// A rounded button with text; a press and a release on it run `action`, an
/// [`Observer`]. 260 x 72 logical pixels, well over the 44 pt touch minimum.
pub fn button<E, M, I>(text: impl Into<String>, action: I) -> impl Scene
where
    E: EventPattern<Event: EntityEvent>,
    I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    M: 'static,
{
    let text = text.into();
    bsn! {
        #Button
        Node
        Children [
            #ButtonInner
            Node {
                width: px(260),
                height: px(72),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: {BUTTON_BORDER},
                border_radius: {BorderRadius::all(BUTTON_RADIUS)},
            }
            BackgroundColor(BUTTON_BACKGROUND)
            BorderColor::all(BUTTON_EDGE)
            InteractionPalette {
                none: BUTTON_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            }
            Children [
                #ButtonText
                Text(text)
                TextFont { font_size: {font::LARGE} }
                TextColor(BUTTON_TEXT)
                // The text passes its picking events to the button, not up the tree.
                Pickable::IGNORE
            ]
            @on(action)
        ]
    }
}
