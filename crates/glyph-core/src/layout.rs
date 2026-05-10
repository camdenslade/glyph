use crate::signal::Signal;
use crate::theme::Theme;
use crate::view::{Color, FontWeight, Shadow, TextAlign, View};
use taffy::{Layout, NodeId, Size, TaffyTree};

/// A single positioned UI element after layout. `layout.location` is in
/// absolute pixel coordinates relative to the window top-left.
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
pub enum FlatViewKind {
    Rect {
        color: Color,
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
        on_click: Box<dyn Fn()>,
        on_hover: Option<Box<dyn Fn(bool)>>,
        on_press: Option<Box<dyn Fn(bool)>>,
        bg_color: Color,
        hover_bg_color: Option<Color>,
        press_bg_color: Option<Color>,
        text_color: Color,
        corner_radius: f32,
        font_size: f32,
    },
    TextInput {
        value: Signal<String>,
        focused: Signal<bool>,
        cursor: Signal<usize>,
        placeholder: String,
        font_size: f32,
        bg_color: Color,
        text_color: Color,
        border_color: Color,
        corner_radius: f32,
        on_change: Option<Box<dyn Fn(String)>>,
        on_submit: Option<Box<dyn Fn(String)>>,
    },
    /// Begin a scissor clip region covering the given viewport rect.
    ClipStart {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    /// End the most recent scissor clip region.
    ClipEnd,
    /// Begin multiplying all descendant colors by this alpha.
    OpacityStart { alpha: f32 },
    /// End the most recent opacity scope.
    OpacityEnd,
    Image {
        path: String,
        corner_radius: f32,
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
        // NodeContext = Some((content, font_size)) for wrapping text nodes, None otherwise.
        let mut taffy: TaffyTree<Option<(String, f32)>> = TaffyTree::new();
        let root_node = build_node(&mut taffy, &root, measure);
        taffy
            .compute_layout_with_measure(
                root_node,
                Size {
                    width: taffy::AvailableSpace::Definite(width),
                    height: taffy::AvailableSpace::Definite(height),
                },
                |known_size, available, _node, ctx, _style| {
                    if let Some(Some((content, font_size))) = ctx {
                        let max_w = known_size.width.unwrap_or_else(|| match available.width {
                            taffy::AvailableSpace::Definite(w) => w,
                            _ => 4096.0,
                        });
                        let (w, h) = measure(content, *font_size, max_w);
                        Size { width: known_size.width.unwrap_or(w), height: known_size.height.unwrap_or(h) }
                    } else {
                        Size { width: known_size.width.unwrap_or(0.0), height: known_size.height.unwrap_or(0.0) }
                    }
                },
            )
            .expect("layout failed");
        let mut flat = Vec::new();
        collect(&taffy, root_node, root, &mut flat, 0.0, 0.0);
        flat
    }
}

/// Recursively replace every `View::Component` with its rendered output.
fn expand(view: View, theme: &Theme) -> View {
    match view {
        View::Component(c) => expand(c.render(theme), theme),
        View::Column { children, style, bg_color, border_color, border_width, corner_radius, shadow, clip } => View::Column {
            children: children.into_iter().map(|c| expand(c, theme)).collect(),
            style, bg_color, border_color, border_width, corner_radius, shadow, clip,
        },
        View::Row { children, style, bg_color, border_color, border_width, corner_radius, shadow, clip } => View::Row {
            children: children.into_iter().map(|c| expand(c, theme)).collect(),
            style, bg_color, border_color, border_width, corner_radius, shadow, clip,
        },
        View::ZStack { children, style, bg_color, border_color, border_width, corner_radius, shadow } => View::ZStack {
            children: children.into_iter().map(|c| expand(c, theme)).collect(),
            style, bg_color, border_color, border_width, corner_radius, shadow,
        },
        View::Scroll { child, offset_x, offset_y, style } => View::Scroll {
            child: Box::new(expand(*child, theme)),
            offset_x,
            offset_y,
            style,
        },
        View::Flexible { child, grow, shrink } => View::Flexible {
            child: Box::new(expand(*child, theme)),
            grow,
            shrink,
        },
        View::Opacity { child, alpha } => View::Opacity {
            child: Box::new(expand(*child, theme)),
            alpha,
        },
        other => other,
    }
}

fn get_style(view: &View) -> taffy::Style {
    match view {
        View::Column { style, .. } | View::Row { style, .. } | View::ZStack { style, .. }
        | View::Scroll { style, .. } | View::Rect { style, .. } | View::Text { style, .. }
        | View::Button { style, .. } | View::TextInput { style, .. } | View::Image { style, .. } => style.clone(),
        View::Flexible { child, grow, shrink } => {
            let mut s = get_style(child);
            s.flex_grow = *grow;
            s.flex_shrink = *shrink;
            s
        }
        View::Spacer => taffy::Style { flex_grow: 1.0, flex_shrink: 1.0, ..Default::default() },
        View::Opacity { child, .. } => get_style(child),
        View::Component(_) => taffy::Style::default(),
    }
}

fn build_node(
    taffy: &mut TaffyTree<Option<(String, f32)>>,
    view: &View,
    measure: &mut dyn FnMut(&str, f32, f32) -> (f32, f32),
) -> NodeId {
    match view {
        View::Column { children, style, .. } | View::Row { children, style, .. } => {
            let child_nodes: Vec<NodeId> =
                children.iter().map(|c| build_node(taffy, c, measure)).collect();
            taffy.new_with_children(style.clone(), &child_nodes).expect("taffy node")
        }
        View::ZStack { children, style, .. } => {
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
            taffy.new_with_children(style.clone(), &child_nodes).expect("taffy node")
        }
        View::Scroll { child, style, .. } => {
            let child_node = build_node(taffy, child, measure);
            taffy.new_with_children(style.clone(), &[child_node]).expect("taffy node")
        }
        View::Rect { style, .. } => taffy.new_leaf_with_context(style.clone(), None).expect("taffy node"),
        View::TextInput { style, .. } => taffy.new_leaf_with_context(style.clone(), None).expect("taffy node"),
        View::Text { content, font_size, wrap, style, .. } => {
            if *wrap {
                // Wrapping text: store content+size as node context so taffy's measure
                // callback can compute the correct height at the actual available width.
                let style = taffy::Style {
                    size: taffy::Size { width: taffy::Dimension::Auto, height: taffy::Dimension::Auto },
                    ..style.clone()
                };
                taffy.new_leaf_with_context(style, Some((content.clone(), *font_size))).expect("taffy node")
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
                    size: taffy::Size { width: w, height: h },
                    ..style.clone()
                };
                taffy.new_leaf_with_context(style, None).expect("taffy node")
            }
        }
        View::Button { label, font_size, style, .. } => {
            let (tw, th) = measure(label, *font_size, 4096.0);
            let lp_val = |lp: taffy::LengthPercentage| match lp {
                taffy::LengthPercentage::Length(v) => v,
                taffy::LengthPercentage::Percent(_) => 0.0,
            };
            let pad_x = lp_val(style.padding.left) + lp_val(style.padding.right);
            let pad_y = lp_val(style.padding.top) + lp_val(style.padding.bottom);
            let w = match style.size.width {
                taffy::Dimension::Length(v) => taffy::Dimension::Length(v),
                _ => taffy::Dimension::Length(tw + pad_x),
            };
            let h = match style.size.height {
                taffy::Dimension::Length(v) => taffy::Dimension::Length(v),
                _ => taffy::Dimension::Length(th + pad_y),
            };
            let style = taffy::Style {
                size: taffy::Size { width: w, height: h },
                ..style.clone()
            };
            taffy.new_leaf(style).expect("taffy node")
        }
        View::Image { style, .. } => taffy.new_leaf_with_context(style.clone(), None).expect("taffy node"),
        View::Spacer => taffy.new_leaf_with_context(
            taffy::Style { flex_grow: 1.0, flex_shrink: 1.0, ..Default::default() },
            None,
        ).expect("taffy node"),
        View::Flexible { child, grow, shrink } => {
            let node = build_node(taffy, child, measure);
            let mut style = get_style(child);
            style.flex_grow = *grow;
            style.flex_shrink = *shrink;
            taffy.set_style(node, style).ok();
            node
        }
        View::Opacity { child, .. } => build_node(taffy, child, measure),
        View::Component(_) => unreachable!(),
    }
}

fn collect(
    taffy: &TaffyTree<Option<(String, f32)>>,
    node: NodeId,
    view: View,
    flat: &mut Vec<FlatView>,
    parent_x: f32,
    parent_y: f32,
) {
    let layout = *taffy.layout(node).expect("layout");
    let x = parent_x + layout.location.x;
    let y = parent_y + layout.location.y;
    let adjusted = Layout { location: taffy::Point { x, y }, ..layout };

    match view {
        View::Rect { color, .. } => {
            flat.push(FlatView { kind: FlatViewKind::Rect { color }, layout: adjusted });
        }
        View::Text { content, font_size, color, weight, align, wrap, .. } => {
            flat.push(FlatView {
                kind: FlatViewKind::Text { content, font_size, color, weight, align, wrap },
                layout: adjusted,
            });
        }
        View::Button { label, on_click, on_hover, on_press, bg_color, hover_bg_color, press_bg_color, text_color, corner_radius, font_size, .. } => {
            flat.push(FlatView {
                kind: FlatViewKind::Button { label, on_click, on_hover, on_press, bg_color, hover_bg_color, press_bg_color, text_color, corner_radius, font_size },
                layout: adjusted,
            });
        }
        View::TextInput { value, focused, cursor, placeholder, font_size, bg_color, text_color, border_color, corner_radius, on_change, on_submit, .. } => {
            flat.push(FlatView {
                kind: FlatViewKind::TextInput { value, focused, cursor, placeholder, font_size, bg_color, text_color, border_color, corner_radius, on_change, on_submit },
                layout: adjusted,
            });
        }
        View::Spacer => {}
        View::Column { children, bg_color, border_color, border_width, corner_radius, shadow, clip, .. }
        | View::Row { children, bg_color, border_color, border_width, corner_radius, shadow, clip, .. } => {
            if bg_color.is_some() || border_color.is_some() || shadow.is_some() {
                flat.push(FlatView {
                    kind: FlatViewKind::ContainerRect {
                        bg_color, border_color, border_width, corner_radius, shadow,
                    },
                    layout: adjusted,
                });
            }
            if clip {
                flat.push(FlatView {
                    kind: FlatViewKind::ClipStart { x, y, width: adjusted.size.width, height: adjusted.size.height },
                    layout: adjusted,
                });
            }
            let child_nodes = taffy.children(node).expect("children");
            for (child_node, child_view) in child_nodes.iter().zip(children.into_iter()) {
                collect(taffy, *child_node, child_view, flat, x, y);
            }
            if clip {
                flat.push(FlatView { kind: FlatViewKind::ClipEnd, layout: adjusted });
            }
        }
        View::ZStack { children, bg_color, border_color, border_width, corner_radius, shadow, .. } => {
            if bg_color.is_some() || border_color.is_some() || shadow.is_some() {
                flat.push(FlatView {
                    kind: FlatViewKind::ContainerRect { bg_color, border_color, border_width, corner_radius, shadow },
                    layout: adjusted,
                });
            }
            let child_nodes = taffy.children(node).expect("children");
            for (child_node, child_view) in child_nodes.iter().zip(children.into_iter()) {
                collect(taffy, *child_node, child_view, flat, x, y);
            }
        }
        View::Scroll { child, offset_x, offset_y, .. } => {
            let vw = adjusted.size.width;
            let vh = adjusted.size.height;
            flat.push(FlatView {
                kind: FlatViewKind::ClipStart { x, y, width: vw, height: vh },
                layout: adjusted,
            });
            let ox = offset_x.get();
            let oy = offset_y.get();
            let child_nodes = taffy.children(node).expect("children");
            collect(taffy, child_nodes[0], *child, flat, x - ox, y - oy);
            flat.push(FlatView { kind: FlatViewKind::ClipEnd, layout: adjusted });
        }
        View::Image { path, corner_radius, .. } => {
            flat.push(FlatView {
                kind: FlatViewKind::Image { path, corner_radius },
                layout: adjusted,
            });
        }
        View::Flexible { child, .. } => {
            collect(taffy, node, *child, flat, parent_x, parent_y);
        }
        View::Opacity { child, alpha } => {
            flat.push(FlatView { kind: FlatViewKind::OpacityStart { alpha }, layout: adjusted });
            collect(taffy, node, *child, flat, parent_x, parent_y);
            flat.push(FlatView { kind: FlatViewKind::OpacityEnd, layout: adjusted });
        }
        View::Component(_) => unreachable!(),
    }
}
