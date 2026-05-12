use crate::colors::light;
use crate::spacing::*;
use core_glyph::{text, Color, FontWeight, TextAlign, View};

// Headings
pub fn display(content: impl Into<String>) -> View {
    text(content, TEXT_5XL).weight(FontWeight::Bold).into()
}
pub fn h1(content: impl Into<String>) -> View {
    text(content, TEXT_4XL).weight(FontWeight::Bold).into()
}
pub fn h2(content: impl Into<String>) -> View {
    text(content, TEXT_3XL).weight(FontWeight::Bold).into()
}
pub fn h3(content: impl Into<String>) -> View {
    text(content, TEXT_2XL).weight(FontWeight::Bold).into()
}
pub fn h4(content: impl Into<String>) -> View {
    text(content, TEXT_XL).weight(FontWeight::Bold).into()
}
pub fn h5(content: impl Into<String>) -> View {
    text(content, TEXT_LG).weight(FontWeight::Bold).into()
}
pub fn h6(content: impl Into<String>) -> View {
    text(content, TEXT_BASE).weight(FontWeight::Bold).into()
}

// Colored heading variants
pub fn h1_colored(content: impl Into<String>, color: Color) -> View {
    text(content, TEXT_4XL)
        .weight(FontWeight::Bold)
        .color(color)
        .into()
}
pub fn h2_colored(content: impl Into<String>, color: Color) -> View {
    text(content, TEXT_3XL)
        .weight(FontWeight::Bold)
        .color(color)
        .into()
}
pub fn h3_colored(content: impl Into<String>, color: Color) -> View {
    text(content, TEXT_2XL)
        .weight(FontWeight::Bold)
        .color(color)
        .into()
}

// Body text
pub fn body_lg(content: impl Into<String>) -> View {
    text(content, TEXT_LG).into()
}
pub fn body(content: impl Into<String>) -> View {
    text(content, TEXT_BASE).into()
}
pub fn body_sm(content: impl Into<String>) -> View {
    text(content, TEXT_SM).into()
}

// Colored body
pub fn body_colored(content: impl Into<String>, color: Color) -> View {
    text(content, TEXT_BASE).color(color).into()
}
pub fn body_sm_colored(content: impl Into<String>, color: Color) -> View {
    text(content, TEXT_SM).color(color).into()
}

// Semantic text styles
pub fn muted(content: impl Into<String>) -> View {
    text(content, TEXT_BASE).color(light::TEXT_MUTED).into()
}
pub fn muted_sm(content: impl Into<String>) -> View {
    text(content, TEXT_SM).color(light::TEXT_MUTED).into()
}
pub fn muted_xs(content: impl Into<String>) -> View {
    text(content, TEXT_XS).color(light::TEXT_MUTED).into()
}
pub fn subtle(content: impl Into<String>) -> View {
    text(content, TEXT_SM).color(light::TEXT_SUBTLE).into()
}

pub fn caption(content: impl Into<String>) -> View {
    text(content, TEXT_XS).color(light::TEXT_MUTED).into()
}
pub fn caption_colored(content: impl Into<String>, color: Color) -> View {
    text(content, TEXT_XS).color(color).into()
}

pub fn label(content: impl Into<String>) -> View {
    text(content, TEXT_SM).weight(FontWeight::Bold).into()
}
pub fn label_muted(content: impl Into<String>) -> View {
    text(content, TEXT_SM)
        .weight(FontWeight::Bold)
        .color(light::TEXT_MUTED)
        .into()
}

pub fn overline(content: impl Into<String>) -> View {
    text(content, TEXT_XS)
        .weight(FontWeight::Bold)
        .color(light::TEXT_MUTED)
        .into()
}

// Links
pub fn link(content: impl Into<String>) -> View {
    text(content, TEXT_BASE).color(light::ACCENT).into()
}
pub fn link_sm(content: impl Into<String>) -> View {
    text(content, TEXT_SM).color(light::ACCENT).into()
}

// Code / mono
pub fn code_inline(content: impl Into<String>) -> View {
    text(content, TEXT_SM).into()
}

// Alignment helpers
pub fn center(content: impl Into<String>, size: f32) -> View {
    text(content, size).align(TextAlign::Center).into()
}
pub fn right(content: impl Into<String>, size: f32) -> View {
    text(content, size).align(TextAlign::Right).into()
}
pub fn center_bold(content: impl Into<String>, size: f32) -> View {
    text(content, size)
        .align(TextAlign::Center)
        .weight(FontWeight::Bold)
        .into()
}

// Numeric / tabular
pub fn numeric(content: impl Into<String>) -> View {
    text(content, TEXT_BASE).weight(FontWeight::Bold).into()
}
pub fn numeric_lg(content: impl Into<String>) -> View {
    text(content, TEXT_3XL).weight(FontWeight::Bold).into()
}
pub fn numeric_xl(content: impl Into<String>) -> View {
    text(content, TEXT_5XL).weight(FontWeight::Bold).into()
}

// Wrapping paragraph
pub fn paragraph(content: impl Into<String>) -> View {
    text(content, TEXT_BASE).wrap().into()
}
pub fn paragraph_sm(content: impl Into<String>) -> View {
    text(content, TEXT_SM).wrap().into()
}
pub fn paragraph_colored(content: impl Into<String>, color: Color) -> View {
    text(content, TEXT_BASE).color(color).wrap().into()
}
