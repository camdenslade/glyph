use core_glyph::{
    Color, Component, NativeHint, Signal, Theme, View, Widget,
    button, row, text,
};

pub struct Toggle {
    pub on:    Signal<bool>,
    pub label: String,
}

impl Toggle {
    pub fn new(label: impl Into<String>, on: Signal<bool>) -> Self {
        Self { on, label: label.into() }
    }
}

impl Component for Toggle {
    fn render(&self, theme: &Theme) -> View {
        let on    = self.on.clone();
        let is_on = self.on.get();

        let track_color  = if is_on { theme.primary } else { theme.border };
        let knob_label   = if is_on { "●" } else { "○" };
        let knob_w = theme.font_size * 2.5;
        let knob_h = theme.font_size + 8.0;

        row(vec![
            text(&self.label, theme.font_size).color(theme.text).into(),
            button(knob_label, move || on.set(!on.get()))
                .bg(track_color)
                .text_color(Color::WHITE)
                .radius(knob_h * 0.5)
                .font_size(theme.font_size)
                .width(knob_w)
                .height(knob_h)
                .into(),
        ])
        .gap(8.0)
        .auto_size()
        .into()
    }
}

impl Widget for Toggle {
    fn native_hint(&self) -> NativeHint { NativeHint::Toggle }
    fn accessibility_label(&self) -> Option<&str> { Some(&self.label) }
    fn focusable(&self) -> bool { true }
}
