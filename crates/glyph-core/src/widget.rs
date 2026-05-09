use crate::component::Component;
use crate::theme::Theme;
use crate::view::View;

/// Hints to the runtime about what kind of native control this widget maps to.
/// The platform layer uses this to decide whether to render via AppKit/UIKit or
/// fall back to the GPU renderer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NativeHint {
    /// No native equivalent — always render via GPU.
    None,
    /// Maps to a checkbox control (NSButton with switch style on macOS).
    Checkbox,
    /// Maps to a toggle/switch control.
    Toggle,
    /// Maps to a segmented control or radio group.
    RadioGroup,
    /// Maps to a slider control.
    Slider,
    /// Maps to a popup button / dropdown.
    Select,
    /// Maps to a color well.
    ColorPicker,
    /// Maps to a date picker.
    DatePicker,
}

/// Extension of `Component` that carries platform metadata.
///
/// Implement `Widget` to opt in to native platform controls when available.
/// The default implementations delegate everything to `Component::render`,
/// so you only need to override what you care about.
///
/// # Implementing a custom widget
///
/// ```rust
/// use glyph_core::{Component, Signal, Theme, View, NativeHint, Widget, column, text};
///
/// struct MyToggle { on: Signal<bool> }
///
/// impl Component for MyToggle {
///     fn render(&self, theme: &Theme) -> View {
///         // GPU fallback rendering
///         column(vec![text(if self.on.get() { "ON" } else { "OFF" }, 16.0).into()]).into()
///     }
/// }
///
/// impl Widget for MyToggle {
///     fn native_hint(&self) -> NativeHint { NativeHint::Toggle }
/// }
/// ```
pub trait Widget: Component {
    /// What native control this widget resembles, if any.
    fn native_hint(&self) -> NativeHint {
        NativeHint::None
    }

    /// Accessible label for screen readers.
    fn accessibility_label(&self) -> Option<&str> {
        None
    }

    /// Whether this widget can receive keyboard focus.
    fn focusable(&self) -> bool {
        false
    }

    /// Render via GPU (the default path). Override to customize the GPU appearance
    /// independently of the native appearance.
    fn render_gpu(&self, theme: &Theme) -> View {
        self.render(theme)
    }
}
