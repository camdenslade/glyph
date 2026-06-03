//! Overlay components: Modal/Dialog, Alert Dialog, Drawer/Sheet, Popover skeleton,
//! Toast notification, Accordion, Collapsible, HoverCard skeleton.
//!
//! Since we have no portal layer, overlays are rendered inline by the caller
//! and should be placed in a ZStack or at the root column level.

use crate::colors::{dark, with_opacity};
use crate::icons::{icon_alert_circle_outline, icon_checkmark_circle_outline, icon_close_outline, icon_information_circle_outline, icon_warning_outline};
use crate::spacing::*;
use core_glyph::{
    button, button_view, column, rect, row, spacer, text, zstack,
    Color, FontWeight, Shadow, Signal, Theme, View,
};


pub struct DialogConfig {
    pub title: String,
    pub width: f32,
}

impl Default for DialogConfig {
    fn default() -> Self {
        Self { title: String::new(), width: 480.0 }
    }
}

/// A modal dialog panel (caller places this in a ZStack over backdrop).
pub fn dialog(
    theme: &Theme,
    title: impl Into<String>,
    body: Vec<View>,
    actions: Vec<View>,
    width: f32,
) -> View {
    column(vec![
        // Header
        row(vec![
            text(title, TEXT_LG).weight(FontWeight::Bold).color(theme.text).into(),
            spacer(),
        ])
        .gap(0.0).fill_width().padding_x(SPACE_6).padding_y(SPACE_4).into(),
        // Divider
        rect(theme.border).height(1.0).fill_width().into(),
        // Body
        column(body).gap(SPACE_3).padding(SPACE_6).fill_width().into(),
        // Divider
        rect(theme.border).height(1.0).fill_width().into(),
        // Actions
        row(actions).gap(SPACE_2).padding_x(SPACE_6).padding_y(SPACE_4)
            .fill_width().justify_end().into(),
    ])
    .gap(0.0).width(width)
    .bg(theme.surface).border(theme.border, 1.0).radius(RADIUS_2XL)
    .shadow(Shadow { offset_x: 0.0, offset_y: 24.0, blur: 48.0, color: Color::rgba(0.0,0.0,0.0,0.6) })
    .into()
}

/// Full-screen modal with backdrop — place this in a ZStack at the root.
pub fn modal(
    theme: &Theme,
    title: impl Into<String>,
    body: Vec<View>,
    actions: Vec<View>,
    on_close: impl Fn() + 'static,
    width: f32,
) -> View {
    zstack(vec![
        // Backdrop
        button_view(
            column(vec![]).fill_width().fill_height().bg(Color::rgba(0.0, 0.0, 0.0, 0.6)).into(),
            on_close,
        )
        .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT).fill_width().grow().into(),
        // Panel — centered
        column(vec![
            dialog(theme, title, body, actions, width),
        ])
        .fill_width().fill_height().align_center().justify_center().into(),
    ])
    .into()
}


/// A confirmation dialog with icon, message, and confirm/cancel actions.
pub fn alert_dialog(
    theme: &Theme,
    title: impl Into<String>,
    message: impl Into<String>,
    confirm_label: impl Into<String>,
    confirm_destructive: bool,
    on_confirm: impl Fn() + 'static,
    on_cancel: impl Fn() + 'static,
) -> View {
    let confirm_bg    = if confirm_destructive { dark::DANGER } else { theme.primary };
    let confirm_hover = if confirm_destructive { dark::DANGER_HOVER } else { with_opacity(theme.primary, 0.85) };
    let confirm_label_s: String = confirm_label.into();
    dialog(
        theme,
        title,
        vec![text(message, TEXT_SM).color(theme.text_muted).wrap().into()],
        vec![
            button("Cancel", on_cancel)
                .bg(dark::SURFACE_2).hover_bg(dark::SURFACE_3)
                .text_color(theme.text).font_size(TEXT_SM)
                .height(BTN_HEIGHT_SM).padding(SPACE_4).radius(RADIUS_LG).into(),
            button(confirm_label_s, on_confirm)
                .bg(confirm_bg).hover_bg(confirm_hover)
                .text_color(Color::WHITE).font_size(TEXT_SM)
                .height(BTN_HEIGHT_SM).padding(SPACE_4).radius(RADIUS_LG).into(),
        ],
        420.0,
    )
}


/// A slide-in sheet from the right. Caller places this at root ZStack level.
pub fn drawer_right(
    theme: &Theme,
    title: impl Into<String>,
    body: Vec<View>,
    width: f32,
    on_close: impl Fn() + 'static,
) -> View {
    zstack(vec![
        button_view(
            column(vec![]).fill_width().fill_height().bg(Color::rgba(0.0, 0.0, 0.0, 0.5)).into(),
            on_close,
        )
        .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT).fill_width().grow().into(),
        row(vec![
            spacer(),
            column(vec![
                row(vec![
                    text(title, TEXT_LG).weight(FontWeight::Bold).color(theme.text).into(),
                    spacer(),
                    button_view(
                        icon_close_outline(theme.text_muted, 16.0),
                        || {},
                    )
                    .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
                    .width(BTN_HEIGHT_SM).height(BTN_HEIGHT_SM).radius(RADIUS_MD).into(),
                ])
                .gap(0.0).fill_width().padding(SPACE_4).align_center().into(),
                rect(theme.border).height(1.0).fill_width().into(),
                column(body).gap(SPACE_3).padding(SPACE_4).grow().fill_width().into(),
            ])
            .gap(0.0).width(width).fill_height()
            .bg(theme.surface).border(theme.border, 1.0).into(),
        ])
        .fill_width().fill_height().into(),
    ])
    .into()
}

/// Bottom sheet / action sheet.
pub fn bottom_sheet(
    theme: &Theme,
    title: impl Into<String>,
    body: Vec<View>,
    on_close: impl Fn() + 'static,
) -> View {
    zstack(vec![
        button_view(
            column(vec![]).fill_width().fill_height().bg(Color::rgba(0.0, 0.0, 0.0, 0.5)).into(),
            on_close,
        )
        .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT).fill_width().grow().into(),
        column(vec![
            spacer(),
            column(vec![
                column(vec![
                    rect(theme.border).width(36.0).height(4.0).radius(2.0).into()
                ]).align_center().justify_center().padding_y(SPACE_2).fill_width().into(),
                text(title, TEXT_LG).weight(FontWeight::Bold).color(theme.text).into(),
                rect(theme.border).height(1.0).fill_width().into(),
                column(body).gap(SPACE_2).padding(SPACE_4).fill_width().into(),
            ])
            .gap(SPACE_2).padding_x(SPACE_4).padding_y(SPACE_2)
            .bg(theme.surface).border(theme.border, 1.0)
            .radius(RADIUS_2XL).fill_width().into(),
        ])
        .fill_width().fill_height().into(),
    ])
    .into()
}


/// A single accordion section.
pub fn accordion_item(
    theme: &Theme,
    title: impl Into<String>,
    body: Vec<View>,
    open: Signal<bool>,
    on_toggle: impl Fn() + 'static,
) -> View {
    let is_open = open.get();
    let chevron: View = if is_open {
        crate::icons::icon_chevron_down_outline(theme.text_muted, 14.0)
    } else {
        crate::icons::icon_chevron_forward_outline(theme.text_muted, 14.0)
    };

    let mut children = vec![
        button_view(
            row(vec![
                text(title, TEXT_SM).weight(FontWeight::Bold).color(theme.text).into(),
                spacer(),
                chevron,
            ])
            .gap(0.0).fill_width().padding_x(SPACE_4).padding_y(SPACE_3).align_center().into(),
            on_toggle,
        )
        .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
        .fill_width().into(),
    ];

    if is_open {
        children.push(
            column(body)
                .gap(SPACE_2).padding_x(SPACE_4).padding_bottom(SPACE_3)
                .fill_width().into()
        );
    }

    column(children)
        .gap(0.0).fill_width()
        .border(theme.border, 1.0).radius(RADIUS_LG).into()
}

/// A stack of accordion items.
pub fn accordion(items: Vec<View>) -> View {
    column(items).gap(SPACE_1).fill_width().into()
}


pub fn collapsible(
    _theme: &Theme,
    trigger: View,
    body: Vec<View>,
    open: Signal<bool>,
    on_toggle: impl Fn() + 'static,
) -> View {
    let is_open = open.get();
    let mut children = vec![
        button_view(trigger, on_toggle)
            .bg(Color::TRANSPARENT).hover_bg(Color::TRANSPARENT)
            .fill_width().into(),
    ];
    if is_open {
        children.push(column(body).gap(SPACE_2).fill_width().into());
    }
    column(children).gap(SPACE_1).fill_width().into()
}


#[derive(Clone, PartialEq)]
pub enum ToastKind { Info, Success, Warning, Error }

pub fn toast(theme: &Theme, kind: ToastKind, message: impl Into<String>, on_dismiss: impl Fn() + 'static) -> View {
    let (icon, accent): (View, Color) = match kind {
        ToastKind::Info    => (icon_information_circle_outline(dark::INFO, 16.0),    dark::INFO),
        ToastKind::Success => (icon_checkmark_circle_outline(dark::SUCCESS, 16.0),   dark::SUCCESS),
        ToastKind::Warning => (icon_warning_outline(dark::WARNING, 16.0),            dark::WARNING),
        ToastKind::Error   => (icon_alert_circle_outline(dark::DANGER, 16.0),        dark::DANGER),
    };
    row(vec![
        rect(accent).width(3.0).height(48.0).into(),
        column(vec![icon]).width(32.0).align_center().justify_center().into(),
        column(vec![text(message, TEXT_SM).color(theme.text).into()]).grow().into(),
        button_view(
            icon_close_outline(theme.text_muted, 12.0),
            on_dismiss,
        )
        .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
        .width(28.0).height(28.0).radius(14.0).into(),
        spacer_h(SPACE_1),
    ])
    .gap(0.0).fill_width().height(48.0)
    .bg(theme.surface).border(theme.border, 1.0).radius(RADIUS_XL)
    .shadow(Shadow { offset_x: 0.0, offset_y: 8.0, blur: 24.0, color: Color::rgba(0.0,0.0,0.0,0.4) })
    .align_center().into()
}

fn spacer_h(w: f32) -> View {
    rect(Color::TRANSPARENT).width(w).height(1.0).into()
}

/// A stack of toasts positioned at bottom-right.
pub fn toast_stack(toasts: Vec<View>) -> View {
    column(toasts).gap(SPACE_2).width(360.0).into()
}


/// A floating popover panel (positioned by caller in a ZStack).
pub fn popover(theme: &Theme, body: Vec<View>) -> View {
    column(body)
        .gap(SPACE_2).padding(SPACE_3)
        .bg(theme.surface).border(theme.border, 1.0).radius(RADIUS_XL)
        .shadow(Shadow { offset_x: 0.0, offset_y: 8.0, blur: 24.0, color: Color::rgba(0.0,0.0,0.0,0.4) })
        .into()
}

/// Tooltip panel (minimal, no arrow).
pub fn tooltip_panel(content: impl Into<String>) -> View {
    row(vec![text(content, TEXT_XS).color(Color::WHITE).into()])
        .padding_x(SPACE_2).padding_y(SPACE_1)
        .bg(Color::rgba(0.0, 0.0, 0.0, 0.85))
        .radius(RADIUS_MD).into()
}


pub fn dropdown_menu(theme: &Theme, items: Vec<View>) -> View {
    column(items)
        .gap(2.0).padding(SPACE_1)
        .bg(theme.surface).border(theme.border, 1.0).radius(RADIUS_LG)
        .shadow(Shadow { offset_x: 0.0, offset_y: 8.0, blur: 20.0, color: Color::rgba(0.0,0.0,0.0,0.4) })
        .into()
}

pub fn menu_item(theme: &Theme, label: impl Into<String>, icon: Option<View>, on_click: impl Fn() + 'static) -> View {
    let mut inner: Vec<View> = vec![];
    if let Some(i) = icon { inner.push(i); }
    inner.push(text(label, TEXT_SM).color(theme.text).into());
    button_view(
        row(inner).gap(SPACE_2).fill_width().padding_x(SPACE_3).padding_y(SPACE_2).align_center().into(),
        on_click,
    )
    .bg(Color::TRANSPARENT).hover_bg(dark::SURFACE_2)
    .fill_width().radius(RADIUS_MD).into()
}

pub fn menu_item_destructive(_theme: &Theme, label: impl Into<String>, icon: Option<View>, on_click: impl Fn() + 'static) -> View {
    let mut inner: Vec<View> = vec![];
    if let Some(i) = icon { inner.push(i); }
    inner.push(text(label, TEXT_SM).color(dark::DANGER).into());
    button_view(
        row(inner).gap(SPACE_2).fill_width().padding_x(SPACE_3).padding_y(SPACE_2).align_center().into(),
        on_click,
    )
    .bg(Color::TRANSPARENT).hover_bg(with_opacity(dark::DANGER, 0.10))
    .fill_width().radius(RADIUS_MD).into()
}

pub fn menu_separator(theme: &Theme) -> View {
    rect(theme.border).height(1.0).fill_width().into()
}
