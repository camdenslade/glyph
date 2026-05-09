use glyph_core::{
    Color, Component, NativeHint, Signal, Theme, View, Widget,
    button, column, text,
};

pub struct Select {
    pub selected: Signal<usize>,
    pub options: Vec<String>,
    pub open: Signal<bool>,
    pub width: f32,
}

impl Select {
    pub fn new(options: Vec<impl Into<String>>, selected: Signal<usize>) -> Self {
        Self {
            selected,
            options: options.into_iter().map(|o| o.into()).collect(),
            open: Signal::new(false),
            width: 200.0,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
}

impl Component for Select {
    fn render(&self, theme: &Theme) -> View {
        let selected_idx = self.selected.get();
        let is_open = self.open.get();
        let label = self.options.get(selected_idx).map(|s| s.as_str()).unwrap_or("");

        let open = self.open.clone();
        let trigger = button(format!("{} ▾", label), move || open.set(!open.get()))
            .bg(theme.surface)
            .text_color(theme.text)
            .radius(theme.radius)
            .font_size(theme.font_size)
            .into();

        if !is_open {
            return column(vec![trigger]).gap(0.0).width(self.width).into();
        }

        let mut items: Vec<View> = vec![trigger];

        let dropdown_items: Vec<View> = self.options.iter().enumerate().map(|(i, opt)| {
            let selected = self.selected.clone();
            let open = self.open.clone();
            let is_selected = i == selected_idx;
            let bg = if is_selected { theme.primary } else { theme.surface };
            let fg = if is_selected { theme.on_primary } else { theme.text };

            button(opt.as_str(), move || {
                selected.set(i);
                open.set(false);
            })
            .bg(bg)
            .text_color(fg)
            .radius(0.0)
            .font_size(theme.font_size)
            .into()
        }).collect();

        items.push(
            column(dropdown_items)
                .gap(0.0)
                .bg(theme.surface)
                .border(theme.border, 1.0)
                .radius(theme.radius)
                .clip()
                .into()
        );

        column(items).gap(2.0).width(self.width).into()
    }
}

impl Widget for Select {
    fn native_hint(&self) -> NativeHint { NativeHint::Select }
    fn focusable(&self) -> bool { true }
}
