use glyph_core::{
    Color, Component, Easing, FontWeight, Signal, Theme, Tween, View,
    button, column, image, opacity, row, text, text_input,
};
use glyph_platform::{App, WindowOpener};
use glyph_widgets::{Checkbox, Select, Toggle};

struct SearchBox {
    value:   Signal<String>,
    focused: Signal<bool>,
    cursor:  Signal<usize>,
}

impl SearchBox {
    fn new() -> Self {
        Self {
            value:   Signal::new(String::new()),
            focused: Signal::new(false),
            cursor:  Signal::new(0),
        }
    }
}

impl Component for SearchBox {
    fn render(&self, theme: &Theme) -> View {
        column(vec![
            text_input(self.value.clone(), self.focused.clone(), self.cursor.clone())
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
        ])
        .auto_size()
        .gap(8.0)
        .into()
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
                .width(180.0)
                .into(),
            button("Increment", move || count.set(count.get() + 1))
                .bg(theme.primary)
                .text_color(theme.on_primary)
                .radius(theme.radius)
                .into(),
        ])
        .auto_size()
        .gap(16.0)
        .padding(16.0)
        .bg(theme.surface)
        .radius(theme.radius)
        .shadow(glyph_core::Shadow::new(0.0, 1.0, 8.0, Color::rgba(0.0, 0.0, 0.0, 0.15)))
        .into()
    }
}

/// A button with hover and press color animations.
struct AnimatedButton {
    bg:          Signal<Color>,
    tween:       Tween<Color>,
    hover_color: Signal<Color>,  // tracks the hover target so press can return to it
}

impl AnimatedButton {
    fn new(theme: &Theme) -> Self {
        let bg = Signal::new(theme.primary);
        let tween = Tween::new(bg.clone(), Easing::EaseOut, 0.15);
        Self { bg, tween, hover_color: Signal::new(theme.primary) }
    }
}

impl Component for AnimatedButton {
    fn render(&self, theme: &Theme) -> View {
        let bg_color = self.bg.get();
        let rest    = theme.primary;
        let hovered = Color::rgb(
            (rest.r * 1.25).min(1.0),
            (rest.g * 1.25).min(1.0),
            (rest.b * 1.25).min(1.0),
        );
        let pressed = Color::rgb(
            (rest.r * 0.75).min(1.0),
            (rest.g * 0.75).min(1.0),
            (rest.b * 0.75).min(1.0),
        );

        let t1 = self.tween.clone();
        let t3 = self.tween.clone();
        let t4 = self.tween.clone();
        let hc = self.hover_color.clone();
        let hc2 = self.hover_color.clone();

        button("Hover me!", || {})
            .bg(bg_color)
            .text_color(theme.on_primary)
            .radius(theme.radius)
            .font_size(theme.font_size)
            .on_hover(move |is_hovered| {
                hc.set(if is_hovered { hovered } else { rest });
                t1.start(if is_hovered { hovered } else { rest });
            })
            .on_press(move |is_pressed| {
                if is_pressed {
                    t3.start(pressed);
                } else {
                    t4.start(hc2.get());
                }
            })
            .into()
    }
}

struct WidgetShowcase {
    show_counter: Checkbox,
    dark_toggle:  Toggle,
    theme_select: Select,
}

impl WidgetShowcase {
    fn new(dark_mode: Signal<bool>) -> Self {
        Self {
            show_counter: Checkbox::new("Show counter", Signal::new(true)),
            dark_toggle:  Toggle::new("Dark mode", dark_mode),
            theme_select: Select::new(vec!["Light", "Dark", "System"], Signal::new(0)),
        }
    }
}

impl Component for WidgetShowcase {
    fn render(&self, theme: &Theme) -> View {
        column(vec![
            self.show_counter.into_view(theme),
            self.dark_toggle.into_view(theme),
            self.theme_select.into_view(theme),
        ])
        .gap(12.0)
        .auto_size()
        .into()
    }
}

struct DemoApp {
    counter:       Counter,
    search:        SearchBox,
    anim_btn:      AnimatedButton,
    widgets:       WidgetShowcase,
    dark_mode:     Signal<bool>,
    // Entrance animation — signals and tweens kept alive for the app lifetime.
    opacities:     Vec<Signal<f32>>,
    op_tweens:     Vec<Tween<f32>>,
    entered:       Signal<bool>,
}

impl DemoApp {
    fn new() -> Self {
        let theme = Theme::light();
        let dark_mode = Signal::new(false);
        let opacities: Vec<Signal<f32>> = (0..4).map(|_| Signal::new(0.0f32)).collect();
        let op_tweens: Vec<Tween<f32>> = opacities.iter().enumerate().map(|(i, sig)| {
            let duration = 0.3 + i as f32 * 0.12;
            Tween::new(sig.clone(), Easing::EaseOut, duration)
        }).collect();
        let entered = Signal::new(false);
        Self {
            counter:   Counter::new(),
            search:    SearchBox::new(),
            anim_btn:  AnimatedButton::new(&theme),
            widgets:   WidgetShowcase::new(dark_mode.clone()),
            dark_mode,
            opacities,
            op_tweens,
            entered,
        }
    }

    fn trigger_entrance(&self) {
        for tween in &self.op_tweens {
            tween.animate(0.0, 1.0);
        }
    }
}

impl Component for DemoApp {
    fn render(&self, theme: &Theme) -> View {
        if !self.entered.get() {
            self.entered.set(true);
            self.trigger_entrance();
        }

        let show_counter = self.widgets.show_counter.checked.get();
        let [a0, a1, a2, a3] = [
            self.opacities[0].get(),
            self.opacities[1].get(),
            self.opacities[2].get(),
            self.opacities[3].get(),
        ];

        let mut children: Vec<View> = vec![
            opacity(a0, image("Glyph.png").size(120.0, 120.0).radius(12.0)),
        ];
        if show_counter {
            children.push(opacity(a1, self.counter.into_view(theme)));
        }
        children.push(opacity(a1, self.search.into_view(theme)));
        children.push(opacity(a2, self.anim_btn.into_view(theme)));
        children.push(opacity(a3, self.widgets.into_view(theme)));

        column(children)
            .justify(glyph_core::JustifyContent::FlexStart)
            .gap(16.0)
            .padding_y(32.0)
            .into()
    }
}

fn second_window(theme: &Theme, _opener: &WindowOpener) -> View {
    column(vec![
        text("Second Window", 32.0).color(theme.text).into(),
        text("This window was opened from the main window.", theme.font_size)
            .color(theme.text_muted)
            .wrap()
            .into(),
    ])
    .justify(glyph_core::JustifyContent::FlexStart)
    .padding_y(32.0)
    .into()
}

fn main() {
    let app = DemoApp::new();
    let dark_signal = app.dark_mode.clone();

    App::run(
        move |opener| {
            let theme = if dark_signal.get() { Theme::dark() } else { Theme::light() };
            let opener2 = opener.clone();
            let t = theme.clone();
            let mut view = app.render(&theme);
            if let View::Column { ref mut children, .. } = view {
                children.push(
                    button("Open Second Window", move || {
                        let t2 = t.clone();
                        opener2.open(
                            move |_op| (t2.clone(), second_window(&t2, _op)),
                            "Second Window", 400.0, 250.0, Theme::light(),
                        );
                    })
                    .bg(theme.surface)
                    .text_color(theme.text)
                    .radius(theme.radius)
                    .into(),
                );
            }
            (theme, view)
        },
        Theme::light(),
        "Glyph Demo",
        800.0,
        600.0,
    );
}
