//! Stateful form controls: Checkbox, Radio, Switch, Select, Slider, Stepper, SearchBar.
//! All state is Signal-based — callers own the signals so they persist across re-renders.

use crate::colors::{dark, with_opacity};
use crate::icons::{
    icon_checkmark, icon_chevron_down_outline, icon_close_outline, icon_remove_outline,
    icon_search_outline,
};
use crate::spacing::*;
use core_glyph::{
    button, button_view, column, rect, row, spacer, text, text_input, Color, Signal,
    Theme, View,
};


/// A labelled checkbox. `checked` signal owned by caller.
pub fn checkbox(
    theme: &Theme,
    label: impl Into<String>,
    checked: Signal<bool>,
    on_change: impl Fn(bool) + 'static,
) -> View {
    let c = checked.get();
    let box_bg  = if c { theme.primary }  else { Color::TRANSPARENT };
    let box_bdr = if c { theme.primary }  else { theme.border };
    let check: View = if c {
        icon_checkmark(Color::WHITE, 9.0)
    } else {
        column(vec![]).into()
    };
    let label_str: String = label.into();
    button_view(
        row(vec![
            column(vec![check])
                .width(16.0).height(16.0)
                .align_center().justify_center()
                .bg(box_bg).border(box_bdr, 1.5).radius(3.0).into(),
            text(label_str, TEXT_SM).color(theme.text).into(),
        ])
        .gap(SPACE_2).align_center().into(),
        move || on_change(!c),
    )
    .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT).into()
}

/// Checkbox without a label — just the box.
pub fn checkbox_bare(
    theme: &Theme,
    checked: Signal<bool>,
    on_change: impl Fn(bool) + 'static,
) -> View {
    let c = checked.get();
    let box_bg  = if c { theme.primary }  else { Color::TRANSPARENT };
    let box_bdr = if c { theme.primary }  else { theme.border };
    let check: View = if c { icon_checkmark(Color::WHITE, 9.0) } else { column(vec![]).into() };
    button_view(
        column(vec![check])
            .width(16.0).height(16.0)
            .align_center().justify_center()
            .bg(box_bg).border(box_bdr, 1.5).radius(3.0).into(),
        move || on_change(!c),
    )
    .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT).into()
}


/// Single radio button. `selected` is true when this option is active.
pub fn radio_button(
    theme: &Theme,
    label: impl Into<String>,
    selected: bool,
    on_select: impl Fn() + 'static,
) -> View {
    let dot: View = if selected {
        rect(theme.primary).width(8.0).height(8.0).radius(4.0).into()
    } else {
        column(vec![]).into()
    };
    let ring_color = if selected { theme.primary } else { theme.border };
    let label_str: String = label.into();
    button_view(
        row(vec![
            column(vec![dot])
                .width(16.0).height(16.0).align_center().justify_center()
                .bg(Color::TRANSPARENT).border(ring_color, 1.5).radius(8.0).into(),
            text(label_str, TEXT_SM).color(theme.text).into(),
        ])
        .gap(SPACE_2).align_center().into(),
        on_select,
    )
    .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT).into()
}

/// A radio group — renders a column of radio buttons.
pub fn radio_group<T: Clone + PartialEq + 'static>(
    theme: &Theme,
    options: Vec<(T, &str)>,
    selected: Signal<T>,
    on_change: impl Fn(T) + 'static + Clone,
) -> View {
    let items: Vec<View> = options
        .into_iter()
        .map(|(value, label)| {
            let cur = selected.get();
            let is_sel = cur == value;
            let v2 = value.clone();
            let cb = on_change.clone();
            radio_button(theme, label, is_sel, move || cb(v2.clone()))
        })
        .collect();
    column(items).gap(SPACE_2).into()
}


/// A toggle switch (iOS-style). `enabled` signal owned by caller.
pub fn switch(
    theme: &Theme,
    enabled: Signal<bool>,
    on_change: impl Fn(bool) + 'static,
) -> View {
    let on = enabled.get();
    let track_bg = if on { theme.primary } else { dark::SURFACE_3 };
    let thumb_x: f32 = if on { 18.0 } else { 2.0 };
    button_view(
        row(vec![
            // Track
            column(vec![
                // Thumb
                rect(Color::WHITE)
                    .width(14.0).height(14.0).radius(7.0).into(),
            ])
            .width(34.0).height(18.0)
            .bg(track_bg).radius(9.0)
            .padding_left(thumb_x)
            .padding_top(2.0)
            .into(),
        ])
        .align_center().into(),
        move || on_change(!on),
    )
    .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT).into()
}

/// Switch with a label.
pub fn switch_row(
    theme: &Theme,
    label: impl Into<String>,
    enabled: Signal<bool>,
    on_change: impl Fn(bool) + 'static,
) -> View {
    row(vec![
        text(label, TEXT_SM).color(theme.text).into(),
        spacer(),
        switch(theme, enabled, on_change),
    ])
    .gap(SPACE_3).align_center().fill_width().into()
}


/// A closed select — shows the selected label with a chevron.
/// When clicked it cycles through options (a lightweight approximation;
/// real dropdown overlay needs a portal layer not yet available).
pub fn select<T: Clone + PartialEq + 'static>(
    theme: &Theme,
    options: &[(T, &str)],
    selected: Signal<T>,
    on_change: impl Fn(T) + 'static,
) -> View {
    let cur = selected.get();
    let label = options
        .iter()
        .find(|(v, _)| v == &cur)
        .map(|(_, l)| *l)
        .unwrap_or("—");

    // Cycle to next option on click
    let opts: Vec<T> = options.iter().map(|(v, _)| v.clone()).collect();
    let cur_idx = opts.iter().position(|v| *v == cur).unwrap_or(0);
    let next_idx = (cur_idx + 1) % opts.len();
    let next_val = opts[next_idx].clone();

    button_view(
        row(vec![
            text(label, TEXT_SM).color(theme.text).into(),
            spacer(),
            icon_chevron_down_outline(theme.text_muted, 12.0),
        ])
        .gap(SPACE_2).align_center()
        .padding_x(SPACE_3).padding_y(SPACE_2).into(),
        move || on_change(next_val.clone()),
    )
    .bg(theme.surface)
    .hover_bg(dark::SURFACE_2)
    .into()
}

/// Inline select menu — renders all options as a visible list (for small sets).
pub fn select_menu<T: Clone + PartialEq + 'static>(
    theme: &Theme,
    options: &[(T, &str)],
    selected: Signal<T>,
    on_change: impl Fn(T) + 'static + Clone,
) -> View {
    let cur = selected.get();
    let items: Vec<View> = options
        .iter()
        .map(|(value, label)| {
            let is_active = *value == cur;
            let bg = if is_active { with_opacity(theme.primary, 0.12) } else { Color::TRANSPARENT };
            let fg = if is_active { theme.primary } else { theme.text };
            let v2 = value.clone();
            let cb = on_change.clone();
            button(*label, move || cb(v2.clone()))
                .bg(bg).hover_bg(dark::SURFACE_2)
                .text_color(fg).font_size(TEXT_SM)
                .height(BTN_HEIGHT_SM).fill_width().padding(SPACE_3)
                .into()
        })
        .collect();
    column(items)
        .gap(0.0).bg(theme.surface)
        .border(theme.border, 1.0).radius(RADIUS_LG).into()
}


/// Horizontal progress-style slider. Value 0.0–1.0.
/// Click position determines value (platform sends x-coord in on_change).
pub fn slider(theme: &Theme, value: f32, width: f32) -> View {
    let fill_w = (value.clamp(0.0, 1.0) * width).max(0.0);
    let _thumb_x = fill_w.clamp(0.0, width - 12.0);
    row(vec![
        // Filled track
        rect(theme.primary).width(fill_w).height(4.0).radius(2.0).into(),
        // Thumb
        rect(Color::WHITE).width(12.0).height(12.0).radius(6.0).into(),
        // Empty track
        rect(dark::SURFACE_3).width((width - fill_w - 12.0).max(0.0)).height(4.0).radius(2.0).into(),
    ])
    .gap(0.0).height(12.0).width(width).align_center().into()
}

pub fn slider_labeled(theme: &Theme, label: impl Into<String>, value: f32, width: f32) -> View {
    row(vec![
        text(label, TEXT_SM).color(theme.text_muted).into(),
        slider(theme, value, width),
        text(format!("{:.0}%", value * 100.0), TEXT_XS).color(theme.text_muted).into(),
    ])
    .gap(SPACE_3).align_center().into()
}


pub fn stepper(
    theme: &Theme,
    value: i32,
    min: i32,
    max: i32,
    on_change: impl Fn(i32) + 'static + Clone,
) -> View {
    let dec = on_change.clone();
    let inc = on_change.clone();
    let can_dec = value > min;
    let can_inc = value < max;

    row(vec![
        button_view(
            icon_remove_outline(if can_dec { theme.text } else { theme.text_muted }, 14.0),
            move || { if can_dec { dec(value - 1); } },
        )
        .bg(dark::SURFACE_2).hover_bg(dark::SURFACE_3)
        .width(28.0).height(28.0).radius(RADIUS_MD).into(),

        column(vec![text(value.to_string(), TEXT_SM).color(theme.text).into()])
            .width(40.0).height(28.0).align_center().justify_center()
            .bg(theme.surface).border(theme.border, 1.0).into(),

        button_view(
            icon_checkmark(if can_inc { theme.text } else { theme.text_muted }, 14.0),
            move || { if can_inc { inc(value + 1); } },
        )
        .bg(dark::SURFACE_2).hover_bg(dark::SURFACE_3)
        .width(28.0).height(28.0).radius(RADIUS_MD).into(),
    ])
    .gap(0.0).align_center().into()
}


/// A search input with icon and optional clear button.
pub fn search_bar(
    theme: &Theme,
    value: Signal<String>,
    focused: Signal<bool>,
    cursor: Signal<usize>,
    placeholder: impl Into<String>,
    on_change: impl Fn(String) + 'static,
    on_clear: impl Fn() + 'static,
) -> View {
    let has_value = !value.get().is_empty();
    row(vec![
        column(vec![icon_search_outline(theme.text_muted, 14.0)])
            .width(32.0).height(32.0).align_center().justify_center().into(),
        text_input(value.clone(), focused, cursor)
            .placeholder(placeholder).font_size(TEXT_SM)
            .bg(Color::TRANSPARENT).text_color(theme.text).border_color(Color::TRANSPARENT)
            .fill_width()
            .on_change(on_change).into(),
        if has_value {
            button_view(
                icon_close_outline(theme.text_muted, 12.0),
                on_clear,
            )
            .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
            .width(28.0).height(28.0).radius(14.0).into()
        } else { column(vec![]).into() },
    ])
    .gap(0.0).height(36.0).fill_width()
    .bg(theme.surface).border(theme.border, 1.0).radius(RADIUS_XL)
    .align_center().into()
}


/// A button that holds active/inactive visual state.
pub fn toggle_btn(
    theme: &Theme,
    label: impl Into<String>,
    active: bool,
    on_click: impl Fn() + 'static,
) -> View {
    let bg    = if active { with_opacity(theme.primary, 0.15) } else { Color::TRANSPARENT };
    let fg    = if active { theme.primary } else { theme.text_muted };
    let _bdr   = if active { theme.primary } else { theme.border };
    button(label, on_click)
        .bg(bg).hover_bg(dark::SURFACE_2)
        .text_color(fg).font_size(TEXT_SM)
        .height(BTN_HEIGHT_SM).padding(SPACE_3).radius(RADIUS_MD)
        .into()
}

/// A segmented control (joined toggle buttons).
pub fn toggle_group<T: Clone + PartialEq + 'static>(
    theme: &Theme,
    options: &[(T, &str)],
    selected: &T,
    on_change: impl Fn(T) + 'static + Clone,
) -> View {
    let items: Vec<View> = options.iter().map(|(val, label)| {
        let is_active = val == selected;
        let bg    = if is_active { with_opacity(theme.primary, 0.15) } else { Color::TRANSPARENT };
        let fg    = if is_active { theme.primary } else { theme.text_muted };
        let v2 = val.clone();
        let cb = on_change.clone();
        button(*label, move || cb(v2.clone()))
            .bg(bg).hover_bg(dark::SURFACE_2)
            .text_color(fg).font_size(TEXT_SM)
            .height(BTN_HEIGHT_SM).padding(SPACE_3).into()
    }).collect();
    row(items)
        .gap(0.0).padding(2.0)
        .bg(dark::SURFACE_2).radius(RADIUS_LG).into()
}


/// Visually joined buttons in a row.
pub fn button_group(buttons: Vec<View>) -> View {
    row(buttons).gap(0.0)
        .border(Color::rgb(0.3, 0.3, 0.35), 1.0)
        .radius(RADIUS_MD).into()
}
