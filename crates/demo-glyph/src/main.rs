use glyph::prelude::*;

fn main() {
    // State
    let dark       = Signal::new(false);
    let count      = Signal::new(0i32);
    let scroll_y   = Signal::new(0.0f32);
    let max_scroll = Signal::new((-1.0f32, -1.0f32));
    let tab        = Signal::new(0usize);
    let input_val  = Signal::new(String::new());
    let input_foc  = Signal::new(false);
    let input_cur  = Signal::new(0usize);

    App::run(
        move |_opener, _closer| {
            let theme = if dark.get() { dark_theme() } else { light_theme() };

            // ── Header ──────────────────────────────────────────────────────
            let header = row(vec![
                text("Glyph", TEXT_2XL).color(theme.text).into(),
                spacer(),
                badge_success("v0.1"),
            ]).fill_width().align_center().into();

            // ── Counter ─────────────────────────────────────────────────────
            let c1 = count.clone(); let c2 = count.clone();
            let counter = card(
                &theme,
                vec![
                    text("Counter", TEXT_SM).color(theme.text_muted).into(),
                    row(vec![
                        btn_secondary(&theme, "−", move || c1.set(c1.get() - 1)),
                        text(count.get().to_string(), TEXT_3XL)
                            .color(theme.text).into(),
                        btn(&theme, "+", move || c2.set(c2.get() + 1)),
                    ]).gap(SPACE_4).align_center().into(),
                ],
            );

            // ── Text input ───────────────────────────────────────────────────
            let s = input_val.clone();
            let preview = if input_val.get().is_empty() {
                "Start typing…".to_string()
            } else {
                input_val.get()
            };
            let input_card = card(
                &theme,
                vec![
                    text("Text input", TEXT_SM).color(theme.text_muted).into(),
                    text_input(input_val.clone(), input_foc.clone(), input_cur.clone())
                        .placeholder("Type something…")
                        .bg(theme.background).text_color(theme.text)
                        .border_color(theme.border).radius(theme.radius)
                        .font_size(theme.font_size)
                        .on_change(move |v| s.set(v))
                        .fill_width().into(),
                    text(preview, TEXT_SM).color(theme.text_muted).into(),
                ],
            );

            // ── Theme toggle ─────────────────────────────────────────────────
            let d = dark.clone();
            let theme_card = card(
                &theme,
                vec![
                    switch_row(&theme, "Dark mode", dark.clone(), move |v| d.set(v)),
                ],
            );

            // ── Tabs ─────────────────────────────────────────────────────────
            let t = tab.clone();
            let tab_content: View = match tab.get() {
                0 => text("Overview content.", TEXT_SM).color(theme.text_muted).into(),
                1 => text("Activity feed.", TEXT_SM).color(theme.text_muted).into(),
                _ => text("Settings panel.", TEXT_SM).color(theme.text_muted).into(),
            };
            let tabs_card = card(
                &theme,
                vec![
                    tab_bar(&theme,
                        vec![("Overview", tab.get()==0), ("Activity", tab.get()==1), ("Settings", tab.get()==2)],
                        move |i| t.set(i),
                    ),
                    tab_content,
                ],
            );

            // ── Alerts ───────────────────────────────────────────────────────
            let alerts = column(vec![
                alert_info("Info", "This is a Glyph app."),
                alert_success("Success", "Everything is working."),
                alert_warning("Warning", "Check your settings."),
                alert_danger("Error", "Something went wrong."),
            ]).gap(SPACE_2).fill_width().into();

            // ── Badges ───────────────────────────────────────────────────────
            let badges = row(vec![
                badge("default"),
                badge_success("success"),
                badge_warning("warning"),
                badge_danger("danger"),
            ]).gap(SPACE_2).into();

            // ── Progress ─────────────────────────────────────────────────────
            let progress = card(&theme, vec![
                text("Progress", TEXT_SM).color(theme.text_muted).into(),
                progress_bar(0.72, theme.primary, theme.border, 6.0),
                progress_bar(0.45, Color::rgb(0.247, 0.722, 0.314), theme.border, 6.0),
                progress_bar(0.91, Color::rgb(0.973, 0.32, 0.286), theme.border, 6.0),
            ]);

            let content = column(vec![
                header,
                counter,
                input_card,
                theme_card,
                tabs_card,
                alerts,
                badges,
                progress,
            ])
            .gap(SPACE_4).padding(SPACE_6).fill_width().into();

            let view = scroll(content, Signal::new(0.0), scroll_y.clone(), max_scroll.clone())
                .fill_width().grow().into();

            (theme, view)
        },
        light_theme(),
        "Glyph Demo",
        720.0,
        900.0,
    );
}
