#![allow(clippy::wrong_self_convention)]
use crate::theme::Theme;
use crate::view::View;

/// A self-contained UI component that owns its state and knows how to render itself.
///
/// Implement this on any struct that holds `Signal` fields. `render` receives the
/// active `Theme` so components can read semantic color tokens instead of
/// hardcoding values. It is called every frame and should return a fresh `View`
/// tree.
///
/// # Example
///
/// ```rust
/// use glyph_core::{Component, Signal, Theme, View, button, column, text};
///
/// struct Counter { count: Signal<i32> }
///
/// impl Component for Counter {
///     fn render(&self, theme: &Theme) -> View {
///         let count = self.count.clone();
///         column(vec![
///             text(format!("{}", self.count.get()), 32.0).into(),
///             button("Add", move || count.set(count.get() + 1))
///                 .bg(theme.primary)
///                 .text_color(theme.on_primary)
///                 .radius(theme.radius)
///                 .into(),
///         ]).into()
///     }
/// }
/// ```
pub trait Component {
    fn render(&self, theme: &Theme) -> View;

    /// Render with the given theme and return the result. Convenience for
    /// embedding a component inside another component's `render`.
    fn into_view(&self, theme: &Theme) -> View {
        self.render(theme)
    }
}
