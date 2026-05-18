use core_glyph::{
    Color, Component, Easing, Router, Signal, Theme, Tween, View,
    button, column, image, opacity, row, scroll, spacer, text,
    text_area, text_input, virtual_list,
};
use platform_glyph::{App, WindowCloser, WindowOpener};
use ui_glyph::{
    alert_danger, alert_info,
    alert_success, alert_warning,
    avatar_placeholder_md,
    badge, badge_danger, badge_success, badge_warning,
    btn, btn_danger, btn_ghost, btn_secondary,
    card_section,
    count_bubble,
    divider_with_label,
    dot_online, dot_busy,
    empty_state_with_action,
    form_field,
    hr,
    kbd,
    nav_item,
    pill_primary, pill_success, pill_danger,
    progress_bar,
    skeleton, skeleton_text,
    stat_card_with_change,
    tab_bar,
    tag,
    BLUE_500, GREEN_500, RED_500,
    spacing::SPACE_3,
};
use widgets_glyph::{Checkbox, Select, Slider, Toggle};

// Counter

struct Counter {
    count: Signal<i32>,
}
impl Counter {
    fn new() -> Self { Self { count: Signal::new(0) } }
}
impl Component for Counter {
    fn render(&self, theme: &Theme) -> View {
        let c = self.count.clone();
        let c2 = self.count.clone();
        let header = row(vec![
            text("Counter", theme.font_size).color(theme.text).into(),
            spacer(),
            count_bubble(self.count.get().unsigned_abs(), BLUE_500),
        ]).fill_width().auto_size().into();
        let body = vec![
            row(vec![
                btn_secondary(theme, "-", move || c.set(c.get() - 1)),
                text(format!("{}", self.count.get()), 28.0)
                    .color(theme.text).width(60.0).into(),
                btn(theme, "+", move || c2.set(c2.get() + 1)),
            ]).auto_size().gap(12.0).align_center().into(),
        ];
        card_section(theme, header, body)
    }
}

// Text inputs

struct TextInputs {
    value:    Signal<String>,
    focused:  Signal<bool>,
    cursor:   Signal<usize>,
    ta_value: Signal<String>,
    ta_focus: Signal<bool>,
    ta_cur:   Signal<usize>,
    ta_scroll:Signal<f32>,
}
impl TextInputs {
    fn new() -> Self {
        Self {
            value:     Signal::new(String::new()),
            focused:   Signal::new(false),
            cursor:    Signal::new(0),
            ta_value:  Signal::new(String::new()),
            ta_focus:  Signal::new(false),
            ta_cur:    Signal::new(0),
            ta_scroll: Signal::new(0.0),
        }
    }
}
impl Component for TextInputs {
    fn render(&self, theme: &Theme) -> View {
        let preview = if self.value.get().is_empty() {
            "Nothing typed yet.".to_string()
        } else {
            format!("→ {}", self.value.get())
        };
        let single = text_input(
            self.value.clone(), self.focused.clone(), self.cursor.clone(),
        )
        .placeholder("Type something...")
        .bg(theme.background).text_color(theme.text)
        .border_color(theme.border).radius(theme.radius)
        .font_size(theme.font_size).fill_width().into();

        let multi = text_area(
            self.ta_value.clone(), self.ta_focus.clone(),
            self.ta_cur.clone(), self.ta_scroll.clone(),
        )
        .placeholder("Multi-line...")
        .bg(theme.background).text_color(theme.text)
        .border_color(theme.border).radius(theme.radius)
        .font_size(theme.font_size).fill_width().height(60.0).into();

        let header = text("Text Inputs", theme.font_size).color(theme.text).into();
        let body = vec![
            form_field(theme, "Single line", single, None::<&str>, None::<&str>),
            text(preview, 13.0).color(theme.text_muted).into(),
            form_field(theme, "Multi line", multi, Some("Shift+Enter for newline"), None::<&str>),
        ];
        card_section(theme, header, body)
    }
}

// Animated button

struct AnimButton {
    bg:    Signal<Color>,
    tween: Tween<Color>,
    hover: Signal<Color>,
}
impl AnimButton {
    fn new(theme: &Theme) -> Self {
        let bg = Signal::new(theme.primary);
        let tween = Tween::new(bg.clone(), Easing::EaseOut, 0.15);
        Self { bg, tween, hover: Signal::new(theme.primary) }
    }
}
impl Component for AnimButton {
    fn render(&self, theme: &Theme) -> View {
        let rest  = theme.primary;
        let bright = Color::rgb((rest.r * 1.2).min(1.0), (rest.g * 1.2).min(1.0), (rest.b * 1.2).min(1.0));
        let dim    = Color::rgb(rest.r * 0.75, rest.g * 0.75, rest.b * 0.75);
        let t1 = self.tween.clone(); let t2 = self.tween.clone();
        let hc = self.hover.clone(); let hc2 = self.hover.clone();
        let header = text("Animations", theme.font_size).color(theme.text).into();
        let body = vec![
            button("Hover & click me", || {})
                .bg(self.bg.get()).text_color(theme.on_primary)
                .radius(theme.radius).font_size(theme.font_size)
                .on_hover(move |h| { let c = if h { bright } else { rest }; hc.set(c); t1.start(c); })
                .on_press(move |p| { t2.start(if p { dim } else { hc2.get() }); })
                .into(),
            text("Color tweens on hover and press.", 13.0).color(theme.text_muted).into(),
        ];
        card_section(theme, header, body)
    }
}

// Button showcase

fn button_showcase(theme: &Theme) -> View {
    let header = text("Buttons", theme.font_size).color(theme.text).into();
    let body = vec![
        row(vec![
            btn(theme, "Primary", || {}),
            btn_secondary(theme, "Secondary", || {}),
            btn_ghost(theme, "Ghost", || {}),
            btn_danger(theme, "Danger", || {}),
        ]).auto_size().gap(8.0).into(),
        hr(theme),
        row(vec![
            badge("default"),
            badge_success("success"),
            badge_warning("warning"),
            badge_danger("danger"),
        ]).auto_size().gap(6.0).into(),
        row(vec![
            pill_primary(theme, "primary"),
            pill_success("success"),
            pill_danger("danger"),
            tag(theme, "tag"),
        ]).auto_size().gap(6.0).into(),
    ];
    card_section(theme, header, body)
}

// Status & avatars

fn status_section(theme: &Theme) -> View {
    let bg1 = theme.primary;
    let bg2 = ui_glyph::colors::dark::SUCCESS;
    let header = text("Status & Avatars", theme.font_size).color(theme.text).into();
    let body = vec![
        row(vec![
            avatar_placeholder_md("JS", bg1),
            avatar_placeholder_md("AK", bg2),
            column(vec![
                row(vec![dot_online(), text("Online", 13.0).color(theme.text).into()])
                    .auto_size().gap(6.0).into(),
                row(vec![dot_busy(), text("Busy", 13.0).color(theme.text_muted).into()])
                    .auto_size().gap(6.0).into(),
            ]).auto_size().gap(4.0).into(),
        ]).auto_size().gap(12.0).into(),
    ];
    card_section(theme, header, body)
}

// Stats

fn stats_section(theme: &Theme) -> View {
    row(vec![
        stat_card_with_change(theme, "Total Views", "48,291", "+12.4%", true),
        stat_card_with_change(theme, "Signups", "1,204", "+3.1%", true),
        stat_card_with_change(theme, "Churn", "2.3%", "-0.4%", false),
    ]).fill_width().gap(12.0).into()
}

// Alerts

fn alerts_section(theme: &Theme) -> View {
    let header = text("Alerts", theme.font_size).color(theme.text).into();
    let body = vec![
        alert_info("Note", "This is an informational message."),
        alert_success("Success", "Your changes have been saved."),
        alert_warning("Warning", "This action cannot be undone."),
        alert_danger("Error", "Something went wrong. Please retry."),
    ];
    card_section(theme, header, body)
}

// Progress

fn progress_section(theme: &Theme) -> View {
    let header = text("Progress", theme.font_size).color(theme.text).into();
    let body = vec![
        text("Storage", 13.0).color(theme.text_muted).into(),
        progress_bar(0.72, BLUE_500, theme.border, 8.0),
        text("Memory", 13.0).color(theme.text_muted).into(),
        progress_bar(0.45, GREEN_500, theme.border, 8.0),
        text("CPU", 13.0).color(theme.text_muted).into(),
        progress_bar(0.91, RED_500, theme.border, 8.0),
    ];
    card_section(theme, header, body)
}

// Skeleton loaders

fn skeleton_section(theme: &Theme) -> View {
    let header = text("Skeleton loaders", theme.font_size).color(theme.text).into();
    let body = vec![
        row(vec![
            skeleton(theme, 40.0, 40.0),
            column(vec![
                skeleton_text(theme, 160.0),
                skeleton_text(theme, 120.0),
            ]).auto_size().gap(6.0).into(),
        ]).auto_size().gap(12.0).into(),
        skeleton(theme, f32::MAX, 80.0),
    ];
    card_section(theme, header, body)
}

// Keyboard shortcuts

fn kbd_section(theme: &Theme) -> View {
    let header = text("Keyboard shortcuts", theme.font_size).color(theme.text).into();
    let body = vec![
        row(vec![
            text("Save", 13.0).color(theme.text).into(),
            spacer(),
            kbd(theme, "⌘"), kbd(theme, "S"),
        ]).fill_width().auto_size().into(),
        row(vec![
            text("Close window", 13.0).color(theme.text).into(),
            spacer(),
            kbd(theme, "⌘"), kbd(theme, "W"),
        ]).fill_width().auto_size().into(),
        row(vec![
            text("Open command palette", 13.0).color(theme.text).into(),
            spacer(),
            kbd(theme, "⌘"), kbd(theme, "K"),
        ]).fill_width().auto_size().into(),
    ];
    card_section(theme, header, body)
}

// Controls (checkbox, toggle, select, slider)

struct Controls {
    show_counter: Checkbox,
    dark_toggle:  Toggle,
    theme_select: Select,
    slider:       Slider,
}
impl Controls {
    fn new(dark: Signal<bool>) -> Self {
        Self {
            show_counter: Checkbox::new("Show counter", Signal::new(true)),
            dark_toggle:  Toggle::new("Dark mode", dark),
            theme_select: Select::new(vec!["System", "Light", "Dark"], Signal::new(0)),
            slider:       Slider::new("Volume", Signal::new(0.5), 0.0, 1.0),
        }
    }
}
impl Component for Controls {
    fn render(&self, theme: &Theme) -> View {
        let header = text("Controls", theme.font_size).color(theme.text).into();
        let body = vec![
            self.show_counter.into_view(theme),
            self.dark_toggle.into_view(theme),
            self.theme_select.into_view(theme),
            self.slider.into_view(theme),
        ];
        card_section(theme, header, body)
    }
}

// Tab bar demo

struct TabDemo { tab: Signal<usize> }
impl TabDemo {
    fn new() -> Self { Self { tab: Signal::new(0) } }
}
impl Component for TabDemo {
    fn render(&self, theme: &Theme) -> View {
        let t = self.tab.clone();
        let tabs = vec![
            ("Overview", self.tab.get() == 0),
            ("Activity", self.tab.get() == 1),
            ("Settings", self.tab.get() == 2),
        ];
        let content: View = match self.tab.get() {
            0 => text("Overview content goes here.", 14.0).color(theme.text_muted).wrap().into(),
            1 => text("Recent activity feed.", 14.0).color(theme.text_muted).into(),
            _ => text("Settings panel.", 14.0).color(theme.text_muted).into(),
        };
        let tb = tab_bar(theme, tabs, move |i| t.set(i));
        let header = text("Tab Bar", theme.font_size).color(theme.text).into();
        let body = vec![tb, content];
        card_section(theme, header, body)
    }
}

// Navigation / multi-window

struct NavDemo { page: Signal<usize> }
impl NavDemo {
    fn new() -> Self { Self { page: Signal::new(0) } }

    fn view(&self, theme: &Theme, opener: &WindowOpener) -> View {
        let p1 = self.page.clone();
        let p2 = self.page.clone();
        let o  = opener.clone();
        let content: View = match self.page.get() {
            0 => column(vec![
                text("Navigate within the app or open new windows.", 13.0)
                    .color(theme.text_muted).wrap().into(),
                row(vec![
                    btn(theme, "Go to detail", move || p1.set(1)),
                    btn_ghost(theme, "New window", move || {
                        o.open(|op, cl| {
                            let t = Theme::dark();
                            (t.clone(), second_window(&t, op, cl))
                        }, "Detail", 480.0, 320.0, Theme::dark());
                    }),
                ]).auto_size().gap(8.0).into(),
            ]).fill_width().gap(10.0).into(),
            _ => column(vec![
                text("Detail page.", 14.0).color(theme.text_muted).into(),
                btn_secondary(theme, "← Back", move || p2.set(0)),
            ]).fill_width().gap(10.0).into(),
        };
        let header = text("Navigation", theme.font_size).color(theme.text).into();
        card_section(theme, header, vec![content])
    }
}

// Virtual list

struct VList { offset_y: Signal<f32> }
impl VList {
    fn new() -> Self { Self { offset_y: Signal::new(0.0) } }
}
impl Component for VList {
    fn render(&self, theme: &Theme) -> View {
        let oy = self.offset_y.clone();
        let border = theme.border;
        let tc = theme.text;
        let tm = theme.text_muted;
        let header = row(vec![
            text("Virtual List", theme.font_size).color(theme.text).into(),
            spacer(),
            badge("1 000 rows"),
        ]).fill_width().auto_size().into();
        let body = vec![
            virtual_list(1000, 40.0, oy, 200.0, move |i| {
                column(vec![
                    row(vec![
                        text(format!("Row {}", i + 1), 14.0).color(tc).into(),
                        spacer(),
                        text(format!("#{:04}", i + 1), 12.0).color(tm).into(),
                    ])
                    .fill_width().auto_size().into(),
                    core_glyph::rect(border).fill_width().height(1.0).into(),
                ])
                .fill_width().auto_size()
                .padding_x(12.0).padding_y(8.0).into()
            })
            .fill_width().height(200.0).into(),
        ];
        card_section(theme, header, body)
    }
}

// Empty state

fn empty_section(theme: &Theme) -> View {
    empty_state_with_action(
        theme,
        "No results found",
        "Try adjusting your filters or search query.",
        btn(theme, "Clear filters", || {}),
    )
}

fn nav_section(theme: &Theme, router: &Router) -> View {
    let cur = router.current();
    let r0 = router.stack.clone();
    let r1 = router.stack.clone();
    let r2 = router.stack.clone();

    let nav: Vec<View> = vec![
        nav_item(theme, "Dashboard", cur == 0, move || r0.set(vec![0])),
        nav_item(theme, "Projects",  cur == 1, move || r1.set(vec![1])),
        nav_item(theme, "Settings",  cur == 2, move || r2.set(vec![2])),
        divider_with_label(theme, "Recent"),
        nav_item(theme, "glyph-core", false, || {}),
        nav_item(theme, "glyph-demo", false, || {}),
    ];

    let page_content: View = match cur {
        0 => text("Dashboard - metrics and activity at a glance.", 13.0).color(theme.text_muted).wrap().into(),
        1 => text("Projects - browse and manage your repositories.", 13.0).color(theme.text_muted).wrap().into(),
        _ => text("Settings - account, appearance, and integrations.", 13.0).color(theme.text_muted).wrap().into(),
    };

    let mut body = nav;
    body.push(page_content);
    let header = text("Nav Items", theme.font_size).color(theme.text).into();
    card_section(theme, header, body)
}

// Root app

struct Demo {
    counter:    Counter,
    inputs:     TextInputs,
    anim_btn:   AnimButton,
    controls:   Controls,
    tab_demo:   TabDemo,
    vlist:      VList,
    nav:        NavDemo,
    router:     Router,
    dark:       Signal<bool>,
    scroll_x:   Signal<f32>,
    scroll_y:   Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
    opacities:  Vec<Signal<f32>>,
    tweens:     Vec<Tween<f32>>,
    entered:    Signal<bool>,
}

impl Demo {
    fn new() -> Self {
        let dark   = Signal::new(false);
        let theme  = Theme::light();
        let opacities: Vec<Signal<f32>> = (0..6).map(|_| Signal::new(0.0)).collect();
        let tweens: Vec<Tween<f32>> = opacities.iter().enumerate().map(|(i, s)| {
            Tween::new(s.clone(), Easing::EaseOut, 0.2 + i as f32 * 0.06)
        }).collect();
        Self {
            counter:   Counter::new(),
            inputs:    TextInputs::new(),
            anim_btn:  AnimButton::new(&theme),
            controls:  Controls::new(dark.clone()),
            tab_demo:  TabDemo::new(),
            vlist:     VList::new(),
            nav:       NavDemo::new(),
            router:    Router::new(vec![
                Box::new(|_| text("Dashboard - metrics and activity at a glance.", 13.0).into()),
                Box::new(|_| text("Projects - browse and manage your repositories.", 13.0).into()),
                Box::new(|_| text("Settings - account, appearance, and integrations.", 13.0).into()),
            ]),
            dark,
            scroll_x:  Signal::new(0.0),
            scroll_y:  Signal::new(0.0),
            max_scroll: Signal::new((-1.0, -1.0)),
            opacities,
            tweens,
            entered:   Signal::new(false),
        }
    }

    fn render(&self, theme: &Theme, opener: &WindowOpener, _closer: &WindowCloser) -> View {
        if !self.entered.get() {
            self.entered.set(true);
            for t in &self.tweens { t.animate(0.0, 1.0); }
        }

        let show_counter = self.controls.show_counter.checked.get();
        let a: Vec<f32> = self.opacities.iter().map(|s| s.get()).collect();

        let header: View = row(vec![
            image("Glyph.png").size(40.0, 40.0).radius(8.0).into(),
            column(vec![
                text("Glyph", 20.0).color(theme.text).into(),
                text("Rust UI framework", 13.0).color(theme.text_muted).into(),
            ]).auto_size().gap(2.0).into(),
            spacer(),
            badge_success("v0.1"),
        ]).fill_width().auto_size().padding_y(4.0).into();

        let mut sections: Vec<View> = vec![opacity(a[0], header)];
        sections.push(opacity(a[1], stats_section(theme)));
        if show_counter {
            sections.push(opacity(a[1], self.counter.into_view(theme)));
        }
        sections.push(opacity(a[2], button_showcase(theme)));
        sections.push(opacity(a[2], self.anim_btn.into_view(theme)));
        sections.push(opacity(a[2], alerts_section(theme)));
        sections.push(opacity(a[3], self.inputs.into_view(theme)));
        sections.push(opacity(a[3], progress_section(theme)));
        sections.push(opacity(a[3], skeleton_section(theme)));
        sections.push(opacity(a[4], status_section(theme)));
        sections.push(opacity(a[4], kbd_section(theme)));
        sections.push(opacity(a[4], self.tab_demo.into_view(theme)));
        sections.push(opacity(a[4], self.controls.into_view(theme)));
        sections.push(opacity(a[4], self.nav.view(theme, opener)));
        sections.push(opacity(a[4], nav_section(theme, &self.router)));
        sections.push(opacity(a[5], empty_section(theme)));
        sections.push(opacity(a[5], self.vlist.into_view(theme)));

        let inner = column(sections)
            .justify(core_glyph::JustifyContent::FlexStart)
            .gap(SPACE_3)
            .padding_x(20.0)
            .padding_y(24.0)
            .fill_width()
            .into();

        scroll(inner, self.scroll_x.clone(), self.scroll_y.clone(), self.max_scroll.clone()).into()
    }
}

fn second_window(theme: &Theme, _opener: &WindowOpener, closer: &WindowCloser) -> View {
    let cl = closer.clone();
    column(vec![
        text("Second Window", 24.0).color(theme.text).into(),
        text("Opened from the main window.", 14.0).color(theme.text_muted).wrap().into(),
        btn_secondary(theme, "Close", move || cl.close()),
    ])
    .justify(core_glyph::JustifyContent::FlexStart)
    .gap(12.0).padding(32.0).fill_width().into()
}

fn main() {
    let demo = Demo::new();
    let dark = demo.dark.clone();

    App::run(
        move |opener, closer| {
            let theme = if dark.get() { Theme::dark() } else { Theme::light() };
            (theme.clone(), demo.render(&theme, opener, closer))
        },
        Theme::light(),
        "Glyph Demo",
        800.0,
        1200.0,
    );
}
