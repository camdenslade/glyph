use glyph_core::{
    Color, Component, NativeHint, Signal, Theme, View, Widget,
    button, row, text,
};

pub struct Checkbox {
    pub checked: Signal<bool>,
    pub label: String,
}

impl Checkbox {
    pub fn new(label: impl Into<String>, checked: Signal<bool>) -> Self {
        Self { checked, label: label.into() }
    }
}

impl Component for Checkbox {
    fn render(&self, theme: &Theme) -> View {
        let checked = self.checked.clone();
        let is_checked = self.checked.get();

        let tick_color = if is_checked { theme.on_primary } else { Color::TRANSPARENT };
        let box_bg = if is_checked { theme.primary } else { theme.surface };
        let _box_border = if is_checked { theme.primary } else { theme.border };

        row(vec![
            button(if is_checked { "✓" } else { " " }, move || {
                checked.set(!checked.get());
            })
            .bg(box_bg)
            .text_color(tick_color)
            .radius(theme.radius * 0.5)
            .font_size(theme.font_size * 0.9)
            .into(),
            text(&self.label, theme.font_size).color(theme.text).into(),
        ])
        .gap(8.0)
        .into()
    }
}

impl Widget for Checkbox {
    fn native_hint(&self) -> NativeHint { NativeHint::Checkbox }
    fn accessibility_label(&self) -> Option<&str> { Some(&self.label) }
    fn focusable(&self) -> bool { true }
}
