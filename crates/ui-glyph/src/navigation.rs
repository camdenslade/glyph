//! Navigation components: Topbar, Sidebar, BottomTabBar, Breadcrumb, Pagination,
//! CommandPalette skeleton, Stepper wizard.

use crate::colors::{dark, with_opacity};
use crate::icons::{
    icon_chevron_back_outline, icon_chevron_forward_outline, icon_close_outline,
    icon_ellipsis_horizontal_outline, icon_search_outline,
};
use crate::spacing::*;
use core_glyph::{
    button, button_view, column, row, spacer, text, text_input, Color, FontWeight, Signal,
    Theme, View,
};


pub struct TopbarConfig<'a> {
    pub title: &'a str,
    pub leading: Option<View>,
    pub trailing: Vec<View>,
    pub subtitle: Option<&'a str>,
    pub height: f32,
}

impl<'a> Default for TopbarConfig<'a> {
    fn default() -> Self {
        Self { title: "", leading: None, trailing: vec![], subtitle: None, height: 44.0 }
    }
}

pub fn topbar(theme: &Theme, cfg: TopbarConfig) -> View {
    let title_col: View = if let Some(sub) = cfg.subtitle {
        column(vec![
            text(cfg.title, TEXT_BASE).weight(FontWeight::Bold).color(theme.text).into(),
            text(sub, TEXT_XS).color(theme.text_muted).into(),
        ]).gap(1.0).align_center().into()
    } else {
        text(cfg.title, TEXT_BASE).weight(FontWeight::Bold).color(theme.text).into()
    };

    let mut items: Vec<View> = vec![];
    if let Some(lead) = cfg.leading { items.push(lead); }
    items.push(spacer());
    items.push(title_col);
    items.push(spacer());
    items.extend(cfg.trailing);

    row(items)
        .gap(SPACE_2).height(cfg.height).fill_width()
        .padding_x(SPACE_3)
        .bg(theme.surface).border(theme.border, 1.0)
        .align_center().into()
}

/// Simple back button.
pub fn back_button(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button_view(
        row(vec![
            icon_chevron_back_outline(theme.primary, 16.0),
            text(label, TEXT_SM).color(theme.primary).into(),
        ])
        .gap(SPACE_1).align_center().into(),
        on_click,
    )
    .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
    .height(BTN_HEIGHT_MD).padding(SPACE_2).radius(RADIUS_LG).into()
}


pub struct SidebarItem {
    pub label: String,
    pub icon: Option<View>,
    pub badge: Option<u32>,
    pub active: bool,
}

pub fn sidebar(theme: &Theme, items: Vec<(SidebarItem, Box<dyn Fn()>)>) -> View {
    let rows: Vec<View> = items.into_iter().map(|(item, cb)| {
        sidebar_item(theme, item, cb)
    }).collect();
    column(rows).gap(2.0).padding(SPACE_2).fill_height().into()
}

pub fn sidebar_item(theme: &Theme, item: SidebarItem, on_click: impl Fn() + 'static) -> View {
    let bg    = if item.active { with_opacity(theme.primary, 0.10) } else { Color::TRANSPARENT };
    let fg    = if item.active { theme.primary } else { theme.text };
    let hover = if item.active { with_opacity(theme.primary, 0.14) } else { dark::SURFACE_2 };
    let mut inner: Vec<View> = vec![];
    if let Some(icon) = item.icon { inner.push(icon); }
    inner.push(text(item.label, TEXT_SM).color(fg).into());
    inner.push(spacer());
    if let Some(n) = item.badge {
        let label = if n > 99 { "99+".to_string() } else { n.to_string() };
        inner.push(
            row(vec![text(label, TEXT_XS).color(Color::WHITE).weight(FontWeight::Bold).into()])
                .padding_x(SPACE_1).padding_y(1.0)
                .bg(theme.primary).radius(RADIUS_FULL).into()
        );
    }
    button_view(
        row(inner).gap(SPACE_2).align_center().fill_width().padding_x(SPACE_3).into(),
        on_click,
    )
    .bg(bg).hover_bg(hover)
    .height(BTN_HEIGHT_MD).fill_width().radius(RADIUS_LG).into()
}

pub fn sidebar_section_label(theme: &Theme, label: impl Into<String>) -> View {
    row(vec![
        text(label, TEXT_XS).weight(FontWeight::Bold).color(theme.text_muted).into(),
    ])
    .padding_x(SPACE_3).padding_y(SPACE_2).into()
}


pub struct TabItem {
    pub label: String,
    pub icon: View,
    pub active: bool,
}

pub fn bottom_tab_bar(theme: &Theme, tabs: Vec<(TabItem, Box<dyn Fn()>)>) -> View {
    let items: Vec<View> = tabs.into_iter().map(|(tab, cb)| {
        let fg = if tab.active { theme.primary } else { theme.text_muted };
        button_view(
            column(vec![
                tab.icon,
                text(tab.label, TEXT_XS).color(fg).into(),
            ])
            .gap(SPACE_1).align_center().justify_center()
            .padding_y(SPACE_2).grow().into(),
            cb,
        )
        .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
        .grow().into()
    }).collect();
    row(items)
        .gap(0.0).height(56.0).fill_width()
        .bg(theme.surface).border(theme.border, 1.0).into()
}


pub fn pagination(
    theme: &Theme,
    current_page: usize,
    total_pages: usize,
    on_page: impl Fn(usize) + 'static + Clone,
) -> View {
    let mut items: Vec<View> = vec![];

    // Prev
    let can_prev = current_page > 1;
    let prev_cb = on_page.clone();
    let prev_page = current_page.saturating_sub(1);
    items.push(
        button_view(
            icon_chevron_back_outline(if can_prev { theme.text } else { theme.text_muted }, 14.0),
            move || { if can_prev { prev_cb(prev_page); } },
        )
        .bg(dark::SURFACE_2).hover_bg(dark::SURFACE_3)
        .width(BTN_HEIGHT_SM).height(BTN_HEIGHT_SM).radius(RADIUS_MD).into()
    );

    // Page numbers — show window around current
    let window = 5usize;
    let half = window / 2;
    let start = current_page.saturating_sub(half).max(1);
    let end = (start + window - 1).min(total_pages);

    for p in start..=end {
        let is_cur = p == current_page;
        let bg = if is_cur { theme.primary } else { Color::TRANSPARENT };
        let fg = if is_cur { Color::WHITE } else { theme.text };
        let cb = on_page.clone();
        items.push(
            button(p.to_string(), move || cb(p))
                .bg(bg).hover_bg(dark::SURFACE_2)
                .text_color(fg).font_size(TEXT_SM)
                .width(BTN_HEIGHT_SM).height(BTN_HEIGHT_SM).radius(RADIUS_MD).into()
        );
    }

    // Next
    let can_next = current_page < total_pages;
    let next_cb = on_page.clone();
    let next_page = (current_page + 1).min(total_pages);
    items.push(
        button_view(
            icon_chevron_forward_outline(if can_next { theme.text } else { theme.text_muted }, 14.0),
            move || { if can_next { next_cb(next_page); } },
        )
        .bg(dark::SURFACE_2).hover_bg(dark::SURFACE_3)
        .width(BTN_HEIGHT_SM).height(BTN_HEIGHT_SM).radius(RADIUS_MD).into()
    );

    row(items).gap(SPACE_1).align_center().into()
}


#[allow(clippy::type_complexity)]
pub fn breadcrumb_nav(
    theme: &Theme,
    segments: Vec<(impl Into<String>, Option<Box<dyn Fn()>>)>,
) -> View {
    let n = segments.len();
    let items: Vec<View> = segments.into_iter().enumerate().flat_map(|(i, (label, cb))| {
        let is_last = i == n - 1;
        let label_str: String = label.into();
        let text_view: View = if let Some(f) = cb {
            button(label_str, f)
                .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT)
                .text_color(theme.primary).font_size(TEXT_SM).padding(0.0).into()
        } else {
            text(label_str, TEXT_SM).color(theme.text).into()
        };
        if is_last {
            vec![text_view]
        } else {
            vec![
                text_view,
                text("/", TEXT_SM).color(theme.text_muted).into(),
            ]
        }
    }).collect();
    row(items).gap(SPACE_2).align_center().into()
}


pub fn steps_indicator(theme: &Theme, steps: Vec<&str>, current: usize) -> View {
    let n = steps.len();
    let items: Vec<View> = steps.into_iter().enumerate().flat_map(|(i, label)| {
        let done    = i < current;
        let active  = i == current;
        let _future  = i > current;
        let circle_bg  = if done || active { theme.primary } else { dark::SURFACE_2 };
        let circle_fg  = if done || active { Color::WHITE } else { theme.text_muted };
        let label_fg   = if active { theme.text } else if done { theme.primary } else { theme.text_muted };
        let num_str    = (i + 1).to_string();
        let step_view: View = column(vec![
            column(vec![text(num_str, TEXT_XS).color(circle_fg).weight(FontWeight::Bold).into()])
                .width(24.0).height(24.0).align_center().justify_center()
                .bg(circle_bg).radius(12.0).into(),
            text(label, TEXT_XS).color(label_fg).into(),
        ]).gap(SPACE_1).align_center().into();

        if i < n - 1 {
            vec![
                step_view,
                column(vec![]).height(1.0).width(40.0).bg(if done { theme.primary } else { theme.border }).into(),
            ]
        } else {
            vec![step_view]
        }
    }).collect();
    row(items).gap(SPACE_2).align_center().into()
}


pub fn command_palette(
    theme: &Theme,
    query: Signal<String>,
    focused: Signal<bool>,
    cursor: Signal<usize>,
    results: Vec<(String, Box<dyn Fn()>)>,
    on_query: impl Fn(String) + 'static,
    on_close: impl Fn() + 'static,
) -> View {
    let result_rows: Vec<View> = results.into_iter().map(|(label, cb)| {
        button_view(
            row(vec![
                icon_search_outline(theme.text_muted, 13.0),
                text(label, TEXT_SM).color(theme.text).into(),
            ])
            .gap(SPACE_2).padding_x(SPACE_4).padding_y(SPACE_2)
            .fill_width().align_center().into(),
            cb,
        )
        .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
        .fill_width().into()
    }).collect();

    column(vec![
        // Search bar at top
        row(vec![
            column(vec![icon_search_outline(theme.text_muted, 14.0)])
                .width(36.0).height(44.0).align_center().justify_center().into(),
            text_input(query, focused, cursor)
                .placeholder("Type a command or search…")
                .font_size(TEXT_BASE).bg(Color::TRANSPARENT)
                .text_color(theme.text).border_color(Color::TRANSPARENT)
                .fill_width().on_change(on_query).into(),
            button_view(
                icon_close_outline(theme.text_muted, 14.0),
                on_close,
            )
            .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
            .width(36.0).height(36.0).radius(RADIUS_MD).into(),
        ])
        .gap(0.0).height(44.0).fill_width()
        .border(theme.border, 1.0).align_center().into(),

        column(result_rows).gap(0.0).fill_width().into(),
    ])
    .gap(0.0)
    .bg(theme.surface).border(theme.border, 1.0).radius(RADIUS_2XL)
    .shadow(crate::shadows::shadow_2xl())
    .fill_width().into()
}


pub fn menu_bar(theme: &Theme, menus: Vec<(&str, bool)>, on_select: impl Fn(usize) + 'static + Clone) -> View {
    let items: Vec<View> = menus.into_iter().enumerate().map(|(i, (label, active))| {
        let cb = on_select.clone();
        let bg = if active { dark::SURFACE_2 } else { Color::TRANSPARENT };
        button(label, move || cb(i))
            .bg(bg).hover_bg(dark::SURFACE_2)
            .text_color(theme.text).font_size(TEXT_SM)
            .height(BTN_HEIGHT_SM).padding(SPACE_2).into()
    }).collect();
    row(items).gap(0.0).fill_width().padding_x(SPACE_2).into()
}


pub fn toolbar(theme: &Theme, leading: Vec<View>, trailing: Vec<View>) -> View {
    let mut items = leading;
    items.push(spacer());
    items.extend(trailing);
    row(items)
        .gap(SPACE_1).height(44.0).fill_width()
        .padding_x(SPACE_3)
        .bg(theme.surface).border(theme.border, 1.0)
        .align_center().into()
}


pub fn overflow_menu_btn(theme: &Theme, on_click: impl Fn() + 'static) -> View {
    button_view(
        icon_ellipsis_horizontal_outline(theme.text_muted, 16.0),
        on_click,
    )
    .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
    .width(BTN_HEIGHT_SM).height(BTN_HEIGHT_SM).radius(RADIUS_MD).into()
}
