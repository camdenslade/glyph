use crate::colors::light;
use crate::spacing::*;
use glyph_core::{column, flexible, rect, row, spacer, AlignItems, Color, JustifyContent, View};

// Spacers — vertical gaps as invisible rects
pub fn gap(h: f32) -> View {
    rect(Color::TRANSPARENT).height(h).width(1.0).into()
}
pub fn hgap(w: f32) -> View {
    rect(Color::TRANSPARENT).width(w).height(1.0).into()
}

// Convenience gap sizes
pub fn gap_1() -> View {
    gap(SPACE_1)
}
pub fn gap_2() -> View {
    gap(SPACE_2)
}
pub fn gap_3() -> View {
    gap(SPACE_3)
}
pub fn gap_4() -> View {
    gap(SPACE_4)
}
pub fn gap_6() -> View {
    gap(SPACE_6)
}
pub fn gap_8() -> View {
    gap(SPACE_8)
}
pub fn gap_12() -> View {
    gap(SPACE_12)
}
pub fn gap_16() -> View {
    gap(SPACE_16)
}

pub fn hgap_2() -> View {
    hgap(SPACE_2)
}
pub fn hgap_4() -> View {
    hgap(SPACE_4)
}
pub fn hgap_6() -> View {
    hgap(SPACE_6)
}
pub fn hgap_8() -> View {
    hgap(SPACE_8)
}

// Dividers
pub fn divider() -> View {
    rect(light::BORDER).height(1.0).fill_width().into()
}
pub fn divider_colored(color: Color) -> View {
    rect(color).height(1.0).fill_width().into()
}
pub fn divider_v(height: f32) -> View {
    rect(light::BORDER).width(1.0).height(height).into()
}
pub fn divider_v_colored(color: Color, height: f32) -> View {
    rect(color).width(1.0).height(height).into()
}

// Dividers for dark theme
pub fn divider_dark() -> View {
    rect(crate::colors::dark::BORDER)
        .height(1.0)
        .fill_width()
        .into()
}
pub fn divider_dark_v(height: f32) -> View {
    rect(crate::colors::dark::BORDER)
        .width(1.0)
        .height(height)
        .into()
}

// Centered container — max-width with horizontal centering
pub fn container(width: f32, children: Vec<View>) -> View {
    column(children).width(width).into()
}

pub fn container_sm(children: Vec<View>) -> View {
    container(640.0, children)
}
pub fn container_md(children: Vec<View>) -> View {
    container(768.0, children)
}
pub fn container_lg(children: Vec<View>) -> View {
    container(1024.0, children)
}
pub fn container_xl(children: Vec<View>) -> View {
    container(1280.0, children)
}
pub fn container_2xl(children: Vec<View>) -> View {
    container(1536.0, children)
}

// Padded wrappers
pub fn padded(p: f32, child: View) -> View {
    column(vec![child]).padding(p).into()
}
pub fn padded_x(p: f32, child: View) -> View {
    column(vec![child]).padding_x(p).into()
}
pub fn padded_y(p: f32, child: View) -> View {
    column(vec![child]).padding_y(p).into()
}

// Stack helpers
pub fn vstack(gap: f32, children: Vec<View>) -> View {
    column(children).gap(gap).into()
}
pub fn hstack(gap: f32, children: Vec<View>) -> View {
    row(children).gap(gap).into()
}

// Centered content in a row or column
pub fn center_h(children: Vec<View>) -> View {
    row(children).justify(JustifyContent::Center).into()
}
pub fn center_v(children: Vec<View>) -> View {
    column(children).align(AlignItems::Center).into()
}
pub fn center_both(children: Vec<View>) -> View {
    column(children)
        .align(AlignItems::Center)
        .justify(JustifyContent::Center)
        .into()
}

// Common two-panel layouts
pub fn sidebar_layout(sidebar_width: f32, sidebar: Vec<View>, content: Vec<View>) -> View {
    row(vec![
        column(sidebar).width(sidebar_width).into(),
        column(content).grow().fill_width().into(),
    ])
    .gap(0.0)
    .fill_width()
    .into()
}

pub fn sidebar_layout_right(content_width: f32, content: Vec<View>, aside: Vec<View>) -> View {
    row(vec![
        column(content).grow().fill_width().into(),
        column(aside).width(content_width).into(),
    ])
    .gap(0.0)
    .fill_width()
    .into()
}

// Header + scrollable body
pub fn page(header: View, body: View) -> View {
    column(vec![header, body]).gap(0.0).fill_width().into()
}

// Spacer that pushes items to opposite ends
pub fn between(left: View, right: View) -> View {
    row(vec![left, spacer(), right]).fill_width().into()
}

pub fn between_many(items: Vec<View>) -> View {
    row(items)
        .fill_width()
        .justify(JustifyContent::SpaceBetween)
        .into()
}

// Aspect ratio helpers — wrap a fixed-height column
pub fn aspect_square(size: f32, child: View) -> View {
    column(vec![child]).width(size).height(size).into()
}
pub fn aspect_video(width: f32, child: View) -> View {
    column(vec![child])
        .width(width)
        .height(width * 9.0 / 16.0)
        .into()
}

// Overlay — transparent full-cover rect for modals/backdrops
pub fn backdrop(alpha: f32) -> View {
    rect(Color::rgba(0.0, 0.0, 0.0, alpha)).fill_width().into()
}

// Inset container — colored background region
pub fn inset(bg: Color, radius: f32, padding: f32, children: Vec<View>) -> View {
    column(children)
        .bg(bg)
        .radius(radius)
        .padding(padding)
        .into()
}

// Responsive grid rows — evenly-spaced row items
pub fn grid_row(gap: f32, items: Vec<View>) -> View {
    row(items.into_iter().map(flexible).collect())
        .gap(gap)
        .fill_width()
        .into()
}

pub fn grid_row_2(gap: f32, a: View, b: View) -> View {
    grid_row(gap, vec![a, b])
}
pub fn grid_row_3(gap: f32, a: View, b: View, c: View) -> View {
    grid_row(gap, vec![a, b, c])
}
pub fn grid_row_4(gap: f32, a: View, b: View, c: View, d: View) -> View {
    grid_row(gap, vec![a, b, c, d])
}
