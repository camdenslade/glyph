use crate::view::{Color, FontWeight, TextAlign, View};
use taffy::{Layout, NodeId, Size, TaffyTree};

pub struct FlatView<'a> {
    pub kind: FlatViewKind<'a>,
    pub layout: Layout,
}

pub enum FlatViewKind<'a> {
    Rect {
        color: Color,
    },
    Text {
        content: &'a str,
        font_size: f32,
        color: Color,
        weight: FontWeight,
        align: TextAlign,
    },
    Button {
        label: &'a str,
        on_click: &'a dyn Fn(),
        bg_color: Color,
        text_color: Color,
        corner_radius: f32,
        font_size: f32,
    },
}

pub struct ViewTree;

impl ViewTree {
    /// Build a flat list of positioned views. `measure` returns (width, height) for a
    /// text string at a given font size and available width — used to size leaf nodes
    /// with real shaped metrics rather than a character-count approximation.
    pub fn build<'a>(
        root: &'a View,
        width: f32,
        height: f32,
        measure: &mut dyn FnMut(&str, f32, f32) -> (f32, f32),
    ) -> Vec<FlatView<'a>> {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let root_node = build_node(&mut taffy, root, measure);
        taffy
            .compute_layout(
                root_node,
                Size {
                    width: taffy::AvailableSpace::Definite(width),
                    height: taffy::AvailableSpace::Definite(height),
                },
            )
            .expect("layout failed");
        let mut flat = Vec::new();
        collect(&taffy, root_node, root, &mut flat, 0.0, 0.0);
        flat
    }
}

fn build_node(
    taffy: &mut TaffyTree<()>,
    view: &View,
    measure: &mut dyn FnMut(&str, f32, f32) -> (f32, f32),
) -> NodeId {
    match view {
        View::Column { children, style } | View::Row { children, style } => {
            let child_nodes: Vec<NodeId> =
                children.iter().map(|c| build_node(taffy, c, measure)).collect();
            taffy.new_with_children(style.clone(), &child_nodes).expect("taffy node")
        }
        View::Rect { style, .. } => taffy.new_leaf(style.clone()).expect("taffy node"),
        View::Text { content, font_size, style, .. } => {
            // Measure at unconstrained width; taffy constrains within the flex container.
            let (w, h) = measure(content, *font_size, 4096.0);
            let style = taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::Length(w),
                    height: taffy::Dimension::Length(h),
                },
                ..style.clone()
            };
            taffy.new_leaf(style).expect("taffy node")
        }
        View::Button { label, font_size, style, .. } => {
            let (tw, th) = measure(label, *font_size, 4096.0);
            let btn_w = tw + 24.0;
            let btn_h = th + 24.0;
            let style = taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::Length(btn_w),
                    height: taffy::Dimension::Length(btn_h),
                },
                ..style.clone()
            };
            taffy.new_leaf(style).expect("taffy node")
        }
    }
}

fn collect<'a>(
    taffy: &TaffyTree<()>,
    node: NodeId,
    view: &'a View,
    flat: &mut Vec<FlatView<'a>>,
    parent_x: f32,
    parent_y: f32,
) {
    let layout = *taffy.layout(node).expect("layout");
    let x = parent_x + layout.location.x;
    let y = parent_y + layout.location.y;
    let adjusted = Layout { location: taffy::Point { x, y }, ..layout };

    match view {
        View::Rect { color, .. } => {
            flat.push(FlatView { kind: FlatViewKind::Rect { color: *color }, layout: adjusted });
        }
        View::Text { content, font_size, color, weight, align, .. } => {
            flat.push(FlatView {
                kind: FlatViewKind::Text {
                    content: content.as_str(),
                    font_size: *font_size,
                    color: *color,
                    weight: *weight,
                    align: *align,
                },
                layout: adjusted,
            });
        }
        View::Button { label, on_click, bg_color, text_color, corner_radius, font_size, .. } => {
            flat.push(FlatView {
                kind: FlatViewKind::Button {
                    label: label.as_str(),
                    on_click: on_click.as_ref(),
                    bg_color: *bg_color,
                    text_color: *text_color,
                    corner_radius: *corner_radius,
                    font_size: *font_size,
                },
                layout: adjusted,
            });
        }
        View::Column { children, .. } | View::Row { children, .. } => {
            let child_nodes = taffy.children(node).expect("children");
            for (child_node, child_view) in child_nodes.iter().zip(children.iter()) {
                collect(taffy, *child_node, child_view, flat, x, y);
            }
        }
    }
}
