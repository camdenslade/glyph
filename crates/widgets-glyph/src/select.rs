use core_glyph::{
    portal, Component, NativeHint, Signal, Theme, View, Widget,
    button, column, rect, Color,
};

pub struct Select {
    pub selected: Signal<usize>,
    pub options:  Vec<String>,
    pub open:     Signal<bool>,
    pub width:    f32,
    /// Y offset from the window top to the bottom of the trigger button.
    /// Set this so the portal dropdown appears in the right position.
    /// If left at 0.0 the dropdown appears at the top of the window.
    pub y_offset: f32,
}

impl Select {
    pub fn new(options: Vec<impl Into<String>>, selected: Signal<usize>) -> Self {
        Self {
            selected,
            options: options.into_iter().map(|o| o.into()).collect(),
            open:    Signal::new(false),
            width:   200.0,
            y_offset: 0.0,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn y_offset(mut self, y: f32) -> Self {
        self.y_offset = y;
        self
    }
}

impl Component for Select {
    fn render(&self, theme: &Theme) -> View {
        let selected_idx = self.selected.get();
        let is_open      = self.open.get();
        let label        = self.options.get(selected_idx).map(|s| s.as_str()).unwrap_or("");

        let open_sig = self.open.clone();
        let trigger: View = button(format!("{} ▾", label), move || open_sig.set(!open_sig.get()))
            .bg(theme.surface)
            .text_color(theme.text)
            .radius(theme.radius)
            .font_size(theme.font_size)
            .width(self.width)
            .into();

        if !is_open {
            return trigger;
        }

        let mut option_items: Vec<View> = Vec::with_capacity(self.options.len());
        for (i, opt) in self.options.iter().enumerate() {
            let selected = self.selected.clone();
            let open     = self.open.clone();
            let is_sel   = i == selected_idx;
            let bg = if is_sel { theme.primary } else { theme.surface };
            let fg = if is_sel { theme.on_primary } else { theme.text };
            option_items.push(
                button(opt.as_str(), move || {
                    selected.set(i);
                    open.set(false);
                })
                .bg(bg)
                .text_color(fg)
                .radius(0.0)
                .font_size(theme.font_size)
                .width(self.width)
                .into(),
            );
        }

        let dropdown: View = column(option_items)
            .gap(1.0)
            .width(self.width)
            .bg(theme.surface)
            .border(theme.border, 1.0)
            .radius(theme.radius)
            .into();

        // The portal renders the dropdown above all content.
        // A transparent spacer of y_offset pixels pushes it to the right Y position.
        let overlay: View = column(vec![
            rect(Color::TRANSPARENT).height(self.y_offset).width(self.width).into(),
            dropdown,
        ])
        .gap(0.0)
        .width(self.width)
        .into();

        column(vec![
            trigger,
            portal(overlay),
        ])
        .gap(0.0)
        .width(self.width)
        .into()
    }
}

impl Widget for Select {
    fn native_hint(&self) -> NativeHint { NativeHint::Select }
    fn focusable(&self) -> bool { true }
}
