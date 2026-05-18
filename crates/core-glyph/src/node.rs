//! Serializable layout node — the builder's save format.
//!
//! `LayoutNode` mirrors the `View` tree but contains only plain data (no
//! `Signal`s or closures), so it can be serialized to JSON and back.  The
//! builder saves a tree of these; `LayoutNode::into_view` turns it back into
//! a live `View` that Glyph can render.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::signal::Signal;
use crate::view::{
    Color, FontFamily, FontWeight, Shadow, TextAlign, View,
    button, column, image, rect, row, scroll, spacer, text, zstack,
};

/// A serializable description of a single UI node.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LayoutNode {
    /// Which component this node represents.
    pub kind: NodeKind,
    /// Child nodes (for containers).
    #[cfg_attr(feature = "serde", serde(default))]
    pub children: Vec<LayoutNode>,
    /// Layout properties.
    #[cfg_attr(feature = "serde", serde(default))]
    pub layout: LayoutProps,
    /// Visual properties.
    #[cfg_attr(feature = "serde", serde(default))]
    pub style: StyleProps,
}

/// Which kind of primitive or container this node is.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
pub enum NodeKind {
    Column,
    Row,
    ZStack,
    Spacer,
    Text {
        content: String,
        font_size: f32,
        #[cfg_attr(feature = "serde", serde(default))]
        weight: FontWeight,
        #[cfg_attr(feature = "serde", serde(default))]
        align: TextAlign,
        #[cfg_attr(feature = "serde", serde(default))]
        wrap: bool,
        #[cfg_attr(feature = "serde", serde(default))]
        family: FontFamily,
    },
    Button {
        label: String,
        font_size: f32,
        #[cfg_attr(feature = "serde", serde(default))]
        family: FontFamily,
    },
    Rect,
    Image {
        path: String,
        corner_radius: f32,
    },
    Scroll,
}

/// Flexbox layout properties.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LayoutProps {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub fill_width: bool,
    pub gap: Option<f32>,
    pub padding: Option<f32>,
    pub padding_x: Option<f32>,
    pub padding_y: Option<f32>,
}

/// Visual style properties.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StyleProps {
    pub bg_color: Option<Color>,
    pub text_color: Option<Color>,
    pub border_color: Option<Color>,
    pub corner_radius: Option<f32>,
    pub shadow: Option<Shadow>,
    pub opacity: Option<f32>,
}

impl LayoutNode {
    /// Convert this node tree into a live `View` ready for rendering.
    /// Button click handlers are no-ops — the builder wires them up separately.
    pub fn into_view(self) -> View {
        let children: Vec<View> = self.children.into_iter().map(|c| c.into_view()).collect();
        let l = &self.layout;
        let s = &self.style;

        let view = match self.kind {
            NodeKind::Column => {
                let mut v = column(children);
                if let Some(g) = l.gap { v = v.gap(g); }
                if let Some(p) = l.padding { v = v.padding(p); }
                if let Some(p) = l.padding_x { v = v.padding_x(p); }
                if let Some(p) = l.padding_y { v = v.padding_y(p); }
                if let Some(w) = l.width { v = v.width(w); }
                if let Some(h) = l.height { v = v.height(h); }
                if l.fill_width { v = v.fill_width(); }
                if let Some(c) = s.bg_color { v = v.bg(c); }
                if let Some(r) = s.corner_radius { v = v.radius(r); }
                if let Some(sh) = s.shadow { v = v.shadow(sh); }
                v.into()
            }
            NodeKind::Row => {
                let mut v = row(children);
                if let Some(g) = l.gap { v = v.gap(g); }
                if let Some(p) = l.padding { v = v.padding(p); }
                if let Some(p) = l.padding_x { v = v.padding_x(p); }
                if let Some(p) = l.padding_y { v = v.padding_y(p); }
                if let Some(w) = l.width { v = v.width(w); }
                if let Some(h) = l.height { v = v.height(h); }
                if l.fill_width { v = v.fill_width(); }
                if let Some(c) = s.bg_color { v = v.bg(c); }
                if let Some(r) = s.corner_radius { v = v.radius(r); }
                if let Some(sh) = s.shadow { v = v.shadow(sh); }
                v.into()
            }
            NodeKind::ZStack => {
                let mut v = zstack(children);
                if let (Some(w), Some(h)) = (l.width, l.height) { v = v.size(w, h); }
                v.into()
            }
            NodeKind::Spacer => spacer(),
            NodeKind::Text { content, font_size, weight, align, wrap, family } => {
                let mut v = text(content, font_size).weight(weight).align(align).family(family);
                if wrap { v = v.wrap(); }
                if let Some(c) = s.text_color { v = v.color(c); }
                if let Some(w) = l.width { v = v.width(w); }
                v.into()
            }
            NodeKind::Button { label, font_size, family } => {
                let mut v = button(label, || {}).font_size(font_size).family(family);
                if let Some(c) = s.bg_color { v = v.bg(c); }
                if let Some(c) = s.text_color { v = v.text_color(c); }
                if let Some(r) = s.corner_radius { v = v.radius(r); }
                v.into()
            }
            NodeKind::Rect => {
                let color = s.bg_color.unwrap_or(Color::TRANSPARENT);
                let mut v = rect(color);
                if let Some(w) = l.width { v = v.width(w); }
                if let Some(h) = l.height { v = v.height(h); }
                if l.fill_width { v = v.fill_width(); }
                if let Some(r) = s.corner_radius { v = v.radius(r); }
                v.into()
            }
            NodeKind::Image { path, corner_radius } => {
                let mut v = image(path).radius(corner_radius);
                if let Some(w) = l.width {
                    if let Some(h) = l.height {
                        v = v.size(w, h);
                    }
                }
                v.into()
            }
            NodeKind::Scroll => {
                let child = children.into_iter().next().unwrap_or_else(|| column(vec![]).into());
                scroll(
                    child,
                    Signal::new(0.0),
                    Signal::new(0.0),
                    Signal::new((-1.0, -1.0)),
                ).into()
            }
        };

        view
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    fn round_trip(node: &LayoutNode) -> LayoutNode {
        let json = serde_json::to_string(node).expect("serialize");
        serde_json::from_str(&json).expect("deserialize")
    }

    #[test]
    fn text_node_round_trips() {
        let node = LayoutNode {
            kind: NodeKind::Text {
                content: "Hello".into(),
                font_size: 16.0,
                weight: FontWeight::Bold,
                align: TextAlign::Center,
                wrap: true,
                family: FontFamily::SansSerif,
            },
            children: vec![],
            layout: LayoutProps { width: Some(200.0), ..Default::default() },
            style: StyleProps { text_color: Some(Color::BLACK), ..Default::default() },
        };
        let rt = round_trip(&node);
        assert!(matches!(rt.kind, NodeKind::Text { ref content, font_size, weight, align, wrap, .. }
            if content == "Hello" && font_size == 16.0 && weight == FontWeight::Bold
            && align == TextAlign::Center && wrap));
        assert_eq!(rt.layout.width, Some(200.0));
    }

    #[test]
    fn column_with_children_round_trips() {
        let node = LayoutNode {
            kind: NodeKind::Column,
            children: vec![
                LayoutNode {
                    kind: NodeKind::Text { content: "child".into(), font_size: 14.0, weight: FontWeight::Regular, align: TextAlign::Left, wrap: false, family: FontFamily::SansSerif },
                    children: vec![],
                    layout: LayoutProps::default(),
                    style: StyleProps::default(),
                },
                LayoutNode {
                    kind: NodeKind::Button { label: "Click".into(), font_size: 14.0, family: FontFamily::SansSerif },
                    children: vec![],
                    layout: LayoutProps::default(),
                    style: StyleProps { bg_color: Some(Color::rgb(0.0, 0.47, 1.0)), ..Default::default() },
                },
            ],
            layout: LayoutProps { gap: Some(12.0), padding: Some(16.0), ..Default::default() },
            style: StyleProps { bg_color: Some(Color::WHITE), corner_radius: Some(8.0), ..Default::default() },
        };
        let rt = round_trip(&node);
        assert!(matches!(rt.kind, NodeKind::Column));
        assert_eq!(rt.children.len(), 2);
        assert_eq!(rt.layout.gap, Some(12.0));
        assert_eq!(rt.layout.padding, Some(16.0));
        assert_eq!(rt.style.corner_radius, Some(8.0));
        assert!(matches!(rt.children[1].kind, NodeKind::Button { ref label, .. } if label == "Click"));
    }

    #[test]
    fn shadow_and_color_round_trip() {
        let node = LayoutNode {
            kind: NodeKind::Rect,
            children: vec![],
            layout: LayoutProps { width: Some(100.0), height: Some(50.0), ..Default::default() },
            style: StyleProps {
                bg_color: Some(Color::rgba(0.1, 0.2, 0.3, 0.9)),
                shadow: Some(Shadow::new(0.0, 2.0, 8.0, Color::rgba(0.0, 0.0, 0.0, 0.15))),
                ..Default::default()
            },
        };
        let rt = round_trip(&node);
        let color = rt.style.bg_color.unwrap();
        assert!((color.r - 0.1).abs() < 1e-5);
        assert!((color.a - 0.9).abs() < 1e-5);
        let shadow = rt.style.shadow.unwrap();
        assert_eq!(shadow.offset_y, 2.0);
        assert_eq!(shadow.blur, 8.0);
    }

    #[test]
    fn into_view_does_not_panic() {
        let node = LayoutNode {
            kind: NodeKind::Column,
            children: vec![
                LayoutNode {
                    kind: NodeKind::Text { content: "hi".into(), font_size: 16.0, weight: FontWeight::Regular, align: TextAlign::Left, wrap: false, family: FontFamily::SansSerif },
                    children: vec![],
                    layout: LayoutProps::default(),
                    style: StyleProps::default(),
                },
                LayoutNode {
                    kind: NodeKind::Scroll,
                    children: vec![
                        LayoutNode {
                            kind: NodeKind::Spacer,
                            children: vec![],
                            layout: LayoutProps::default(),
                            style: StyleProps::default(),
                        },
                    ],
                    layout: LayoutProps { width: Some(300.0), height: Some(200.0), ..Default::default() },
                    style: StyleProps::default(),
                },
            ],
            layout: LayoutProps { gap: Some(8.0), ..Default::default() },
            style: StyleProps::default(),
        };
        let _ = node.into_view(); // must not panic
    }
}
