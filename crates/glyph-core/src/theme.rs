use crate::view::Color;

/// Semantic color tokens for a UI theme. Components should read from these
/// rather than hardcoding colors so the entire app can be restyled by swapping
/// one `Theme` value.
#[derive(Clone, Debug)]
pub struct Theme {
    /// Window/page background.
    pub background: Color,
    /// Elevated surface (cards, panels, inputs).
    pub surface: Color,
    /// Primary action color (buttons, focus rings, accents).
    pub primary: Color,
    /// Text on primary-colored backgrounds.
    pub on_primary: Color,
    /// Default body text.
    pub text: Color,
    /// Subdued / hint text.
    pub text_muted: Color,
    /// Default border and divider color.
    pub border: Color,
    /// Border color when a field is focused.
    pub border_focused: Color,
    /// Corner radius applied to buttons and inputs.
    pub radius: f32,
    /// Base font size for body text.
    pub font_size: f32,
}

impl Theme {
    /// A clean light theme.
    pub fn light() -> Self {
        Self {
            background:    Color::rgb(1.0, 1.0, 1.0),
            surface:       Color::rgb(0.97, 0.97, 0.97),
            primary:       Color::rgb(0.1, 0.1, 0.9),
            on_primary:    Color::WHITE,
            text:          Color::BLACK,
            text_muted:    Color::rgb(0.5, 0.5, 0.5),
            border:        Color::rgb(0.75, 0.75, 0.75),
            border_focused: Color::rgb(0.2, 0.4, 0.9),
            radius:        8.0,
            font_size:     16.0,
        }
    }

    /// A dark theme.
    pub fn dark() -> Self {
        Self {
            background:    Color::rgb(0.1,  0.1,  0.12),
            surface:       Color::rgb(0.16, 0.16, 0.18),
            primary:       Color::rgb(0.35, 0.55, 1.0),
            on_primary:    Color::WHITE,
            text:          Color::rgb(0.92, 0.92, 0.92),
            text_muted:    Color::rgb(0.55, 0.55, 0.55),
            border:        Color::rgb(0.3,  0.3,  0.32),
            border_focused: Color::rgb(0.35, 0.55, 1.0),
            radius:        8.0,
            font_size:     16.0,
        }
    }
}
