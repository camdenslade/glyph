//! Core types shared across all glyph crates.
//!
//! Provides the `View` tree (the declarative UI description), `Signal<T>` for
//! reactive state, `Component` for composable widgets, `Theme` for semantic
//! color tokens, and `ViewTree` which runs Taffy layout and produces a flat list
//! of pixel-positioned quads ready for the GPU renderer.

pub mod anim;
mod component;
mod layout;
pub mod node;
mod router;
mod signal;
mod theme;
mod view;
mod widget;

pub use anim::{Easing, Tween, tick_tweens};
pub use component::Component;
pub use widget::{NativeHint, Widget};
pub use taffy::{AlignItems, JustifyContent};
pub use layout::{FlatView, FlatViewKind, ViewTree};
pub use taffy::Layout as TaffyLayout;
pub use signal::{Signal, clear_redraw, needs_redraw};
pub use theme::Theme;
pub use view::{
    ButtonView, Color, ColumnView, FontFamily, FontWeight, ImageView, Lerp, RectView, RowView, ScrollView, Shadow,
    TextAlign, TextAreaView, TextInputView, TextView, View, VirtualListView, ZStackView,
    button, column, flex, flexible, image, list, opacity, rect, row, scroll, spacer, text, text_area, text_input, virtual_list, zstack,
};
pub use router::Router;
pub use node::{LayoutNode, LayoutProps, NodeKind, StyleProps};
