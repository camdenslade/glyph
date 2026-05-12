use core_glyph::{
    AlignItems, Color, Component, NativeHint, Signal, Theme, View, Widget,
    button, column, row, text,
};

pub struct RadioGroup {
    pub selected: Signal<usize>,
    pub options: Vec<String>,
}

impl RadioGroup {
    pub fn new(options: Vec<impl Into<String>>, selected: Signal<usize>) -> Self {
        Self {
            selected,
            options: options.into_iter().map(|o| o.into()).collect(),
        }
    }
}

impl Component for RadioGroup {
    fn render(&self, theme: &Theme) -> View {
        let selected_idx = self.selected.get();

        let items: Vec<View> = self.options.iter().enumerate().map(|(i, label)| {
            let selected = self.selected.clone();
            let is_selected = i == selected_idx;
            let dot_color = if is_selected { theme.primary } else { theme.border };

            row(vec![
                button(if is_selected { "●" } else { "○" }, move || selected.set(i))
                    .bg(Color::TRANSPARENT)
                    .text_color(dot_color)
                    .font_size(theme.font_size)
                    .into(),
                text(label, theme.font_size).color(theme.text).into(),
            ])
            .gap(6.0)
            .into()
        }).collect();

        column(items).gap(8.0).align(AlignItems::FlexStart).into()
    }
}

impl Widget for RadioGroup {
    fn native_hint(&self) -> NativeHint { NativeHint::RadioGroup }
    fn focusable(&self) -> bool { true }
}
