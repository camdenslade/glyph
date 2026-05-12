use crate::colors::{dark, light, with_opacity};
use crate::layout::{divider, divider_dark};
use crate::shadows::{shadow_dark_md, shadow_md, shadow_sm};
use crate::spacing::*;
use glyph_core::{
    button, column, flexible, image, rect, row, spacer, text, AlignItems, Color, FontWeight,
    JustifyContent, Shadow, View,
};

// Card — the fundamental surface container

pub struct CardOptions {
    pub bg: Color,
    pub border: Option<Color>,
    pub border_width: f32,
    pub radius: f32,
    pub padding: f32,
    pub shadow: Option<Shadow>,
    pub gap: f32,
}

impl CardOptions {
    pub fn light() -> Self {
        CardOptions {
            bg: light::SURFACE,
            border: Some(light::BORDER),
            border_width: 1.0,
            radius: RADIUS_XL,
            padding: SPACE_4,
            shadow: Some(shadow_sm()),
            gap: SPACE_3,
        }
    }
    pub fn dark() -> Self {
        CardOptions {
            bg: dark::SURFACE,
            border: Some(dark::BORDER),
            border_width: 1.0,
            radius: RADIUS_XL,
            padding: SPACE_4,
            shadow: Some(shadow_dark_md()),
            gap: SPACE_3,
        }
    }
    pub fn flat() -> Self {
        CardOptions {
            border: None,
            shadow: None,
            ..CardOptions::light()
        }
    }
    pub fn elevated() -> Self {
        CardOptions {
            shadow: Some(shadow_md()),
            border: None,
            ..CardOptions::light()
        }
    }
}

pub fn card(children: Vec<View>) -> View {
    card_opts(children, CardOptions::light())
}
pub fn card_dark(children: Vec<View>) -> View {
    card_opts(children, CardOptions::dark())
}
pub fn card_flat(children: Vec<View>) -> View {
    card_opts(children, CardOptions::flat())
}
pub fn card_elevated(children: Vec<View>) -> View {
    card_opts(children, CardOptions::elevated())
}

pub fn card_opts(children: Vec<View>, opts: CardOptions) -> View {
    let mut col = column(children)
        .gap(opts.gap)
        .padding(opts.padding)
        .radius(opts.radius)
        .bg(opts.bg);
    if let Some(bc) = opts.border {
        col = col.border(bc, opts.border_width);
    }
    if let Some(sh) = opts.shadow {
        col = col.shadow(sh);
    }
    col.into()
}

// Card with header/body/footer sections
pub fn card_section(header: View, body: Vec<View>) -> View {
    column(vec![
        column(vec![header])
            .padding_x(SPACE_4)
            .padding_y(SPACE_3)
            .into(),
        divider(),
        column(body).gap(SPACE_3).padding(SPACE_4).into(),
    ])
    .gap(0.0)
    .bg(light::SURFACE)
    .border(light::BORDER, 1.0)
    .radius(RADIUS_XL)
    .shadow(shadow_sm())
    .into()
}

pub fn card_section_footer(header: View, body: Vec<View>, footer: Vec<View>) -> View {
    column(vec![
        column(vec![header])
            .padding_x(SPACE_4)
            .padding_y(SPACE_3)
            .into(),
        divider(),
        column(body).gap(SPACE_3).padding(SPACE_4).into(),
        divider(),
        row(footer).gap(SPACE_2).padding(SPACE_4).into(),
    ])
    .gap(0.0)
    .bg(light::SURFACE)
    .border(light::BORDER, 1.0)
    .radius(RADIUS_XL)
    .into()
}

// Badge — small status/label indicator
pub fn badge(label: impl Into<String>) -> View {
    badge_colored(label, light::INFO_BG, light::INFO_FG)
}
pub fn badge_success(label: impl Into<String>) -> View {
    badge_colored(label, light::SUCCESS_BG, light::SUCCESS_FG)
}
pub fn badge_warning(label: impl Into<String>) -> View {
    badge_colored(label, light::WARNING_BG, light::WARNING_FG)
}
pub fn badge_danger(label: impl Into<String>) -> View {
    badge_colored(label, light::DANGER_BG, light::DANGER_FG)
}
pub fn badge_neutral(label: impl Into<String>) -> View {
    badge_colored(label, light::SURFACE_3, light::TEXT_MUTED)
}
pub fn badge_dark(label: impl Into<String>) -> View {
    badge_colored(label, dark::SURFACE_2, dark::TEXT_MUTED)
}

pub fn badge_colored(label: impl Into<String>, bg: Color, fg: Color) -> View {
    row(vec![text(label, TEXT_XS)
        .weight(FontWeight::Bold)
        .color(fg)
        .into()])
    .padding_x(SPACE_2)
    .padding_y(2.0)
    .bg(bg)
    .radius(RADIUS_FULL)
    .into()
}

pub fn badge_dot(label: impl Into<String>, dot_color: Color) -> View {
    row(vec![
        rect(dot_color).width(6.0).height(6.0).radius(3.0).into(),
        text(label, TEXT_XS).color(light::TEXT_MUTED).into(),
    ])
    .gap(SPACE_1)
    .padding_x(SPACE_2)
    .padding_y(2.0)
    .bg(light::SURFACE_2)
    .radius(RADIUS_FULL)
    .into()
}

// Pill — wider badge variant
pub fn pill(label: impl Into<String>, bg: Color, fg: Color) -> View {
    row(vec![text(label, TEXT_SM)
        .weight(FontWeight::Bold)
        .color(fg)
        .into()])
    .padding_x(SPACE_3)
    .padding_y(SPACE_1)
    .bg(bg)
    .radius(RADIUS_FULL)
    .into()
}
pub fn pill_primary(label: impl Into<String>) -> View {
    pill(label, with_opacity(light::ACCENT, 0.12), light::ACCENT)
}
pub fn pill_success(label: impl Into<String>) -> View {
    pill(label, light::SUCCESS_BG, light::SUCCESS_FG)
}
pub fn pill_danger(label: impl Into<String>) -> View {
    pill(label, light::DANGER_BG, light::DANGER_FG)
}
pub fn pill_warning(label: impl Into<String>) -> View {
    pill(label, light::WARNING_BG, light::WARNING_FG)
}
pub fn pill_neutral(label: impl Into<String>) -> View {
    pill(label, light::SURFACE_2, light::TEXT_MUTED)
}

// Tag — bordered square chip
pub fn tag(label: impl Into<String>) -> View {
    row(vec![text(label, TEXT_XS).color(light::TEXT_MUTED).into()])
        .padding_x(SPACE_2)
        .padding_y(2.0)
        .bg(light::SURFACE)
        .border(light::BORDER, 1.0)
        .radius(RADIUS_MD)
        .into()
}
pub fn tag_dark(label: impl Into<String>) -> View {
    row(vec![text(label, TEXT_XS).color(dark::TEXT_MUTED).into()])
        .padding_x(SPACE_2)
        .padding_y(2.0)
        .bg(dark::SURFACE_2)
        .border(dark::BORDER, 1.0)
        .radius(RADIUS_MD)
        .into()
}
pub fn tag_colored(label: impl Into<String>, bg: Color, border: Color, fg: Color) -> View {
    row(vec![text(label, TEXT_XS).color(fg).into()])
        .padding_x(SPACE_2)
        .padding_y(2.0)
        .bg(bg)
        .border(border, 1.0)
        .radius(RADIUS_MD)
        .into()
}

// Avatar
pub fn avatar(path: impl Into<String>, size: f32) -> View {
    column(vec![image(path).size(size, size).radius(size / 2.0).into()])
        .width(size)
        .height(size)
        .into()
}

pub fn avatar_placeholder(initials: impl Into<String>, size: f32, bg: Color, fg: Color) -> View {
    column(vec![text(initials, size * 0.38)
        .weight(FontWeight::Bold)
        .color(fg)
        .into()])
    .width(size)
    .height(size)
    .bg(bg)
    .radius(size / 2.0)
    .align(AlignItems::Center)
    .justify(JustifyContent::Center)
    .into()
}

pub fn avatar_xs(path: impl Into<String>) -> View {
    avatar(path, AVATAR_XS)
}
pub fn avatar_sm(path: impl Into<String>) -> View {
    avatar(path, AVATAR_SM)
}
pub fn avatar_md(path: impl Into<String>) -> View {
    avatar(path, AVATAR_MD)
}
pub fn avatar_lg(path: impl Into<String>) -> View {
    avatar(path, AVATAR_LG)
}
pub fn avatar_xl(path: impl Into<String>) -> View {
    avatar(path, AVATAR_XL)
}

pub fn avatar_placeholder_md(initials: impl Into<String>, bg: Color) -> View {
    avatar_placeholder(initials, AVATAR_MD, bg, Color::WHITE)
}
pub fn avatar_placeholder_lg(initials: impl Into<String>, bg: Color) -> View {
    avatar_placeholder(initials, AVATAR_LG, bg, Color::WHITE)
}
pub fn avatar_placeholder_xl(initials: impl Into<String>, bg: Color) -> View {
    avatar_placeholder(initials, AVATAR_XL, bg, Color::WHITE)
}

// Alert / notification banner
pub fn alert_info(title: impl Into<String>, message: impl Into<String>) -> View {
    alert_colored(title, message, light::INFO_BG, light::INFO_FG, light::INFO)
}
pub fn alert_success(title: impl Into<String>, message: impl Into<String>) -> View {
    alert_colored(
        title,
        message,
        light::SUCCESS_BG,
        light::SUCCESS_FG,
        light::SUCCESS,
    )
}
pub fn alert_warning(title: impl Into<String>, message: impl Into<String>) -> View {
    alert_colored(
        title,
        message,
        light::WARNING_BG,
        light::WARNING_FG,
        light::WARNING,
    )
}
pub fn alert_danger(title: impl Into<String>, message: impl Into<String>) -> View {
    alert_colored(
        title,
        message,
        light::DANGER_BG,
        light::DANGER_FG,
        light::DANGER,
    )
}

fn alert_colored(
    title: impl Into<String>,
    message: impl Into<String>,
    bg: Color,
    fg: Color,
    accent: Color,
) -> View {
    row(vec![
        rect(accent).width(3.0).fill_width().into(),
        column(vec![
            text(title, TEXT_SM)
                .weight(FontWeight::Bold)
                .color(fg)
                .into(),
            text(message, TEXT_SM).color(fg).wrap().into(),
        ])
        .gap(SPACE_1)
        .padding(SPACE_3)
        .fill_width()
        .into(),
    ])
    .gap(0.0)
    .bg(bg)
    .radius(RADIUS_LG)
    .border(with_opacity(accent, 0.3), 1.0)
    .fill_width()
    .into()
}

pub fn alert_info_dark(title: impl Into<String>, message: impl Into<String>) -> View {
    alert_colored_dark(title, message, dark::INFO_BG, dark::INFO_FG, dark::INFO)
}
pub fn alert_success_dark(title: impl Into<String>, message: impl Into<String>) -> View {
    alert_colored_dark(
        title,
        message,
        dark::SUCCESS_BG,
        dark::SUCCESS_FG,
        dark::SUCCESS,
    )
}
pub fn alert_warning_dark(title: impl Into<String>, message: impl Into<String>) -> View {
    alert_colored_dark(
        title,
        message,
        dark::WARNING_BG,
        dark::WARNING_FG,
        dark::WARNING,
    )
}
pub fn alert_danger_dark(title: impl Into<String>, message: impl Into<String>) -> View {
    alert_colored_dark(
        title,
        message,
        dark::DANGER_BG,
        dark::DANGER_FG,
        dark::DANGER,
    )
}

fn alert_colored_dark(
    title: impl Into<String>,
    message: impl Into<String>,
    bg: Color,
    fg: Color,
    accent: Color,
) -> View {
    alert_colored(title, message, bg, fg, accent)
}

// Stat card — metric display
pub fn stat_card(label: impl Into<String>, value: impl Into<String>) -> View {
    card(vec![
        text(label, TEXT_XS)
            .weight(FontWeight::Bold)
            .color(light::TEXT_MUTED)
            .into(),
        text(value, TEXT_3XL)
            .weight(FontWeight::Bold)
            .color(light::TEXT)
            .into(),
    ])
}
pub fn stat_card_dark(label: impl Into<String>, value: impl Into<String>) -> View {
    card_dark(vec![
        text(label, TEXT_XS)
            .weight(FontWeight::Bold)
            .color(dark::TEXT_MUTED)
            .into(),
        text(value, TEXT_3XL)
            .weight(FontWeight::Bold)
            .color(dark::TEXT)
            .into(),
    ])
}
pub fn stat_card_with_change(
    label: impl Into<String>,
    value: impl Into<String>,
    change: impl Into<String>,
    positive: bool,
) -> View {
    let change_color = if positive {
        light::SUCCESS
    } else {
        light::DANGER
    };
    card(vec![
        text(label, TEXT_XS)
            .weight(FontWeight::Bold)
            .color(light::TEXT_MUTED)
            .into(),
        row(vec![
            text(value, TEXT_3XL)
                .weight(FontWeight::Bold)
                .color(light::TEXT)
                .into(),
            spacer(),
            text(change, TEXT_SM)
                .weight(FontWeight::Bold)
                .color(change_color)
                .into(),
        ])
        .fill_width()
        .into(),
    ])
}

// Progress bar
pub fn progress(value: f32, max: f32, color: Color) -> View {
    let _pct = (value / max).clamp(0.0, 1.0);
    row(vec![flexible(
        rect(color).height(6.0).radius(3.0).fill_width(),
    )])
    .fill_width()
    .bg(light::SURFACE_3)
    .radius(3.0)
    .into()
}

pub fn progress_bar(pct: f32, fg: Color, bg: Color, height: f32) -> View {
    let pct = pct.clamp(0.0, 1.0);
    row(vec![rect(fg)
        .width(pct * 200.0)
        .height(height)
        .radius(height / 2.0)
        .into()])
    .width(200.0)
    .height(height)
    .bg(bg)
    .radius(height / 2.0)
    .into()
}

// Dividers with labels
pub fn divider_with_label(label: impl Into<String>) -> View {
    row(vec![
        flexible(rect(light::BORDER).height(1.0).fill_width()),
        text(label, TEXT_XS).color(light::TEXT_MUTED).into(),
        flexible(rect(light::BORDER).height(1.0).fill_width()),
    ])
    .gap(SPACE_3)
    .fill_width()
    .into()
}

pub fn divider_with_label_dark(label: impl Into<String>) -> View {
    row(vec![
        flexible(rect(dark::BORDER).height(1.0).fill_width()),
        text(label, TEXT_XS).color(dark::TEXT_MUTED).into(),
        flexible(rect(dark::BORDER).height(1.0).fill_width()),
    ])
    .gap(SPACE_3)
    .fill_width()
    .into()
}

// Section headers
pub fn section_header(title: impl Into<String>) -> View {
    column(vec![
        text(title, TEXT_LG)
            .weight(FontWeight::Bold)
            .color(light::TEXT)
            .into(),
        divider(),
    ])
    .gap(SPACE_2)
    .padding_y(SPACE_2)
    .into()
}

pub fn section_header_with_action(title: impl Into<String>, action: View) -> View {
    column(vec![
        row(vec![
            text(title, TEXT_LG)
                .weight(FontWeight::Bold)
                .color(light::TEXT)
                .into(),
            spacer(),
            action,
        ])
        .fill_width()
        .into(),
        divider(),
    ])
    .gap(SPACE_2)
    .padding_y(SPACE_2)
    .into()
}

pub fn section_header_dark(title: impl Into<String>) -> View {
    column(vec![
        text(title, TEXT_LG)
            .weight(FontWeight::Bold)
            .color(dark::TEXT)
            .into(),
        divider_dark(),
    ])
    .gap(SPACE_2)
    .padding_y(SPACE_2)
    .into()
}

// List rows
pub fn list_row(children: Vec<View>) -> View {
    row(children)
        .padding_y(SPACE_3)
        .padding_x(SPACE_4)
        .fill_width()
        .into()
}

pub fn list_row_divided(children: Vec<View>) -> View {
    column(vec![list_row(children), divider()]).gap(0.0).into()
}

pub fn list_row_dark(children: Vec<View>) -> View {
    row(children)
        .padding_y(SPACE_3)
        .padding_x(SPACE_4)
        .fill_width()
        .into()
}

pub fn list_row_dark_divided(children: Vec<View>) -> View {
    column(vec![list_row_dark(children), divider_dark()])
        .gap(0.0)
        .into()
}

// Two-line list item — title + subtitle
pub fn list_item(title: impl Into<String>, subtitle: impl Into<String>) -> View {
    list_row(vec![column(vec![
        text(title, TEXT_BASE)
            .weight(FontWeight::Bold)
            .color(light::TEXT)
            .into(),
        text(subtitle, TEXT_SM).color(light::TEXT_MUTED).into(),
    ])
    .gap(SPACE_0_5)
    .into()])
}

pub fn list_item_with_trailing(
    title: impl Into<String>,
    subtitle: impl Into<String>,
    trailing: View,
) -> View {
    list_row(vec![
        column(vec![
            text(title, TEXT_BASE)
                .weight(FontWeight::Bold)
                .color(light::TEXT)
                .into(),
            text(subtitle, TEXT_SM).color(light::TEXT_MUTED).into(),
        ])
        .gap(SPACE_0_5)
        .grow()
        .into(),
        trailing,
    ])
}

// Loading indicator (text-based, no spinner primitives)
pub fn loading(message: impl Into<String>) -> View {
    column(vec![text(message, TEXT_SM).color(light::TEXT_MUTED).into()])
        .align(AlignItems::Center)
        .padding(SPACE_8)
        .into()
}
pub fn loading_dark(message: impl Into<String>) -> View {
    column(vec![text(message, TEXT_SM).color(dark::TEXT_MUTED).into()])
        .align(AlignItems::Center)
        .padding(SPACE_8)
        .into()
}

// Empty state
pub fn empty_state(title: impl Into<String>, description: impl Into<String>) -> View {
    column(vec![
        text(title, TEXT_LG)
            .weight(FontWeight::Bold)
            .color(light::TEXT)
            .into(),
        text(description, TEXT_SM)
            .color(light::TEXT_MUTED)
            .wrap()
            .into(),
    ])
    .gap(SPACE_2)
    .align(AlignItems::Center)
    .padding(SPACE_16)
    .into()
}

pub fn empty_state_with_action(
    title: impl Into<String>,
    description: impl Into<String>,
    action: View,
) -> View {
    column(vec![
        text(title, TEXT_LG)
            .weight(FontWeight::Bold)
            .color(light::TEXT)
            .into(),
        text(description, TEXT_SM)
            .color(light::TEXT_MUTED)
            .wrap()
            .into(),
        action,
    ])
    .gap(SPACE_3)
    .align(AlignItems::Center)
    .padding(SPACE_16)
    .into()
}

pub fn empty_state_dark(title: impl Into<String>, description: impl Into<String>) -> View {
    column(vec![
        text(title, TEXT_LG)
            .weight(FontWeight::Bold)
            .color(dark::TEXT)
            .into(),
        text(description, TEXT_SM)
            .color(dark::TEXT_MUTED)
            .wrap()
            .into(),
    ])
    .gap(SPACE_2)
    .align(AlignItems::Center)
    .padding(SPACE_16)
    .into()
}

// Icon with label row
pub fn icon_label(icon_path: impl Into<String>, label: impl Into<String>, icon_size: f32) -> View {
    row(vec![
        image(icon_path).size(icon_size, icon_size).into(),
        text(label, TEXT_SM).color(light::TEXT).into(),
    ])
    .gap(SPACE_2)
    .into()
}

pub fn icon_label_muted(
    icon_path: impl Into<String>,
    label: impl Into<String>,
    icon_size: f32,
) -> View {
    row(vec![
        image(icon_path).size(icon_size, icon_size).into(),
        text(label, TEXT_SM).color(light::TEXT_MUTED).into(),
    ])
    .gap(SPACE_2)
    .into()
}

// Tooltip-style callout (positioned by caller)
pub fn tooltip(content: impl Into<String>) -> View {
    row(vec![text(content, TEXT_XS).color(Color::WHITE).into()])
        .padding_x(SPACE_2)
        .padding_y(SPACE_1)
        .bg(Color::rgba(0.0, 0.0, 0.0, 0.85))
        .radius(RADIUS_MD)
        .into()
}

// Keyboard shortcut chip
pub fn kbd(key: impl Into<String>) -> View {
    row(vec![text(key, TEXT_XS)
        .weight(FontWeight::Bold)
        .color(light::TEXT_MUTED)
        .into()])
    .padding_x(SPACE_1)
    .padding_y(1.0)
    .bg(light::SURFACE)
    .border(light::BORDER_STRONG, 1.0)
    .radius(RADIUS_MD)
    .into()
}

// Number / count bubble
pub fn count_bubble(n: u32, color: Color) -> View {
    let label = if n > 99 {
        "99+".to_string()
    } else {
        n.to_string()
    };
    row(vec![text(label, TEXT_XS)
        .weight(FontWeight::Bold)
        .color(Color::WHITE)
        .into()])
    .padding_x(SPACE_1)
    .padding_y(1.0)
    .bg(color)
    .radius(RADIUS_FULL)
    .into()
}

// Horizontal rule with spacing
pub fn hr() -> View {
    column(vec![divider()]).padding_y(SPACE_4).into()
}
pub fn hr_dark() -> View {
    column(vec![divider_dark()]).padding_y(SPACE_4).into()
}

// Colored dot indicators
pub fn dot(color: Color, size: f32) -> View {
    rect(color)
        .width(size)
        .height(size)
        .radius(size / 2.0)
        .into()
}
pub fn dot_online() -> View {
    dot(light::SUCCESS, 8.0)
}
pub fn dot_offline() -> View {
    dot(light::TEXT_SUBTLE, 8.0)
}
pub fn dot_busy() -> View {
    dot(light::WARNING, 8.0)
}
pub fn dot_error() -> View {
    dot(light::DANGER, 8.0)
}

// Skeleton / placeholder loading block
pub fn skeleton(width: f32, height: f32) -> View {
    rect(light::SURFACE_3)
        .width(width)
        .height(height)
        .radius(RADIUS_MD)
        .into()
}
pub fn skeleton_dark(width: f32, height: f32) -> View {
    rect(dark::SURFACE_2)
        .width(width)
        .height(height)
        .radius(RADIUS_MD)
        .into()
}
pub fn skeleton_text(width: f32) -> View {
    skeleton(width, TEXT_BASE)
}
pub fn skeleton_text_sm(width: f32) -> View {
    skeleton(width, TEXT_SM)
}
pub fn skeleton_avatar(size: f32) -> View {
    rect(light::SURFACE_3)
        .width(size)
        .height(size)
        .radius(size / 2.0)
        .into()
}

// Code block
pub fn code_block(content: impl Into<String>) -> View {
    column(vec![text(content, TEXT_SM)
        .wrap()
        .color(light::TEXT)
        .into()])
    .padding(SPACE_4)
    .bg(light::SURFACE_2)
    .border(light::BORDER, 1.0)
    .radius(RADIUS_LG)
    .into()
}
pub fn code_block_dark(content: impl Into<String>) -> View {
    column(vec![text(content, TEXT_SM).wrap().color(dark::TEXT).into()])
        .padding(SPACE_4)
        .bg(dark::BG_SUBTLE)
        .border(dark::BORDER, 1.0)
        .radius(RADIUS_LG)
        .into()
}

// Inline code
pub fn code_inline(content: impl Into<String>) -> View {
    row(vec![text(content, TEXT_SM).color(light::TEXT).into()])
        .padding_x(SPACE_1)
        .padding_y(1.0)
        .bg(light::SURFACE_2)
        .border(light::BORDER, 1.0)
        .radius(RADIUS_MD)
        .into()
}

// Tab bar — horizontal navigation tabs
pub fn tab_bar(tabs: Vec<(&str, bool)>, on_select: impl Fn(usize) + 'static + Clone) -> View {
    let items: Vec<View> = tabs
        .into_iter()
        .enumerate()
        .map(|(i, (label, active))| {
            let on_select = on_select.clone();
            let bg = if active {
                light::ACCENT
            } else {
                Color::TRANSPARENT
            };
            let fg = if active {
                Color::WHITE
            } else {
                light::TEXT_MUTED
            };
            let hover = if active {
                light::ACCENT
            } else {
                light::SURFACE_2
            };
            button(label, move || on_select(i))
                .bg(bg)
                .hover_bg(hover)
                .text_color(fg)
                .radius(RADIUS_LG)
                .height(BTN_HEIGHT_SM)
                .padding(SPACE_3)
                .font_size(TEXT_SM)
                .into()
        })
        .collect();
    row(items)
        .gap(SPACE_1)
        .padding(SPACE_1)
        .bg(light::SURFACE_2)
        .radius(RADIUS_XL)
        .into()
}

// Underline-style tabs (like GitHub)
pub fn tab_bar_underline(
    tabs: Vec<(&str, bool)>,
    active_color: Color,
    on_select: impl Fn(usize) + 'static + Clone,
) -> View {
    let items: Vec<View> = tabs
        .into_iter()
        .enumerate()
        .map(|(i, (label, active))| {
            let on_select = on_select.clone();
            let underline = if active {
                active_color
            } else {
                Color::TRANSPARENT
            };
            let fg = if active {
                light::TEXT
            } else {
                light::TEXT_MUTED
            };
            column(vec![
                button(label, move || on_select(i))
                    .bg(Color::TRANSPARENT)
                    .hover_bg(light::SURFACE_2)
                    .text_color(fg)
                    .height(BTN_HEIGHT_SM)
                    .padding(SPACE_3)
                    .font_size(TEXT_SM)
                    .into(),
                rect(underline).height(2.0).into(),
            ])
            .gap(0.0)
            .into()
        })
        .collect();
    column(vec![row(items).gap(0.0).into(), divider()])
        .gap(0.0)
        .into()
}

// Sidebar nav item
pub fn nav_item(label: impl Into<String>, active: bool, on_click: impl Fn() + 'static) -> View {
    let bg = if active {
        with_opacity(light::ACCENT, 0.10)
    } else {
        Color::TRANSPARENT
    };
    let fg = if active { light::ACCENT } else { light::TEXT };
    let hover = if active {
        with_opacity(light::ACCENT, 0.12)
    } else {
        light::SURFACE_2
    };
    button(label, on_click)
        .bg(bg)
        .hover_bg(hover)
        .text_color(fg)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_3)
        .font_size(TEXT_SM)
        .into()
}

pub fn nav_item_dark(
    label: impl Into<String>,
    active: bool,
    on_click: impl Fn() + 'static,
) -> View {
    let bg = if active {
        with_opacity(dark::ACCENT, 0.15)
    } else {
        Color::TRANSPARENT
    };
    let fg = if active { dark::ACCENT } else { dark::TEXT };
    let hover = if active {
        with_opacity(dark::ACCENT, 0.18)
    } else {
        dark::SURFACE_2
    };
    button(label, on_click)
        .bg(bg)
        .hover_bg(hover)
        .text_color(fg)
        .radius(RADIUS_LG)
        .height(BTN_HEIGHT_MD)
        .padding(SPACE_3)
        .font_size(TEXT_SM)
        .into()
}

// Breadcrumb
pub fn breadcrumb(segments: Vec<impl Into<String>>) -> View {
    let n = segments.len();
    let items: Vec<View> = segments
        .into_iter()
        .enumerate()
        .flat_map(|(i, seg)| {
            let s: String = seg.into();
            let is_last = i == n - 1;
            let mut out = vec![text(s, TEXT_SM)
                .color(if is_last { light::TEXT } else { light::ACCENT })
                .into()];
            if !is_last {
                out.push(text("/", TEXT_SM).color(light::TEXT_MUTED).into());
            }
            out
        })
        .collect();
    row(items).gap(SPACE_2).into()
}

// Table
pub fn table_header(cols: Vec<impl Into<String>>) -> View {
    let cells: Vec<View> = cols
        .into_iter()
        .map(|c| {
            column(vec![text(c, TEXT_XS)
                .weight(FontWeight::Bold)
                .color(light::TEXT_MUTED)
                .into()])
            .padding_x(SPACE_4)
            .padding_y(SPACE_3)
            .grow()
            .into()
        })
        .collect();
    column(vec![row(cells).fill_width().into(), divider()])
        .gap(0.0)
        .bg(light::SURFACE_2)
        .into()
}

pub fn table_row(cells: Vec<View>) -> View {
    let wrapped: Vec<View> = cells
        .into_iter()
        .map(|c| {
            column(vec![c])
                .padding_x(SPACE_4)
                .padding_y(SPACE_3)
                .grow()
                .into()
        })
        .collect();
    column(vec![row(wrapped).fill_width().into(), divider()])
        .gap(0.0)
        .into()
}

pub fn table_row_hoverable(cells: Vec<View>) -> View {
    let wrapped: Vec<View> = cells
        .into_iter()
        .map(|c| {
            column(vec![c])
                .padding_x(SPACE_4)
                .padding_y(SPACE_3)
                .grow()
                .into()
        })
        .collect();
    column(vec![row(wrapped).fill_width().into(), divider()])
        .gap(0.0)
        .into()
}

// Form field — label + input + optional hint/error
pub fn form_field(
    label_text: impl Into<String>,
    input: View,
    hint: Option<impl Into<String>>,
    error: Option<impl Into<String>>,
) -> View {
    let mut items: Vec<View> = vec![
        text(label_text, TEXT_SM)
            .weight(FontWeight::Bold)
            .color(light::TEXT)
            .into(),
        input,
    ];
    if let Some(e) = error {
        items.push(text(e, TEXT_XS).color(light::DANGER).into());
    } else if let Some(h) = hint {
        items.push(text(h, TEXT_XS).color(light::TEXT_MUTED).into());
    }
    column(items).gap(SPACE_1).into()
}

pub fn form_field_dark(
    label_text: impl Into<String>,
    input: View,
    hint: Option<impl Into<String>>,
    error: Option<impl Into<String>>,
) -> View {
    let mut items: Vec<View> = vec![
        text(label_text, TEXT_SM)
            .weight(FontWeight::Bold)
            .color(dark::TEXT)
            .into(),
        input,
    ];
    if let Some(e) = error {
        items.push(text(e, TEXT_XS).color(dark::DANGER).into());
    } else if let Some(h) = hint {
        items.push(text(h, TEXT_XS).color(dark::TEXT_MUTED).into());
    }
    column(items).gap(SPACE_1).into()
}

// Checkbox-style toggle row (visual only — state managed by caller)
pub fn toggle_row(label: impl Into<String>, enabled: bool, _on_click: impl Fn() + 'static) -> View {
    let bg = if enabled {
        light::ACCENT
    } else {
        light::SURFACE_3
    };
    row(vec![
        column(vec![rect(bg).width(36.0).height(20.0).radius(10.0).into()])
            .width(36.0)
            .height(20.0)
            .into(),
        text(label, TEXT_SM).color(light::TEXT).into(),
    ])
    .gap(SPACE_3)
    .into()
}

// Inline icon-text pair used in list items
pub fn meta_item(icon_path: impl Into<String>, value: impl Into<String>) -> View {
    row(vec![
        image(icon_path).size(ICON_SM, ICON_SM).into(),
        text(value, TEXT_SM).color(light::TEXT_MUTED).into(),
    ])
    .gap(SPACE_2)
    .into()
}
