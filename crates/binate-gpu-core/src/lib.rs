//! Core types shared across all binate-gpu crates.
//!
//! Provides the `View` tree (the declarative UI description), `Signal<T>` for
//! reactive state, and `ViewTree` which runs Taffy layout and produces a flat
//! list of pixel-positioned quads ready for the GPU renderer.

mod layout;
mod signal;
mod view;

pub use layout::{FlatView, FlatViewKind, ViewTree};
pub use signal::{Signal, clear_redraw, needs_redraw};
pub use view::{
    ButtonView, Color, FontWeight, TextAlign, TextView, View,
    button, column, rect, row, text,
};
