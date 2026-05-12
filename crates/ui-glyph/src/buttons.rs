use crate::colors::{dark, light};
use crate::spacing::*;
use core_glyph::{button, Color, View};

// Primary buttons — solid filled
pub fn btn(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::ACCENT)
        .hover_bg(light::ACCENT_HOVER)
        .text_color(light::ACCENT_FG)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_sm(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::ACCENT)
        .hover_bg(light::ACCENT_HOVER)
        .text_color(light::ACCENT_FG)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_SM)
        .padding(SPACE_3)
        .font_size(TEXT_XS)
        .into()
}

pub fn btn_lg(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::ACCENT)
        .hover_bg(light::ACCENT_HOVER)
        .text_color(light::ACCENT_FG)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_LG)
        .padding(SPACE_6)
        .font_size(TEXT_BASE)
        .into()
}

pub fn btn_xl(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::ACCENT)
        .hover_bg(light::ACCENT_HOVER)
        .text_color(light::ACCENT_FG)
        .radius(RADIUS_2XL)
        .height(BTN_HEIGHT_XL)
        .padding(SPACE_8)
        .font_size(TEXT_LG)
        .into()
}

// Secondary — subtle filled
pub fn btn_secondary(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::SURFACE_2)
        .hover_bg(light::SURFACE_3)
        .text_color(light::TEXT)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_secondary_sm(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::SURFACE_2)
        .hover_bg(light::SURFACE_3)
        .text_color(light::TEXT)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_SM)
        .padding(SPACE_3)
        .font_size(TEXT_XS)
        .into()
}

// Ghost — no background until hover
pub fn btn_ghost(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(light::SURFACE_2)
        .text_color(light::TEXT)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_ghost_sm(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(light::SURFACE_2)
        .text_color(light::TEXT)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_SM)
        .padding(SPACE_3)
        .font_size(TEXT_XS)
        .into()
}

pub fn btn_ghost_muted(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(light::SURFACE_2)
        .text_color(light::TEXT_MUTED)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

// Danger
pub fn btn_danger(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::DANGER)
        .hover_bg(light::DANGER_HOVER)
        .text_color(Color::WHITE)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_danger_sm(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::DANGER)
        .hover_bg(light::DANGER_HOVER)
        .text_color(Color::WHITE)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_SM)
        .padding(SPACE_3)
        .font_size(TEXT_XS)
        .into()
}

pub fn btn_danger_ghost(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(light::DANGER_BG)
        .text_color(light::DANGER)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

// Success
pub fn btn_success(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::SUCCESS)
        .hover_bg(crate::colors::GREEN_700)
        .text_color(Color::WHITE)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

// Dark theme variants
pub fn btn_dark(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(dark::ACCENT)
        .hover_bg(dark::ACCENT_HOVER)
        .text_color(dark::ACCENT_FG)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_dark_secondary(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(dark::SURFACE_2)
        .hover_bg(dark::SURFACE_3)
        .text_color(dark::TEXT)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_dark_ghost(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(dark::SURFACE_2)
        .text_color(dark::TEXT)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_dark_ghost_muted(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(dark::SURFACE_2)
        .text_color(dark::TEXT_MUTED)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_dark_danger(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(dark::DANGER)
        .hover_bg(dark::DANGER_HOVER)
        .text_color(Color::WHITE)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

// Custom — build your own button style
pub struct BtnStyle {
    pub bg: Color,
    pub hover_bg: Color,
    pub text_color: Color,
    pub radius: f32,
    pub height: f32,
    pub padding: f32,
    pub font_size: f32,
}

impl Default for BtnStyle {
    fn default() -> Self {
        BtnStyle {
            bg: light::ACCENT,
            hover_bg: light::ACCENT_HOVER,
            text_color: Color::WHITE,
            radius: RADIUS_XL,
            height: BTN_HEIGHT_MD,
            padding: SPACE_4,
            font_size: TEXT_SM,
        }
    }
}

pub fn btn_styled(
    label: impl Into<String>,
    style: BtnStyle,
    on_click: impl Fn() + 'static,
) -> View {
    button(label, on_click)
        .bg(style.bg)
        .hover_bg(style.hover_bg)
        .text_color(style.text_color)
        .radius(style.radius)
        .height(style.height)
        .padding(style.padding)
        .font_size(style.font_size)
        .into()
}

// Pill-shaped buttons
pub fn btn_pill(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::ACCENT)
        .hover_bg(light::ACCENT_HOVER)
        .text_color(light::ACCENT_FG)
        .radius(RADIUS_FULL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_5)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_pill_secondary(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(light::SURFACE_2)
        .hover_bg(light::SURFACE_3)
        .text_color(light::TEXT)
        .radius(RADIUS_FULL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_5)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_pill_ghost(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(light::SURFACE_2)
        .text_color(light::TEXT)
        .radius(RADIUS_FULL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_5)
        .font_size(TEXT_SM)
        .into()
}

// Colored buttons from palette
pub fn btn_colored(
    label: impl Into<String>,
    bg: Color,
    fg: Color,
    on_click: impl Fn() + 'static,
) -> View {
    button(label, on_click)
        .bg(bg)
        .hover_bg(crate::colors::darken(bg, 0.05))
        .text_color(fg)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}
