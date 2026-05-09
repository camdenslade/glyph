use glyph_core::{
    Component, NativeHint, Signal, Theme, View, Widget,
    column, row, rect, text,
};

pub struct Slider {
    pub value: Signal<f32>,
    pub min: f32,
    pub max: f32,
    pub label: String,
    pub width: f32,
}

impl Slider {
    pub fn new(label: impl Into<String>, value: Signal<f32>, min: f32, max: f32) -> Self {
        Self { value, min, max, label: label.into(), width: 200.0 }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
}

impl Component for Slider {
    fn render(&self, theme: &Theme) -> View {
        let val = self.value.get().clamp(self.min, self.max);
        let pct = (val - self.min) / (self.max - self.min);
        let fill_w = (self.width * pct).max(0.0);

        let value = self.value.clone();
        let min = self.min;
        let max = self.max;
        let width = self.width;

        let track = glyph_core::zstack(vec![
            rect(theme.border).into(),
            column(vec![]).width(fill_w).bg(theme.primary).into(),
        ]);

        column(vec![
            row(vec![
                text(&self.label, theme.font_size).color(theme.text).into(),
                glyph_core::spacer(),
                text(format!("{:.1}", val), theme.font_size * 0.9)
                    .color(theme.text_muted)
                    .into(),
            ])
            .into(),
            track,
            row(vec![
                button_step("-", {
                    let value = value.clone();
                    move || {
                        let step = (max - min) / 20.0;
                        value.set((value.get() - step).max(min));
                    }
                }, theme),
                glyph_core::spacer(),
                button_step("+", {
                    let value = value.clone();
                    move || {
                        let step = (max - min) / 20.0;
                        value.set((value.get() + step).min(max));
                    }
                }, theme),
            ])
            .into(),
        ])
        .gap(6.0)
        .width(width)
        .into()
    }
}

fn button_step(label: &str, f: impl Fn() + 'static, theme: &Theme) -> View {
    glyph_core::button(label, f)
        .bg(theme.surface)
        .text_color(theme.text)
        .radius(theme.radius * 0.5)
        .font_size(theme.font_size)
        .into()
}

impl Widget for Slider {
    fn native_hint(&self) -> NativeHint { NativeHint::Slider }
    fn accessibility_label(&self) -> Option<&str> { Some(&self.label) }
    fn focusable(&self) -> bool { true }
}
