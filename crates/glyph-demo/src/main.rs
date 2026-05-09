use glyph_core::{
    Component, FontWeight, Signal, Theme, View,
    button, column, image, row, text, text_input,
};
use glyph_platform::App;

struct SearchBox {
    value: Signal<String>,
    focused: Signal<bool>,
}

impl SearchBox {
    fn new() -> Self {
        Self {
            value: Signal::new(String::new()),
            focused: Signal::new(false),
        }
    }
}

impl Component for SearchBox {
    fn render(&self, theme: &Theme) -> View {
        column(vec![
            text_input(self.value.clone(), self.focused.clone())
                .placeholder("Type something...")
                .bg(theme.surface)
                .text_color(theme.text)
                .border_color(theme.border)
                .radius(theme.radius)
                .font_size(theme.font_size)
                .width(320.0)
                .into(),
            text(
                if self.value.get().is_empty() {
                    "Nothing typed yet.".to_string()
                } else {
                    format!("You typed: {}", self.value.get())
                },
                theme.font_size,
            )
            .color(theme.text_muted)
            .into(),
        ]).into()
    }
}

struct Counter {
    count: Signal<i32>,
}

impl Counter {
    fn new() -> Self {
        Self { count: Signal::new(0) }
    }
}

impl Component for Counter {
    fn render(&self, theme: &Theme) -> View {
        let count = self.count.clone();
        row(vec![
            text(format!("Count: {}", self.count.get()), 32.0)
                .weight(FontWeight::Bold)
                .color(theme.text)
                .into(),
            button("Increment", move || count.set(count.get() + 1))
                .bg(theme.primary)
                .text_color(theme.on_primary)
                .radius(theme.radius)
                .into(),
        ]).into()
    }
}

struct DemoApp {
    counter: Counter,
    search: SearchBox,
    dark_mode: Signal<bool>,
}

impl DemoApp {
    fn new() -> Self {
        Self {
            counter: Counter::new(),
            search: SearchBox::new(),
            dark_mode: Signal::new(false),
        }
    }
}

impl Component for DemoApp {
    fn render(&self, theme: &Theme) -> View {
        let dark_mode = self.dark_mode.clone();
        let label = if self.dark_mode.get() { "Switch to Light" } else { "Switch to Dark" };

        column(vec![
            image("Glyph.png").size(120.0, 120.0).radius(12.0).into(),
            self.counter.into_view(theme),
            self.search.into_view(theme),
            button(label, move || dark_mode.set(!dark_mode.get()))
                .bg(theme.surface)
                .text_color(theme.text)
                .radius(theme.radius)
                .into(),
        ]).into()
    }
}

fn main() {
    let app = DemoApp::new();
    let dark_signal = app.dark_mode.clone();

    App::run(move |_theme| {
        let theme = if dark_signal.get() { Theme::dark() } else { Theme::light() };
        app.render(&theme)
    });
}
