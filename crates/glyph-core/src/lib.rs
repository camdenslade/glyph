//! Core types shared across all glyph crates.
//!
//! Provides the `View` tree (the declarative UI description), `Signal<T>` for
//! reactive state, `Component` for composable widgets, `Theme` for semantic
//! color tokens, and `ViewTree` which runs Taffy layout and produces a flat list
//! of pixel-positioned quads ready for the GPU renderer.

mod component;
mod layout;
mod signal;
mod theme;
mod view;
mod widget;

pub use component::Component;
pub use widget::{NativeHint, Widget};
pub use taffy::{AlignItems, JustifyContent};
pub use layout::{FlatView, FlatViewKind, ViewTree};
pub use signal::{Signal, clear_redraw, needs_redraw};
pub use theme::Theme;
pub use view::{
    ButtonView, Color, ColumnView, FontWeight, ImageView, RowView, ScrollView, Shadow, TextAlign,
    TextInputView, TextView, View,
    button, column, flex, flexible, image, list, rect, row, scroll, spacer, text, text_input, zstack,
};
