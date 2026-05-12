use crate::colors::{dark, light};
use crate::spacing::RADIUS_XL;
use glyph_core::{Color, Theme};

pub fn light_theme() -> Theme {
    Theme {
        background: light::BG,
        surface: light::SURFACE,
        primary: light::ACCENT,
        on_primary: Color::WHITE,
        text: light::TEXT,
        text_muted: light::TEXT_MUTED,
        border: light::BORDER,
        border_focused: light::ACCENT,
        radius: RADIUS_XL,
        font_size: 15.0,
    }
}

pub fn dark_theme() -> Theme {
    Theme {
        background: dark::BG,
        surface: dark::SURFACE,
        primary: dark::ACCENT,
        on_primary: Color::WHITE,
        text: dark::TEXT,
        text_muted: dark::TEXT_MUTED,
        border: dark::BORDER,
        border_focused: dark::ACCENT,
        radius: RADIUS_XL,
        font_size: 15.0,
    }
}

pub fn slate_dark_theme() -> Theme {
    use crate::colors::{
        BLUE_400, BLUE_500, SLATE_100, SLATE_400, SLATE_800, SLATE_900, SLATE_950,
    };
    Theme {
        background: SLATE_950,
        surface: SLATE_900,
        primary: BLUE_500,
        on_primary: Color::WHITE,
        text: SLATE_100,
        text_muted: SLATE_400,
        border: SLATE_800,
        border_focused: BLUE_400,
        radius: RADIUS_XL,
        font_size: 15.0,
    }
}

pub fn github_dark_theme() -> Theme {
    Theme {
        background: Color::rgb(0.051, 0.067, 0.090),
        surface: Color::rgb(0.086, 0.106, 0.133),
        primary: Color::rgb(0.345, 0.651, 1.000),
        on_primary: Color::rgb(0.902, 0.929, 0.961),
        text: Color::rgb(0.902, 0.929, 0.961),
        text_muted: Color::rgb(0.486, 0.549, 0.624),
        border: Color::rgb(0.188, 0.212, 0.243),
        border_focused: Color::rgb(0.345, 0.651, 1.000),
        radius: 6.0,
        font_size: 14.0,
    }
}

pub fn minimal_light_theme() -> Theme {
    Theme {
        background: Color::WHITE,
        surface: Color::WHITE,
        primary: Color::BLACK,
        on_primary: Color::WHITE,
        text: Color::BLACK,
        text_muted: Color::rgb(0.5, 0.5, 0.5),
        border: Color::rgb(0.9, 0.9, 0.9),
        border_focused: Color::BLACK,
        radius: 4.0,
        font_size: 15.0,
    }
}
