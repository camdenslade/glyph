use crate::signal::Signal;
use crate::theme::Theme;
use crate::view::{Color, FontWeight, Shadow, TextAlign, View};
use std::sync::Arc;
use taffy::{Layout, NodeId, Size, TaffyTree};

#[derive(Clone)]
enum MeasureContext {
    Text { content: String, font_size: f32 },
    Button { label: String, font_size: f32 },
}

/// A single positioned UI element after layout. `layout.location` is in
/// absolute pixel coordinates relative to the window top-left.
#[derive(Clone)]
pub struct FlatView {
    pub kind: FlatViewKind,
    pub layout: Layout,
}

/// The leaf variants of a laid-out view. Container nodes (Column/Row) are
/// consumed during layout and do not appear in the flat list.
///
/// `ClipStart` and `ClipEnd` are sentinels that bracket the children of a
/// `Scroll` node. The renderer sets a scissor rect on `ClipStart` and clears
/// it on `ClipEnd`.
#[derive(Clone)]
pub enum FlatViewKind {
    Rect {
        color: Color,
        corner_radius: f32,
    },
    Text {
        content: String,
        font_size: f32,
        color: Color,
        weight: FontWeight,
        align: TextAlign,
        wrap: bool,
    },
    Button {
        label: String,
        on_click: Arc<dyn Fn()>,
        on_hover: Option<Arc<dyn Fn(bool)>>,
        on_press: Option<Arc<dyn Fn(bool)>>,
        bg_color: Color,
        hover_bg_color: Option<Color>,
        press_bg_color: Option<Color>,
        text_color: Color,
        corner_radius: f32,
        font_size: f32,
        wrap: bool,
    },
    TextInput {
        value: Signal<String>,
        focused: Signal<bool>,
        cursor: Signal<usize>,
        selection: Option<(usize, usize)>,
        composing: Option<(usize, String)>,
        placeholder: String,
        font_size: f32,
        bg_color: Color,
        text_color: Color,
        border_color: Color,
        corner_radius: f32,
        on_change: Option<Arc<dyn Fn(String)>>,
        on_submit: Option<Arc<dyn Fn(String)>>,
    },
    /// Begin a scissor clip region covering the given viewport rect.
    ClipStart {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    /// Scroll region metadata emitted just before a ClipStart for Scroll/VirtualList nodes.
    /// Carries the offset signals and content bounds so the platform can apply momentum
    /// without rebuilding the view tree.
    ScrollRegion {
        offset_x: Signal<f32>,
        offset_y: Signal<f32>,
        max_x: f32,
        max_y: f32,
    },
    /// End the most recent scissor clip region.
    ClipEnd,
    /// Begin multiplying all descendant colors by this alpha.
    OpacityStart {
        alpha: f32,
    },
    /// End the most recent opacity scope.
    OpacityEnd,
    Image {
        path: String,
        corner_radius: f32,
    },
    TextArea {
        value: Signal<String>,
        focused: Signal<bool>,
        cursor: Signal<usize>,
        scroll_y: Signal<f32>,
        placeholder: String,
        font_size: f32,
        bg_color: Color,
        text_color: Color,
        border_color: Color,
        corner_radius: f32,
        on_change: Option<Arc<dyn Fn(String)>>,
    },
    /// A filled rectangle drawn before a container's children — used for
    /// container backgrounds, borders, and shadows.
    ContainerRect {
        bg_color: Option<Color>,
        border_color: Option<Color>,
        border_width: f32,
        corner_radius: f32,
        shadow: Option<Shadow>,
    },
}

/// Stateless entry point for layout. Call `ViewTree::build` each frame.
pub struct ViewTree;

impl ViewTree {
    /// Expand all `Component` nodes (passing `theme` to each), run Taffy flexbox
    /// layout, and return a flat list of positioned leaves.
    pub fn build(
        root: View,
        theme: &Theme,
        width: f32,
        height: f32,
        measure: &mut dyn FnMut(&str, f32, f32) -> (f32, f32),
    ) -> Vec<FlatView> {
        let root = expand(root, theme);
        let mut taffy: TaffyTree<Option<MeasureContext>> = TaffyTree::new();
        let root_node = build_node(&mut taffy, &root, measure);
        taffy
            .compute_layout_with_measure(
                root_node,
                Size {
                    width: taffy::AvailableSpace::Definite(width),
                    height: taffy::AvailableSpace::Definite(height),
                },
                |known_size, available, _node, ctx, _style| {
                    measure_node(ctx.as_deref(), known_size, available, measure)
                },
            )
            .expect("layout failed");
        let mut flat = Vec::new();
        collect(&mut taffy, root_node, root, &mut flat, 0.0, 0.0, measure);
        flat
    }
}

fn measure_node(
    ctx: Option<&Option<MeasureContext>>,
    known_size: Size<Option<f32>>,
    available: Size<taffy::AvailableSpace>,
    measure: &mut dyn FnMut(&str, f32, f32) -> (f32, f32),
) -> Size<f32> {
    match ctx.and_then(|ctx| ctx.as_ref()) {
        Some(MeasureContext::Text { content, font_size }) => {
            let max_w = known_size.width.unwrap_or(match available.width {
                taffy::AvailableSpace::Definite(w) => w,
                _ => 4096.0,
            });
            let (w, h) = measure(content, *font_size, max_w);
            Size {
                width: known_size.width.unwrap_or(w),
                height: known_size.height.unwrap_or(h),
            }
        }
        Some(MeasureContext::Button { label, font_size }) => {
            let max_w = known_size.width.unwrap_or(match available.width {
                taffy::AvailableSpace::Definite(w) => w,
                _ => 4096.0,
            });
            let (measured_w, measured_h) = measure(label, *font_size, max_w);
            Size {
                width: known_size.width.unwrap_or(measured_w),
                height: known_size.height.unwrap_or(measured_h),
            }
        }
        None => Size {
            width: known_size.width.unwrap_or(0.0),
            height: known_size.height.unwrap_or(0.0),
        },
    }
}

/// Recursively replace every `View::Component` with its rendered output.
fn expand(view: View, theme: &Theme) -> View {
    match view {
        View::Component(c) => expand(c.render(theme), theme),
        View::Column {
            children,
            style,
            bg_color,
            border_color,
            border_width,
            corner_radius,
            shadow,
            clip,
        } => View::Column {
            children: children.into_iter().map(|c| expand(c, theme)).collect(),
            style,
            bg_color,
            border_color,
            border_width,
            corner_radius,
            shadow,
            clip,
        },
        View::Row {
            children,
            style,
            bg_color,
            border_color,
            border_width,
            corner_radius,
            shadow,
            clip,
        } => View::Row {
            children: children.into_iter().map(|c| expand(c, theme)).collect(),
            style,
            bg_color,
            border_color,
            border_width,
            corner_radius,
            shadow,
            clip,
        },
        View::ZStack {
            children,
            style,
            bg_color,
            border_color,
            border_width,
            corner_radius,
            shadow,
        } => View::ZStack {
            children: children.into_iter().map(|c| expand(c, theme)).collect(),
            style,
            bg_color,
            border_color,
            border_width,
            corner_radius,
            shadow,
        },
        View::Scroll {
            child,
            offset_x,
            offset_y,
            max_scroll,
            style,
        } => View::Scroll {
            child: Box::new(expand(*child, theme)),
            offset_x,
            offset_y,
            max_scroll,
            style,
        },
        View::Flexible {
            child,
            grow,
            shrink,
        } => View::Flexible {
            child: Box::new(expand(*child, theme)),
            grow,
            shrink,
        },
        View::Opacity { child, alpha } => View::Opacity {
            child: Box::new(expand(*child, theme)),
            alpha,
        },
        View::VirtualList { .. } => view,
        View::TextArea { .. } => view,
        other => other,
    }
}

fn get_style(view: &View) -> taffy::Style {
    match view {
        View::Column { style, .. }
        | View::Row { style, .. }
        | View::ZStack { style, .. }
        | View::Scroll { style, .. }
        | View::Rect { style, .. }
        | View::Text { style, .. }
        | View::Button { style, .. }
        | View::TextInput { style, .. }
        | View::Image { style, .. }
        | View::VirtualList { style, .. }
        | View::TextArea { style, .. } => style.clone(),
        View::Flexible {
            child,
            grow,
            shrink,
        } => {
            let mut s = get_style(child);
            s.flex_grow = *grow;
            s.flex_shrink = *shrink;
            s
        }
        View::Spacer => taffy::Style {
            flex_grow: 1.0,
            flex_shrink: 1.0,
            ..Default::default()
        },
        View::Opacity { child, .. } => get_style(child),
        View::Component(_) => taffy::Style::default(),
    }
}

fn build_node(
    taffy: &mut TaffyTree<Option<MeasureContext>>,
    view: &View,
    measure: &mut dyn FnMut(&str, f32, f32) -> (f32, f32),
) -> NodeId {
    match view {
        View::Column {
            children, style, ..
        }
        | View::Row {
            children, style, ..
        } => {
            let child_nodes: Vec<NodeId> = children
                .iter()
                .map(|c| build_node(taffy, c, measure))
                .collect();
            taffy
                .new_with_children(style.clone(), &child_nodes)
                .expect("taffy node")
        }
        View::ZStack {
            children, style, ..
        } => {
            let child_nodes: Vec<NodeId> = children
                .iter()
                .map(|c| {
                    let mut child_style = get_style(c);
                    child_style.position = taffy::Position::Absolute;
                    let node = build_node(taffy, c, measure);
                    taffy.set_style(node, child_style).ok();
                    node
                })
                .collect();
            taffy
                .new_with_children(style.clone(), &child_nodes)
                .expect("taffy node")
        }
        View::Scroll { child, style, .. } => {
            let child_node = build_node(taffy, child, measure);
            taffy
                .new_with_children(style.clone(), &[child_node])
                .expect("taffy node")
        }
        View::Rect { style, .. } => taffy
            .new_leaf_with_context(style.clone(), None)
            .expect("taffy node"),
        View::TextInput { style, .. } => taffy
            .new_leaf_with_context(style.clone(), None)
            .expect("taffy node"),
        View::Text {
            content,
            font_size,
            wrap,
            style,
            ..
        } => {
            if *wrap {
                // Wrapping text: store content+size as node context so taffy's measure
                // callback can compute the correct height at the actual available width.
                let style = taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::Auto,
                        height: taffy::Dimension::Auto,
                    },
                    ..style.clone()
                };
                taffy
                    .new_leaf_with_context(
                        style,
                        Some(MeasureContext::Text {
                            content: content.clone(),
                            font_size: *font_size,
                        }),
                    )
                    .expect("taffy node")
            } else {
                let (mw, mh) = measure(content, *font_size, 4096.0);
                let w = match style.size.width {
                    taffy::Dimension::Length(v) => taffy::Dimension::Length(v),
                    _ => taffy::Dimension::Length(mw),
                };
                let h = match style.size.height {
                    taffy::Dimension::Length(v) => taffy::Dimension::Length(v),
                    _ => taffy::Dimension::Length(mh),
                };
                let style = taffy::Style {
                    size: taffy::Size {
                        width: w,
                        height: h,
                    },
                    ..style.clone()
                };
                taffy
                    .new_leaf_with_context(style, None)
                    .expect("taffy node")
            }
        }
        View::Button {
            label,
            font_size,
            wrap,
            style,
            ..
        } => {
            let lp_val = |lp: taffy::LengthPercentage| match lp {
                taffy::LengthPercentage::Length(v) => v,
                taffy::LengthPercentage::Percent(_) => 0.0,
            };
            let pad_x = lp_val(style.padding.left) + lp_val(style.padding.right);
            let pad_y = lp_val(style.padding.top) + lp_val(style.padding.bottom);
            if *wrap {
                taffy
                    .new_leaf_with_context(
                        style.clone(),
                        Some(MeasureContext::Button {
                            label: label.clone(),
                            font_size: *font_size,
                        }),
                    )
                    .expect("taffy node")
            } else {
                let (tw, th) = measure(label, *font_size, 4096.0);
                let w = match style.size.width {
                    taffy::Dimension::Length(v) => taffy::Dimension::Length(v),
                    _ => taffy::Dimension::Length(tw + pad_x),
                };
                let h = match style.size.height {
                    taffy::Dimension::Length(v) => taffy::Dimension::Length(v),
                    _ => taffy::Dimension::Length(th + pad_y),
                };
                let style = taffy::Style {
                    size: taffy::Size {
                        width: w,
                        height: h,
                    },
                    ..style.clone()
                };
                taffy.new_leaf(style).expect("taffy node")
            }
        }
        View::Image { style, .. } => taffy
            .new_leaf_with_context(style.clone(), None)
            .expect("taffy node"),
        View::Spacer => taffy
            .new_leaf_with_context(
                taffy::Style {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    ..Default::default()
                },
                None,
            )
            .expect("taffy node"),
        View::Flexible {
            child,
            grow,
            shrink,
        } => {
            let node = build_node(taffy, child, measure);
            let mut style = get_style(child);
            style.flex_grow = *grow;
            style.flex_shrink = *shrink;
            taffy.set_style(node, style).ok();
            node
        }
        View::Opacity { child, .. } => build_node(taffy, child, measure),
        View::TextArea { style, .. } => taffy
            .new_leaf_with_context(style.clone(), None)
            .expect("taffy node"),
        View::VirtualList {
            item_count,
            row_height,
            offset_y,
            build_row,
            viewport_height,
            style,
        } => {
            let inner = build_virtual_inner(
                taffy,
                measure,
                *item_count,
                *row_height,
                offset_y,
                build_row,
                *viewport_height,
            );
            let mut node_style = style.clone();
            node_style.size.height = taffy::Dimension::Length(*viewport_height);
            taffy
                .new_with_children(node_style, &[inner])
                .expect("taffy node")
        }
        View::Component(_) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{button, Theme};

    fn fake_measure(text: &str, _font_size: f32, max_width: f32) -> (f32, f32) {
        let char_w = 10.0_f32;
        let line_h = 12.0_f32;
        let full_w = text.chars().count() as f32 * char_w;
        let chars_per_line = (max_width / char_w).floor().max(1.0) as usize;
        let lines = text.chars().count().max(1).div_ceil(chars_per_line);
        (full_w.min(max_width), lines as f32 * line_h)
    }

    fn button_layout(flat: &[FlatView]) -> Layout {
        flat.iter()
            .find_map(|fv| match fv.kind {
                FlatViewKind::Button { .. } => Some(fv.layout),
                _ => None,
            })
            .expect("button layout")
    }

    #[test]
    fn wrapped_button_uses_constrained_text_height_plus_padding() {
        let view = button("abcdefghij", || {})
            .width(60.0)
            .padding(5.0)
            .wrap()
            .into_view();
        let mut measure = fake_measure;

        let flat = ViewTree::build(view, &Theme::light(), 200.0, 200.0, &mut measure);
        let layout = button_layout(&flat);

        assert_eq!(layout.size.width, 60.0);
        assert_eq!(layout.size.height, 34.0);
    }

    #[test]
    fn button_wrap_is_opt_in() {
        let view = button("abcdefghij", || {})
            .width(60.0)
            .padding(5.0)
            .into_view();
        let mut measure = fake_measure;

        let flat = ViewTree::build(view, &Theme::light(), 200.0, 200.0, &mut measure);
        let layout = button_layout(&flat);

        assert_eq!(layout.size.width, 60.0);
        assert_eq!(layout.size.height, 22.0);
    }
}

fn build_virtual_inner(
    taffy: &mut TaffyTree<Option<MeasureContext>>,
    measure: &mut dyn FnMut(&str, f32, f32) -> (f32, f32),
    item_count: usize,
    row_height: f32,
    offset_y: &Signal<f32>,
    build_row: &dyn Fn(usize) -> View,
    viewport_height: f32,
) -> NodeId {
    let oy = offset_y.get();
    let first_row = (oy / row_height).floor() as usize;
    let visible_count = (viewport_height / row_height).ceil() as usize + 1;
    let last_row = (first_row + visible_count).min(item_count);
    let col_style = taffy::Style {
        flex_direction: taffy::FlexDirection::Column,
        align_items: Some(taffy::AlignItems::Stretch),
        size: taffy::Size {
            width: taffy::Dimension::Percent(1.0),
            height: taffy::Dimension::Auto,
        },
        ..Default::default()
    };
    let child_nodes: Vec<NodeId> = (first_row..last_row)
        .map(|i| {
            let row_view = build_row(i);
            build_node(taffy, &row_view, measure)
        })
        .collect();
    taffy
        .new_with_children(col_style, &child_nodes)
        .expect("taffy node")
}

fn collect(
    taffy: &mut TaffyTree<Option<MeasureContext>>,
    node: NodeId,
    view: View,
    flat: &mut Vec<FlatView>,
    parent_x: f32,
    parent_y: f32,
    measure: &mut dyn FnMut(&str, f32, f32) -> (f32, f32),
) {
    let layout = *taffy.layout(node).expect("layout");
    let x = parent_x + layout.location.x;
    let y = parent_y + layout.location.y;
    let adjusted = Layout {
        location: taffy::Point { x, y },
        ..layout
    };

    match view {
        View::Rect {
            color,
            corner_radius,
            ..
        } => {
            flat.push(FlatView {
                kind: FlatViewKind::Rect {
                    color,
                    corner_radius,
                },
                layout: adjusted,
            });
        }
        View::Text {
            content,
            font_size,
            color,
            weight,
            align,
            wrap,
            ..
        } => {
            flat.push(FlatView {
                kind: FlatViewKind::Text {
                    content,
                    font_size,
                    color,
                    weight,
                    align,
                    wrap,
                },
                layout: adjusted,
            });
        }
        View::Button {
            label,
            on_click,
            on_hover,
            on_press,
            bg_color,
            hover_bg_color,
            press_bg_color,
            text_color,
            corner_radius,
            font_size,
            wrap,
            ..
        } => {
            flat.push(FlatView {
                kind: FlatViewKind::Button {
                    label,
                    on_click,
                    on_hover,
                    on_press,
                    bg_color,
                    hover_bg_color,
                    press_bg_color,
                    text_color,
                    corner_radius,
                    font_size,
                    wrap,
                },
                layout: adjusted,
            });
        }
        View::TextInput {
            value,
            focused,
            cursor,
            placeholder,
            font_size,
            bg_color,
            text_color,
            border_color,
            corner_radius,
            on_change,
            on_submit,
            ..
        } => {
            flat.push(FlatView {
                kind: FlatViewKind::TextInput {
                    value,
                    focused,
                    cursor,
                    selection: None,
                    composing: None,
                    placeholder,
                    font_size,
                    bg_color,
                    text_color,
                    border_color,
                    corner_radius,
                    on_change,
                    on_submit,
                },
                layout: adjusted,
            });
        }
        View::Spacer => {}
        View::Column {
            children,
            bg_color,
            border_color,
            border_width,
            corner_radius,
            shadow,
            clip,
            ..
        }
        | View::Row {
            children,
            bg_color,
            border_color,
            border_width,
            corner_radius,
            shadow,
            clip,
            ..
        } => {
            if bg_color.is_some() || border_color.is_some() || shadow.is_some() {
                flat.push(FlatView {
                    kind: FlatViewKind::ContainerRect {
                        bg_color,
                        border_color,
                        border_width,
                        corner_radius,
                        shadow,
                    },
                    layout: adjusted,
                });
            }
            if clip {
                flat.push(FlatView {
                    kind: FlatViewKind::ClipStart {
                        x,
                        y,
                        width: adjusted.size.width,
                        height: adjusted.size.height,
                    },
                    layout: adjusted,
                });
            }
            let child_nodes = taffy.children(node).expect("children");
            for (child_node, child_view) in child_nodes.iter().zip(children.into_iter()) {
                collect(taffy, *child_node, child_view, flat, x, y, measure);
            }
            if clip {
                flat.push(FlatView {
                    kind: FlatViewKind::ClipEnd,
                    layout: adjusted,
                });
            }
        }
        View::ZStack {
            children,
            bg_color,
            border_color,
            border_width,
            corner_radius,
            shadow,
            ..
        } => {
            if bg_color.is_some() || border_color.is_some() || shadow.is_some() {
                flat.push(FlatView {
                    kind: FlatViewKind::ContainerRect {
                        bg_color,
                        border_color,
                        border_width,
                        corner_radius,
                        shadow,
                    },
                    layout: adjusted,
                });
            }
            let child_nodes = taffy.children(node).expect("children");
            for (child_node, child_view) in child_nodes.iter().zip(children.into_iter()) {
                collect(taffy, *child_node, child_view, flat, x, y, measure);
            }
        }
        View::Scroll {
            child,
            offset_x,
            offset_y,
            max_scroll,
            ..
        } => {
            let vw = adjusted.size.width;
            let vh = adjusted.size.height;
            let child_nodes = taffy.children(node).expect("children");
            // TODO: scroll is still a little laggy. The root cause is that scroll offset
            // is baked into flat list positions in collect(), so every scroll pixel requires
            // a full build+layout pass. The fix is to move scroll translation into the
            // renderer (pass offset to render, keep flat positions viewport-relative) so
            // scroll frames skip layout entirely. The max_scroll cache below is a partial
            // mitigation (avoids the second taffy pass after the first frame) but the main
            // layout pass still runs every frame.
            let (max_x, max_y) = {
                let cached = max_scroll.get();
                if cached.0 > 0.0 || cached.1 > 0.0 {
                    cached
                } else {
                    taffy
                        .compute_layout_with_measure(
                            child_nodes[0],
                            Size {
                                width: taffy::AvailableSpace::Definite(vw),
                                height: taffy::AvailableSpace::MaxContent,
                            },
                            |known_size, available, _node, ctx, _style| {
                                measure_node(ctx.as_deref(), known_size, available, measure)
                            },
                        )
                        .ok();
                    let nl = taffy.layout(child_nodes[0]).expect("child layout");
                    let mx = (nl.size.width - vw).max(0.0);
                    let my = (nl.size.height - vh).max(0.0);
                    max_scroll.set((mx, my));
                    (mx, my)
                }
            };
            flat.push(FlatView {
                kind: FlatViewKind::ScrollRegion {
                    offset_x: offset_x.clone(),
                    offset_y: offset_y.clone(),
                    max_x,
                    max_y,
                },
                layout: adjusted,
            });
            flat.push(FlatView {
                kind: FlatViewKind::ClipStart {
                    x,
                    y,
                    width: vw,
                    height: vh,
                },
                layout: adjusted,
            });
            // Pass x,y without offset — renderer applies scroll offset at draw time
            collect(taffy, child_nodes[0], *child, flat, x, y, measure);
            flat.push(FlatView {
                kind: FlatViewKind::ClipEnd,
                layout: adjusted,
            });
        }
        View::Image {
            path,
            corner_radius,
            ..
        } => {
            flat.push(FlatView {
                kind: FlatViewKind::Image {
                    path,
                    corner_radius,
                },
                layout: adjusted,
            });
        }
        View::Flexible { child, .. } => {
            collect(taffy, node, *child, flat, parent_x, parent_y, measure);
        }
        View::Opacity { child, alpha } => {
            flat.push(FlatView {
                kind: FlatViewKind::OpacityStart { alpha },
                layout: adjusted,
            });
            collect(taffy, node, *child, flat, parent_x, parent_y, measure);
            flat.push(FlatView {
                kind: FlatViewKind::OpacityEnd,
                layout: adjusted,
            });
        }
        View::TextArea {
            value,
            focused,
            cursor,
            scroll_y,
            placeholder,
            font_size,
            bg_color,
            text_color,
            border_color,
            corner_radius,
            on_change,
            ..
        } => {
            flat.push(FlatView {
                kind: FlatViewKind::TextArea {
                    value,
                    focused,
                    cursor,
                    scroll_y,
                    placeholder,
                    font_size,
                    bg_color,
                    text_color,
                    border_color,
                    corner_radius,
                    on_change,
                },
                layout: adjusted,
            });
        }
        View::VirtualList {
            item_count,
            row_height,
            offset_y,
            build_row,
            viewport_height,
            ..
        } => {
            let vw = adjusted.size.width;
            let vh = adjusted.size.height;
            let max_y = (item_count as f32 * row_height - viewport_height).max(0.0);
            flat.push(FlatView {
                kind: FlatViewKind::ScrollRegion {
                    offset_x: Signal::new(0.0),
                    offset_y: offset_y.clone(),
                    max_x: 0.0,
                    max_y,
                },
                layout: adjusted,
            });
            flat.push(FlatView {
                kind: FlatViewKind::ClipStart {
                    x,
                    y,
                    width: vw,
                    height: vh,
                },
                layout: adjusted,
            });
            let oy = offset_y.get();
            let frac_offset = oy % row_height;
            let first_row = (oy / row_height).floor() as usize;
            let visible_count = (viewport_height / row_height).ceil() as usize + 1;
            let last_row = (first_row + visible_count).min(item_count);
            let child_nodes = taffy.children(node).expect("children");
            let col_node = child_nodes[0];
            let col_children = taffy.children(col_node).expect("col children");
            for (ci, &child_node) in col_children.iter().enumerate() {
                let row_idx = first_row + ci;
                if row_idx >= last_row {
                    break;
                }
                let row_view = build_row(row_idx);
                collect(
                    taffy,
                    child_node,
                    row_view,
                    flat,
                    x,
                    y - frac_offset,
                    measure,
                );
            }
            flat.push(FlatView {
                kind: FlatViewKind::ClipEnd,
                layout: adjusted,
            });
        }
        View::Component(_) => unreachable!(),
    }
}
