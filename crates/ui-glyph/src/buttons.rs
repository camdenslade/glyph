use crate::colors::{dark, light, with_opacity};
use crate::spacing::*;
use core_glyph::{button, Color, Theme, View};

pub fn btn(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(theme.primary)
        .hover_bg(with_opacity(theme.primary, 0.85))
        .text_color(theme.on_primary)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_sm(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(theme.primary)
        .hover_bg(with_opacity(theme.primary, 0.85))
        .text_color(theme.on_primary)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_SM)
        .padding(SPACE_3)
        .font_size(TEXT_XS)
        .into()
}

pub fn btn_lg(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(theme.primary)
        .hover_bg(with_opacity(theme.primary, 0.85))
        .text_color(theme.on_primary)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_LG)
        .padding(SPACE_6)
        .font_size(TEXT_BASE)
        .into()
}

pub fn btn_xl(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(theme.primary)
        .hover_bg(with_opacity(theme.primary, 0.85))
        .text_color(theme.on_primary)
        .radius(RADIUS_2XL)
        .height(BTN_HEIGHT_XL)
        .padding(SPACE_8)
        .font_size(TEXT_LG)
        .into()
}

pub fn btn_secondary(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(theme.surface)
        .hover_bg(dark::SURFACE_3)
        .text_color(theme.text)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_secondary_sm(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(theme.surface)
        .hover_bg(dark::SURFACE_3)
        .text_color(theme.text)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_SM)
        .padding(SPACE_3)
        .font_size(TEXT_XS)
        .into()
}

pub fn btn_ghost(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(theme.surface)
        .text_color(theme.text)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_ghost_sm(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(theme.surface)
        .text_color(theme.text)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_SM)
        .padding(SPACE_3)
        .font_size(TEXT_XS)
        .into()
}

pub fn btn_ghost_muted(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(theme.surface)
        .text_color(theme.text_muted)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_danger(_theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
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

pub fn btn_danger_sm(_theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(dark::DANGER)
        .hover_bg(dark::DANGER_HOVER)
        .text_color(Color::WHITE)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_SM)
        .padding(SPACE_3)
        .font_size(TEXT_XS)
        .into()
}

pub fn btn_danger_ghost(_theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(dark::DANGER_BG)
        .text_color(dark::DANGER)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_success(label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(dark::SUCCESS)
        .hover_bg(crate::colors::GREEN_700)
        .text_color(Color::WHITE)
        .radius(RADIUS_XL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_4)
        .font_size(TEXT_SM)
        .into()
}

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

pub fn btn_pill(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(theme.primary)
        .hover_bg(with_opacity(theme.primary, 0.85))
        .text_color(theme.on_primary)
        .radius(RADIUS_FULL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_5)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_pill_secondary(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(theme.surface)
        .hover_bg(dark::SURFACE_3)
        .text_color(theme.text)
        .radius(RADIUS_FULL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_5)
        .font_size(TEXT_SM)
        .into()
}

pub fn btn_pill_ghost(theme: &Theme, label: impl Into<String>, on_click: impl Fn() + 'static) -> View {
    button(label, on_click)
        .bg(Color::TRANSPARENT)
        .hover_bg(theme.surface)
        .text_color(theme.text)
        .radius(RADIUS_FULL)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_5)
        .font_size(TEXT_SM)
        .into()
}

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
