//! Data display: Table (styled), Timeline, Feed, MediaObject, DescriptionList,
//! Heatmap, Sparkline (primitive bar-based), DataList, Metric/KPI card variants.

use crate::colors::{dark, with_opacity};
use crate::spacing::*;
use core_glyph::{
    button_view, column, rect, row, spacer, text, Color,
    FontWeight, Theme, View,
};


pub struct TableColumn {
    pub header: String,
    pub width: Option<f32>, // None = flex-grow
    pub align_right: bool,
}

/// Renders a themed table with selectable rows.
pub fn data_table(
    theme: &Theme,
    columns: &[TableColumn],
    rows: Vec<Vec<View>>,
    on_row_click: Option<impl Fn(usize) + 'static + Clone>,
) -> View {
    let mut children: Vec<View> = vec![table_head(theme, columns)];
    for (i, row_cells) in rows.into_iter().enumerate() {
        let row_view = table_data_row(theme, columns, row_cells, i % 2 == 1);
        if let Some(ref cb) = on_row_click {
            let cb = cb.clone();
            children.push(
                button_view(row_view, move || cb(i))
                    .bg(Color::TRANSPARENT).hover_bg(with_opacity(theme.primary, 0.06))
                    .fill_width().into()
            );
        } else {
            children.push(row_view);
        }
    }
    column(children)
        .gap(0.0).fill_width()
        .bg(theme.surface).border(theme.border, 1.0).radius(RADIUS_XL).into()
}

fn table_head(theme: &Theme, columns: &[TableColumn]) -> View {
    let cells: Vec<View> = columns.iter().map(|col| {
        let cell = column(vec![
            text(col.header.clone(), TEXT_XS)
                .weight(FontWeight::Bold).color(theme.text_muted).into()
        ])
        .padding_x(SPACE_4).padding_y(SPACE_3);
        let cell = if let Some(w) = col.width { cell.width(w) } else { cell.grow() };
        cell.into()
    }).collect();
    column(vec![
        row(cells).fill_width().into(),
        rect(theme.border).height(1.0).fill_width().into(),
    ])
    .gap(0.0).bg(dark::SURFACE_2).into()
}

fn table_data_row(theme: &Theme, columns: &[TableColumn], cells: Vec<View>, striped: bool) -> View {
    let bg = if striped { dark::BG_SUBTLE } else { Color::TRANSPARENT };
    let wrapped: Vec<View> = cells.into_iter().zip(columns.iter()).map(|(cell, col)| {
        let c = column(vec![cell])
            .padding_x(SPACE_4).padding_y(SPACE_3);
        let c = if let Some(w) = col.width { c.width(w) } else { c.grow() };
        c.into()
    }).collect();
    column(vec![
        row(wrapped).fill_width().bg(bg).into(),
        rect(theme.border).height(1.0).fill_width().into(),
    ]).gap(0.0).into()
}


pub struct TimelineItem {
    pub icon: Option<View>,
    pub title: String,
    pub body: Option<String>,
    pub timestamp: String,
    pub accent: Option<Color>,
}

pub fn timeline(theme: &Theme, items: Vec<TimelineItem>) -> View {
    let rows: Vec<View> = items.into_iter().enumerate().map(|(i, item)| {
        timeline_item(theme, item, i == 0)
    }).collect();
    column(rows).gap(0.0).fill_width().into()
}

fn timeline_item(theme: &Theme, item: TimelineItem, _first: bool) -> View {
    let accent = item.accent.unwrap_or(theme.primary);
    let dot: View = if let Some(icon) = item.icon {
        column(vec![icon])
            .width(28.0).height(28.0).align_center().justify_center()
            .bg(with_opacity(accent, 0.15)).radius(14.0).into()
    } else {
        column(vec![])
            .width(10.0).height(10.0)
            .bg(accent).radius(5.0).into()
    };

    let mut body_parts: Vec<View> = vec![
        row(vec![
            text(item.title, TEXT_SM).weight(FontWeight::Bold).color(theme.text).into(),
            spacer(),
            text(item.timestamp, TEXT_XS).color(theme.text_muted).into(),
        ]).fill_width().into(),
    ];
    if let Some(b) = item.body {
        body_parts.push(text(b, TEXT_SM).color(theme.text_muted).wrap().into());
    }

    row(vec![
        // Left: dot + line
        column(vec![
            dot,
            column(vec![]).width(2.0).grow().bg(with_opacity(theme.border, 0.5)).into(),
        ])
        .gap(0.0).width(28.0).align_center().into(),
        // Right: content
        column(body_parts).gap(SPACE_1).grow().padding_bottom(SPACE_4).into(),
    ])
    .gap(SPACE_3).fill_width().into()
}


pub fn media_object(
    theme: &Theme,
    media: View,
    title: impl Into<String>,
    subtitle: impl Into<String>,
    trailing: Option<View>,
) -> View {
    let mut items: Vec<View> = vec![
        media,
        column(vec![
            text(title, TEXT_BASE).weight(FontWeight::Bold).color(theme.text).into(),
            text(subtitle, TEXT_SM).color(theme.text_muted).into(),
        ]).gap(SPACE_0_5).grow().into(),
    ];
    if let Some(t) = trailing { items.push(t); }
    row(items).gap(SPACE_3).fill_width().align_center().into()
}

pub fn media_object_sm(
    theme: &Theme,
    media: View,
    title: impl Into<String>,
    trailing: Option<View>,
) -> View {
    let mut items = vec![
        media,
        column(vec![text(title, TEXT_SM).color(theme.text).into()]).grow().into(),
    ];
    if let Some(t) = trailing { items.push(t); }
    row(items).gap(SPACE_2).fill_width().align_center().into()
}


pub fn description_list(theme: &Theme, items: Vec<(impl Into<String>, impl Into<String>)>) -> View {
    let rows: Vec<View> = items.into_iter().map(|(key, val)| {
        row(vec![
            text(key, TEXT_SM).weight(FontWeight::Bold).color(theme.text_muted).width(140.0).into(),
            column(vec![text(val, TEXT_SM).color(theme.text).into()]).grow().into(),
        ])
        .gap(SPACE_3).fill_width().padding_y(SPACE_2).align_center().into()
    }).collect();
    column(rows).gap(0.0).fill_width()
        .border(theme.border, 1.0).radius(RADIUS_LG)
        .padding_x(SPACE_4).padding_y(SPACE_1).into()
}


/// `cells` is a flat grid: `cols` cells per row.
/// Each cell value 0.0–1.0 maps to a color intensity.
pub fn heatmap(accent: Color, cells: &[f32], cols: usize, cell_size: f32, gap: f32) -> View {
    let rows: Vec<View> = cells.chunks(cols).map(|row_vals| {
        let cells: Vec<View> = row_vals.iter().map(|&v| {
            let v = v.clamp(0.0, 1.0);
            let color = if v == 0.0 {
                dark::SURFACE_2
            } else {
                Color::rgba(accent.r, accent.g, accent.b, 0.2 + v * 0.8)
            };
            rect(color).width(cell_size).height(cell_size).radius(2.0).into()
        }).collect();
        row(cells).gap(gap).into()
    }).collect();
    column(rows).gap(gap).into()
}


/// A small bar chart inline. Values 0.0–1.0 relative to max.
pub fn sparkline(values: &[f32], color: Color, bar_w: f32, max_h: f32, gap: f32) -> View {
    let max_val = values.iter().cloned().fold(0.0_f32, f32::max).max(1.0);
    let bars: Vec<View> = values.iter().map(|&v| {
        let h = ((v / max_val) * max_h).max(2.0);
        column(vec![
            spacer(),
            rect(color).width(bar_w).height(h).radius(1.0).into(),
        ]).gap(0.0).height(max_h).into()
    }).collect();
    row(bars).gap(gap).height(max_h).align_center().into()
}


pub struct BarDatum {
    pub label: String,
    pub value: f32,
    pub color: Option<Color>,
}

pub fn bar_chart(
    theme: &Theme,
    data: &[BarDatum],
    width: f32,
    height: f32,
) -> View {
    let max_val = data.iter().map(|d| d.value).fold(0.0_f32, f32::max).max(1.0);
    let bar_w = ((width - data.len() as f32 * 4.0) / data.len() as f32).max(8.0);

    let bars: Vec<View> = data.iter().map(|d| {
        let h = (d.value / max_val) * (height - 24.0);
        let color = d.color.unwrap_or(theme.primary);
        let label_short = if d.label.len() > 6 { format!("{}…", &d.label[..5]) } else { d.label.clone() };
        column(vec![
            spacer(),
            rect(color).width(bar_w).height(h).radius(3.0).into(),
            text(label_short, TEXT_XS).color(theme.text_muted).into(),
        ])
        .gap(SPACE_1).height(height).align_center().into()
    }).collect();

    column(vec![
        row(bars).gap(4.0).height(height).fill_width().align_center().into(),
    ])
    .fill_width().into()
}


/// Simple text-based gauge indicator.
pub fn gauge(theme: &Theme, label: impl Into<String>, value: f32, max: f32, size: f32, color: Color) -> View {
    let pct = (value / max).clamp(0.0, 1.0);
    let pct_label = format!("{:.0}%", pct * 100.0);
    column(vec![
        // Arc approximation using a progress bar in a circle shape
        column(vec![
            text(pct_label, TEXT_2XL).weight(FontWeight::Bold).color(color).into(),
            text(label, TEXT_XS).color(theme.text_muted).into(),
        ])
        .width(size).height(size)
        .align_center().justify_center()
        .bg(with_opacity(color, 0.12))
        .border(color, 3.0)
        .radius(size / 2.0)
        .into(),
    ])
    .align_center().into()
}


pub fn data_list(theme: &Theme, items: Vec<(impl Into<String>, View)>) -> View {
    let rows: Vec<View> = items.into_iter().map(|(key, val_view)| {
        row(vec![
            text(key, TEXT_XS).weight(FontWeight::Bold).color(theme.text_muted).width(100.0).into(),
            val_view,
        ])
        .gap(SPACE_2).fill_width().padding_y(SPACE_1).align_center().into()
    }).collect();
    column(rows).gap(0.0).fill_width().into()
}


pub fn spinner(theme: &Theme, size: f32) -> View {
    // GPU renderer doesn't have animated rotation yet, so we render a static arc
    column(vec![
        rect(with_opacity(theme.primary, 0.3))
            .width(size).height(size).radius(size / 2.0).into(),
        rect(theme.primary)
            .width(size / 6.0).height(size / 6.0).radius(size / 12.0).into(),
    ])
    .width(size).height(size).align_center().justify_center().into()
}

pub fn spinner_row(theme: &Theme, label: impl Into<String>) -> View {
    row(vec![
        spinner(theme, 16.0),
        text(label, TEXT_SM).color(theme.text_muted).into(),
    ])
    .gap(SPACE_2).align_center().into()
}
