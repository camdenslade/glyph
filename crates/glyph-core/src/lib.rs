//! Core types shared across all glyph crates.
//!
//! Provides the `View` tree (the declarative UI description), `Signal<T>` for
//! reactive state, `Component` for composable widgets, `Theme` for semantic
//! color tokens, and `ViewTree` which runs Taffy layout and produces a flat list
//! of pixel-positioned quads ready for the GPU renderer.

pub mod anim;
mod component;
mod layout;
mod signal;
mod theme;
mod view;
mod widget;

pub use anim::{Easing, Tween, tick_tweens};
pub use component::Component;
pub use widget::{NativeHint, Widget};
pub use taffy::{AlignItems, JustifyContent};
pub use layout::{FlatView, FlatViewKind, ViewTree};
pub use signal::{Signal, clear_redraw, needs_redraw};
pub use theme::Theme;
pub use view::{
    ButtonView, Color, ColumnView, FontWeight, ImageView, Lerp, RowView, ScrollView, Shadow,
    TextAlign, TextInputView, TextView, View, ZStackView,
    button, column, flex, flexible, image, list, rect, row, scroll, spacer, text, text_input, zstack,
};
