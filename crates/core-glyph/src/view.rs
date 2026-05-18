use crate::component::Component;
use crate::signal::Signal;
use std::sync::Arc;
use taffy::Style;

/// Linear RGBA color with components in [0, 1].
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// Types that can be linearly interpolated between two values.
pub trait Lerp: Clone + Send + 'static {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(a: &f32, b: &f32, t: f32) -> f32 {
        a + (b - a) * t
    }
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Parse a hex color string. Accepts `#rgb`, `#rrggbb`, `#rrggbbaa`.
    /// Panics on invalid input — use only with compile-time-known strings.
    pub fn hex(s: &str) -> Self {
        let s = s.trim_start_matches('#');
        let (r, g, b, a) = match s.len() {
            3 => {
                let r = u8::from_str_radix(&s[0..1].repeat(2), 16).unwrap();
                let g = u8::from_str_radix(&s[1..2].repeat(2), 16).unwrap();
                let b = u8::from_str_radix(&s[2..3].repeat(2), 16).unwrap();
                (r, g, b, 255u8)
            }
            6 => {
                let r = u8::from_str_radix(&s[0..2], 16).unwrap();
                let g = u8::from_str_radix(&s[2..4], 16).unwrap();
                let b = u8::from_str_radix(&s[4..6], 16).unwrap();
                (r, g, b, 255u8)
            }
            8 => {
                let r = u8::from_str_radix(&s[0..2], 16).unwrap();
                let g = u8::from_str_radix(&s[2..4], 16).unwrap();
                let b = u8::from_str_radix(&s[4..6], 16).unwrap();
                let a = u8::from_str_radix(&s[6..8], 16).unwrap();
                (r, g, b, a)
            }
            _ => panic!("Color::hex: invalid hex string"),
        };
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub color: Color,
}

impl Lerp for Color {
    fn lerp(a: &Color, b: &Color, t: f32) -> Color {
        Color {
            r: a.r + (b.r - a.r) * t,
            g: a.g + (b.g - a.g) * t,
            b: a.b + (b.b - a.b) * t,
            a: a.a + (b.a - a.a) * t,
        }
    }
}

impl Shadow {
    pub fn new(offset_x: f32, offset_y: f32, blur: f32, color: Color) -> Self {
        Self {
            offset_x,
            offset_y,
            blur,
            color,
        }
    }
}

/// Font family selector. `Name` loads a font previously registered via
/// `App::load_font` / `Renderer::load_font`. Falls back to the system
/// sans-serif if the name is not found.
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontFamily {
    /// The system default sans-serif (the default).
    #[default]
    SansSerif,
    /// Serif system font.
    Serif,
    /// Monospace system font.
    Monospace,
    /// A named font loaded via `load_font`.
    Name(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    #[default]
    Regular,
    Bold,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// The UI tree. Each variant is a node; `Column`, `Row`, and `Scroll` are
/// containers, the rest are leaves that produce rendered output.
pub enum View {
    /// Wraps a child and sets flex_grow/flex_shrink so it fills remaining space.
    Flexible {
        child: Box<View>,
        grow: f32,
        shrink: f32,
    },
    Rect {
        color: Color,
        corner_radius: f32,
        style: Style,
    },
    Text {
        content: String,
        font_size: f32,
        color: Color,
        weight: FontWeight,
        align: TextAlign,
        wrap: bool,
        family: FontFamily,
        style: Style,
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
        family: FontFamily,
        style: Style,
    },
    Column {
        children: Vec<View>,
        style: Style,
        bg_color: Option<Color>,
        border_color: Option<Color>,
        border_width: f32,
        corner_radius: f32,
        shadow: Option<Shadow>,
        clip: bool,
    },
    Row {
        children: Vec<View>,
        style: Style,
        bg_color: Option<Color>,
        border_color: Option<Color>,
        border_width: f32,
        corner_radius: f32,
        shadow: Option<Shadow>,
        clip: bool,
    },
    /// Children are stacked on top of each other, all sharing the same origin.
    /// Later children render above earlier ones. Size is determined by the largest child.
    ZStack {
        children: Vec<View>,
        style: Style,
        bg_color: Option<Color>,
        border_color: Option<Color>,
        border_width: f32,
        corner_radius: f32,
        shadow: Option<Shadow>,
    },
    /// A clipped, scrollable region. The child is laid out at its natural size;
    /// `offset_x` and `offset_y` shift content within the viewport.
    Scroll {
        child: Box<View>,
        offset_x: Signal<f32>,
        offset_y: Signal<f32>,
        /// Cached content size written by collect(); avoids a second taffy pass each frame.
        max_scroll: Signal<(f32, f32)>,
        style: Style,
    },
    /// An embedded component. Renders by calling `component.render()` inline,
    /// so the result is transparent to layout and the flat list.
    Component(Box<dyn Component>),
    /// A raster image loaded from a file path. Size must be set explicitly via
    /// the builder since there is no intrinsic-size measurement at layout time.
    Image {
        path: String,
        corner_radius: f32,
        style: Style,
    },
    /// A single-line text input field. `value` holds the current string;
    /// `focused` is true when this field has keyboard focus;
    /// `cursor` is the byte offset of the insertion point within `value`.
    TextInput {
        value: Signal<String>,
        focused: Signal<bool>,
        cursor: Signal<usize>,
        /// Horizontal scroll offset in logical pixels. Updated by the renderer
        /// when the cursor moves beyond the visible area.
        scroll_x: Signal<f32>,
        placeholder: String,
        font_size: f32,
        bg_color: Color,
        text_color: Color,
        border_color: Color,
        corner_radius: f32,
        on_change: Option<Arc<dyn Fn(String)>>,
        on_submit: Option<Arc<dyn Fn(String)>>,
        style: Style,
    },
    /// A virtualized list that only builds row views for the visible range.
    VirtualList {
        item_count: usize,
        row_height: f32,
        offset_y: Signal<f32>,
        build_row: Box<dyn Fn(usize) -> View>,
        viewport_height: f32,
        style: Style,
    },
    /// Invisible flexible spacer. Fills remaining space along the parent axis.
    Spacer,
    /// A multi-line text input field. `value` holds the current string;
    /// `focused` is true when this field has keyboard focus;
    /// `cursor` is the byte offset of the insertion point within `value`;
    /// `scroll_y` is the vertical scroll offset within the textarea.
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
        style: Style,
    },
    /// Multiplies the alpha of all descendant colors by `alpha`. Layout is
    /// pass-through — the child occupies exactly the same space as without this wrapper.
    Opacity { child: Box<View>, alpha: f32 },
}

/// Builder returned by [`text`]. Call `.into()` or `.into_view()` to finish.
pub struct TextView {
    view: View,
}

impl TextView {
    pub fn color(mut self, color: Color) -> Self {
        if let View::Text {
            color: ref mut c, ..
        } = self.view
        {
            *c = color;
        }
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        if let View::Text {
            weight: ref mut w, ..
        } = self.view
        {
            *w = weight;
        }
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        if let View::Text {
            align: ref mut a, ..
        } = self.view
        {
            *a = align;
        }
        self
    }

    pub fn wrap(mut self) -> Self {
        if let View::Text {
            wrap: ref mut w, ..
        } = self.view
        {
            *w = true;
        }
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        if let View::Text { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Length(w);
        }
        self
    }

    pub fn family(mut self, family: FontFamily) -> Self {
        if let View::Text { family: ref mut f, .. } = self.view {
            *f = family;
        }
        self
    }

    pub fn into_view(self) -> View {
        self.view
    }
}

impl From<TextView> for View {
    fn from(t: TextView) -> Self {
        t.view
    }
}

/// Builder returned by [`button`]. Call `.into()` or `.into_view()` to finish.
pub struct ButtonView {
    view: View,
}

impl ButtonView {
    pub fn bg(mut self, color: Color) -> Self {
        if let View::Button {
            ref mut bg_color, ..
        } = self.view
        {
            *bg_color = color;
        }
        self
    }

    pub fn hover_bg(mut self, color: Color) -> Self {
        if let View::Button {
            ref mut hover_bg_color,
            ..
        } = self.view
        {
            *hover_bg_color = Some(color);
        }
        self
    }

    pub fn on_hover(mut self, f: impl Fn(bool) + 'static) -> Self {
        if let View::Button {
            ref mut on_hover, ..
        } = self.view
        {
            *on_hover = Some(Arc::new(f));
        }
        self
    }

    pub fn on_press(mut self, f: impl Fn(bool) + 'static) -> Self {
        if let View::Button {
            ref mut on_press, ..
        } = self.view
        {
            *on_press = Some(Arc::new(f));
        }
        self
    }

    pub fn press_bg(mut self, color: Color) -> Self {
        if let View::Button {
            ref mut press_bg_color,
            ..
        } = self.view
        {
            *press_bg_color = Some(color);
        }
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        if let View::Button {
            text_color: ref mut c,
            ..
        } = self.view
        {
            *c = color;
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::Button {
            ref mut corner_radius,
            ..
        } = self.view
        {
            *corner_radius = r;
        }
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        if let View::Button {
            font_size: ref mut s,
            ..
        } = self.view
        {
            *s = size;
        }
        self
    }

    pub fn wrap(mut self) -> Self {
        if let View::Button { ref mut wrap, .. } = self.view {
            *wrap = true;
        }
        self
    }

    pub fn no_wrap(mut self) -> Self {
        if let View::Button { ref mut wrap, .. } = self.view {
            *wrap = false;
        }
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        if let View::Button { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Length(w);
        }
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        if let View::Button { ref mut style, .. } = self.view {
            style.size.height = taffy::Dimension::Length(h);
        }
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        if let View::Button { ref mut style, .. } = self.view {
            style.padding = taffy::Rect::length(p);
        }
        self
    }

    pub fn margin(mut self, m: f32) -> Self {
        if let View::Button { ref mut style, .. } = self.view {
            style.margin = taffy::Rect::length(m);
        }
        self
    }

    pub fn auto_size(mut self) -> Self {
        if let View::Button { ref mut style, .. } = self.view {
            style.size = taffy::Size::auto();
        }
        self
    }

    pub fn fill_width(mut self) -> Self {
        if let View::Button { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Percent(1.0);
        }
        self
    }

    pub fn grow(mut self) -> Self {
        if let View::Button { ref mut style, .. } = self.view {
            style.flex_grow = 1.0;
        }
        self
    }

    pub fn family(mut self, family: FontFamily) -> Self {
        if let View::Button { family: ref mut f, .. } = self.view {
            *f = family;
        }
        self
    }

    pub fn into_view(self) -> View {
        self.view
    }
}

impl From<ButtonView> for View {
    fn from(b: ButtonView) -> Self {
        b.view
    }
}

pub fn text(content: impl Into<String>, font_size: f32) -> TextView {
    TextView {
        view: View::Text {
            content: content.into(),
            font_size,
            color: Color::BLACK,
            weight: FontWeight::Regular,
            align: TextAlign::Left,
            wrap: false,
            family: FontFamily::SansSerif,
            style: Style::default(),
        },
    }
}

pub fn button(label: impl Into<String>, on_click: impl Fn() + 'static) -> ButtonView {
    ButtonView {
        view: View::Button {
            label: label.into(),
            on_click: Arc::new(on_click),
            on_hover: None,
            on_press: None,
            bg_color: Color::rgb(0.85, 0.85, 0.85),
            hover_bg_color: None,
            press_bg_color: None,
            text_color: Color::BLACK,
            corner_radius: 8.0,
            font_size: 16.0,
            wrap: false,
            family: FontFamily::SansSerif,
            style: Style {
                padding: taffy::Rect::length(12.0),
                ..Default::default()
            },
        },
    }
}

/// Builder returned by [`column`].
pub struct ColumnView {
    view: View,
}

impl ColumnView {
    pub fn gap(mut self, gap: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.gap.height = taffy::LengthPercentage::Length(gap);
        }
        self
    }

    pub fn align(mut self, align: taffy::AlignItems) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.align_items = Some(align);
        }
        self
    }

    pub fn justify(mut self, justify: taffy::JustifyContent) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.justify_content = Some(justify);
        }
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.padding = taffy::Rect::length(p);
        }
        self
    }

    pub fn padding_x(mut self, p: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            let lp = taffy::LengthPercentage::Length(p);
            style.padding.left = lp;
            style.padding.right = lp;
        }
        self
    }

    pub fn padding_y(mut self, p: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            let lp = taffy::LengthPercentage::Length(p);
            style.padding.top = lp;
            style.padding.bottom = lp;
        }
        self
    }

    pub fn padding_top(mut self, p: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.padding.top = taffy::LengthPercentage::Length(p);
        }
        self
    }

    pub fn padding_bottom(mut self, p: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.padding.bottom = taffy::LengthPercentage::Length(p);
        }
        self
    }

    pub fn padding_left(mut self, p: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.padding.left = taffy::LengthPercentage::Length(p);
        }
        self
    }

    pub fn padding_right(mut self, p: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.padding.right = taffy::LengthPercentage::Length(p);
        }
        self
    }

    pub fn margin(mut self, m: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.margin = taffy::Rect::length(m);
        }
        self
    }

    pub fn margin_x(mut self, m: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            let lp = taffy::LengthPercentageAuto::Length(m);
            style.margin.left = lp;
            style.margin.right = lp;
        }
        self
    }

    pub fn margin_y(mut self, m: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            let lp = taffy::LengthPercentageAuto::Length(m);
            style.margin.top = lp;
            style.margin.bottom = lp;
        }
        self
    }

    pub fn fill_height(mut self) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.size.height = taffy::Dimension::Percent(1.0);
        }
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.size = taffy::Size {
                width: taffy::Dimension::Length(width),
                height: taffy::Dimension::Length(height),
            };
        }
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Length(w);
        }
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.size.height = taffy::Dimension::Length(h);
        }
        self
    }

    pub fn min_size(mut self, width: f32, height: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.min_size = taffy::Size {
                width: taffy::Dimension::Length(width),
                height: taffy::Dimension::Length(height),
            };
        }
        self
    }

    pub fn max_size(mut self, width: f32, height: f32) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.max_size = taffy::Size {
                width: taffy::Dimension::Length(width),
                height: taffy::Dimension::Length(height),
            };
        }
        self
    }

    /// Remove the default 100%×100% size so the column shrinks to its content.
    pub fn auto_size(mut self) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.size = taffy::Size::auto();
        }
        self
    }

    pub fn grow(mut self) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.flex_grow = 1.0;
        }
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        if let View::Column {
            ref mut bg_color, ..
        } = self.view
        {
            *bg_color = Some(color);
        }
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        if let View::Column {
            ref mut border_color,
            ref mut border_width,
            ..
        } = self.view
        {
            *border_color = Some(color);
            *border_width = width;
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::Column {
            ref mut corner_radius,
            ..
        } = self.view
        {
            *corner_radius = r;
        }
        self
    }

    pub fn shadow(mut self, s: Shadow) -> Self {
        if let View::Column {
            shadow: ref mut sh, ..
        } = self.view
        {
            *sh = Some(s);
        }
        self
    }

    pub fn fill_width(mut self) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Percent(1.0);
        }
        self
    }

    pub fn clip(mut self) -> Self {
        if let View::Column {
            clip: ref mut c, ..
        } = self.view
        {
            *c = true;
        }
        self
    }

    pub fn no_gap(self) -> Self {
        self.gap(0.0)
    }

    pub fn align_start(self) -> Self {
        self.align(taffy::AlignItems::FlexStart)
    }
    pub fn align_center(self) -> Self {
        self.align(taffy::AlignItems::Center)
    }
    pub fn align_end(self) -> Self {
        self.align(taffy::AlignItems::FlexEnd)
    }
    pub fn align_stretch(self) -> Self {
        self.align(taffy::AlignItems::Stretch)
    }

    pub fn justify_start(self) -> Self {
        self.justify(taffy::JustifyContent::FlexStart)
    }
    pub fn justify_center(self) -> Self {
        self.justify(taffy::JustifyContent::Center)
    }
    pub fn justify_end(self) -> Self {
        self.justify(taffy::JustifyContent::FlexEnd)
    }
    pub fn justify_between(self) -> Self {
        self.justify(taffy::JustifyContent::SpaceBetween)
    }
    pub fn justify_around(self) -> Self {
        self.justify(taffy::JustifyContent::SpaceAround)
    }

    pub fn into_view(self) -> View {
        self.view
    }
}

impl From<ColumnView> for View {
    fn from(c: ColumnView) -> Self {
        c.view
    }
}

/// Builder returned by [`row`].
pub struct RowView {
    view: View,
}

impl RowView {
    pub fn gap(mut self, gap: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.gap.width = taffy::LengthPercentage::Length(gap);
        }
        self
    }

    pub fn align(mut self, align: taffy::AlignItems) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.align_items = Some(align);
        }
        self
    }

    pub fn justify(mut self, justify: taffy::JustifyContent) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.justify_content = Some(justify);
        }
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.padding = taffy::Rect::length(p);
        }
        self
    }

    pub fn padding_x(mut self, p: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            let lp = taffy::LengthPercentage::Length(p);
            style.padding.left = lp;
            style.padding.right = lp;
        }
        self
    }

    pub fn padding_y(mut self, p: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            let lp = taffy::LengthPercentage::Length(p);
            style.padding.top = lp;
            style.padding.bottom = lp;
        }
        self
    }

    pub fn padding_top(mut self, p: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.padding.top = taffy::LengthPercentage::Length(p);
        }
        self
    }

    pub fn padding_bottom(mut self, p: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.padding.bottom = taffy::LengthPercentage::Length(p);
        }
        self
    }

    pub fn padding_left(mut self, p: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.padding.left = taffy::LengthPercentage::Length(p);
        }
        self
    }

    pub fn padding_right(mut self, p: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.padding.right = taffy::LengthPercentage::Length(p);
        }
        self
    }

    pub fn margin(mut self, m: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.margin = taffy::Rect::length(m);
        }
        self
    }

    pub fn margin_x(mut self, m: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            let lp = taffy::LengthPercentageAuto::Length(m);
            style.margin.left = lp;
            style.margin.right = lp;
        }
        self
    }

    pub fn margin_y(mut self, m: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            let lp = taffy::LengthPercentageAuto::Length(m);
            style.margin.top = lp;
            style.margin.bottom = lp;
        }
        self
    }

    pub fn fill_height(mut self) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.size.height = taffy::Dimension::Percent(1.0);
        }
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.size = taffy::Size {
                width: taffy::Dimension::Length(width),
                height: taffy::Dimension::Length(height),
            };
        }
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Length(w);
        }
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.size.height = taffy::Dimension::Length(h);
        }
        self
    }

    pub fn min_size(mut self, width: f32, height: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.min_size = taffy::Size {
                width: taffy::Dimension::Length(width),
                height: taffy::Dimension::Length(height),
            };
        }
        self
    }

    pub fn max_size(mut self, width: f32, height: f32) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.max_size = taffy::Size {
                width: taffy::Dimension::Length(width),
                height: taffy::Dimension::Length(height),
            };
        }
        self
    }

    /// Remove the default auto width so the row shrinks to its content.
    pub fn auto_size(mut self) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.size = taffy::Size::auto();
        }
        self
    }

    pub fn grow(mut self) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.flex_grow = 1.0;
        }
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        if let View::Row {
            ref mut bg_color, ..
        } = self.view
        {
            *bg_color = Some(color);
        }
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        if let View::Row {
            ref mut border_color,
            ref mut border_width,
            ..
        } = self.view
        {
            *border_color = Some(color);
            *border_width = width;
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::Row {
            ref mut corner_radius,
            ..
        } = self.view
        {
            *corner_radius = r;
        }
        self
    }

    pub fn shadow(mut self, s: Shadow) -> Self {
        if let View::Row {
            shadow: ref mut sh, ..
        } = self.view
        {
            *sh = Some(s);
        }
        self
    }

    pub fn fill_width(mut self) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Percent(1.0);
        }
        self
    }

    pub fn clip(mut self) -> Self {
        if let View::Row {
            clip: ref mut c, ..
        } = self.view
        {
            *c = true;
        }
        self
    }

    pub fn no_gap(self) -> Self {
        self.gap(0.0)
    }

    pub fn align_start(self) -> Self {
        self.align(taffy::AlignItems::FlexStart)
    }
    pub fn align_center(self) -> Self {
        self.align(taffy::AlignItems::Center)
    }
    pub fn align_end(self) -> Self {
        self.align(taffy::AlignItems::FlexEnd)
    }
    pub fn align_stretch(self) -> Self {
        self.align(taffy::AlignItems::Stretch)
    }

    pub fn justify_start(self) -> Self {
        self.justify(taffy::JustifyContent::FlexStart)
    }
    pub fn justify_center(self) -> Self {
        self.justify(taffy::JustifyContent::Center)
    }
    pub fn justify_end(self) -> Self {
        self.justify(taffy::JustifyContent::FlexEnd)
    }
    pub fn justify_between(self) -> Self {
        self.justify(taffy::JustifyContent::SpaceBetween)
    }
    pub fn justify_around(self) -> Self {
        self.justify(taffy::JustifyContent::SpaceAround)
    }

    pub fn no_wrap(mut self) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.flex_wrap = taffy::FlexWrap::NoWrap;
        }
        self
    }

    pub fn into_view(self) -> View {
        self.view
    }
}

impl From<RowView> for View {
    fn from(r: RowView) -> Self {
        r.view
    }
}

pub fn column(children: Vec<View>) -> ColumnView {
    ColumnView {
        view: View::Column {
            children,
            style: Style {
                flex_direction: taffy::FlexDirection::Column,
                align_items: Some(taffy::AlignItems::Stretch),
                justify_content: Some(taffy::JustifyContent::FlexStart),
                size: taffy::Size {
                    width: taffy::Dimension::Percent(1.0),
                    height: taffy::Dimension::Auto,
                },
                gap: taffy::Size {
                    width: taffy::LengthPercentage::Length(0.0),
                    height: taffy::LengthPercentage::Length(16.0),
                },
                ..Default::default()
            },
            bg_color: None,
            border_color: None,
            border_width: 0.0,
            corner_radius: 0.0,
            shadow: None,
            clip: false,
        },
    }
}

pub fn row(children: Vec<View>) -> RowView {
    RowView {
        view: View::Row {
            children,
            style: Style {
                flex_direction: taffy::FlexDirection::Row,
                align_items: Some(taffy::AlignItems::Stretch),
                gap: taffy::Size {
                    width: taffy::LengthPercentage::Length(16.0),
                    height: taffy::LengthPercentage::Length(0.0),
                },
                ..Default::default()
            },
            bg_color: None,
            border_color: None,
            border_width: 0.0,
            corner_radius: 0.0,
            shadow: None,
            clip: false,
        },
    }
}

/// A scrollable vertical list with uniform item spacing. Simpler than
/// composing `scroll` + `column` manually for the common case.
pub fn list(items: Vec<View>, gap: f32, offset_y: Signal<f32>) -> ScrollView {
    let inner = View::Column {
        children: items,
        style: Style {
            flex_direction: taffy::FlexDirection::Column,
            align_items: Some(taffy::AlignItems::Stretch),
            gap: taffy::Size {
                width: taffy::LengthPercentage::Length(0.0),
                height: taffy::LengthPercentage::Length(gap),
            },
            size: taffy::Size {
                width: taffy::Dimension::Percent(1.0),
                height: taffy::Dimension::Auto,
            },
            ..Default::default()
        },
        bg_color: None,
        border_color: None,
        border_width: 0.0,
        corner_radius: 0.0,
        shadow: None,
        clip: false,
    };
    scroll(inner, Signal::new(0.0), offset_y, Signal::new((-1.0, -1.0)))
}

pub struct RectView {
    view: View,
}

impl RectView {
    pub fn width(mut self, w: f32) -> Self {
        if let View::Rect { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Length(w);
        }
        self
    }
    pub fn height(mut self, h: f32) -> Self {
        if let View::Rect { ref mut style, .. } = self.view {
            style.size.height = taffy::Dimension::Length(h);
        }
        self
    }
    pub fn fill_width(mut self) -> Self {
        if let View::Rect { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Percent(1.0);
        }
        self
    }
    pub fn radius(mut self, r: f32) -> Self {
        if let View::Rect {
            ref mut corner_radius,
            ..
        } = self.view
        {
            *corner_radius = r;
        }
        self
    }
}

impl From<RectView> for View {
    fn from(r: RectView) -> Self {
        r.view
    }
}

pub fn rect(color: Color) -> RectView {
    RectView {
        view: View::Rect {
            color,
            corner_radius: 0.0,
            style: Style::default(),
        },
    }
}

/// Wraps `child` so it grows to fill remaining space along the parent's main axis.
/// `grow` is the flex-grow factor (1.0 = fill equally with other flexible children).
pub fn flexible(child: impl Into<View>) -> View {
    View::Flexible {
        child: Box::new(child.into()),
        grow: 1.0,
        shrink: 1.0,
    }
}

/// Like [`flexible`] but with an explicit grow factor.
pub fn flex(child: impl Into<View>, grow: f32) -> View {
    View::Flexible {
        child: Box::new(child.into()),
        grow,
        shrink: 1.0,
    }
}

/// Builder returned by [`image`]. Call `.into()` to finish.
pub struct ImageView {
    view: View,
}

impl ImageView {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        if let View::Image { ref mut style, .. } = self.view {
            style.size = taffy::Size {
                width: taffy::Dimension::Length(width),
                height: taffy::Dimension::Length(height),
            };
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::Image {
            ref mut corner_radius,
            ..
        } = self.view
        {
            *corner_radius = r;
        }
        self
    }
}

impl From<ImageView> for View {
    fn from(i: ImageView) -> Self {
        i.view
    }
}

/// A raster image from `path`. Width and height default to 0 — call `.size()`
/// to give the image explicit dimensions in the layout.
pub fn image(path: impl Into<String>) -> ImageView {
    ImageView {
        view: View::Image {
            path: path.into(),
            corner_radius: 0.0,
            style: Style::default(),
        },
    }
}

/// Builder returned by [`text_input`]. Call `.into()` to finish.
pub struct TextInputView {
    view: View,
}

impl TextInputView {
    pub fn placeholder(mut self, s: impl Into<String>) -> Self {
        if let View::TextInput {
            ref mut placeholder,
            ..
        } = self.view
        {
            *placeholder = s.into();
        }
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        if let View::TextInput {
            ref mut font_size, ..
        } = self.view
        {
            *font_size = size;
        }
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        if let View::TextInput {
            ref mut bg_color, ..
        } = self.view
        {
            *bg_color = color;
        }
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        if let View::TextInput {
            text_color: ref mut c,
            ..
        } = self.view
        {
            *c = color;
        }
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        if let View::TextInput {
            border_color: ref mut c,
            ..
        } = self.view
        {
            *c = color;
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::TextInput {
            ref mut corner_radius,
            ..
        } = self.view
        {
            *corner_radius = r;
        }
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        if let View::TextInput { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Length(w);
        }
        self
    }

    pub fn fill_width(mut self) -> Self {
        if let View::TextInput { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Percent(1.0);
        }
        self
    }

    pub fn on_change(mut self, f: impl Fn(String) + 'static) -> Self {
        if let View::TextInput {
            ref mut on_change, ..
        } = self.view
        {
            *on_change = Some(Arc::new(f));
        }
        self
    }

    pub fn on_submit(mut self, f: impl Fn(String) + 'static) -> Self {
        if let View::TextInput {
            ref mut on_submit, ..
        } = self.view
        {
            *on_submit = Some(Arc::new(f));
        }
        self
    }
}

impl From<TextInputView> for View {
    fn from(t: TextInputView) -> Self {
        t.view
    }
}

/// A single-line text input. `value`, `focused`, and `cursor` are shared signals
/// that must be owned by the component to persist across frames.
pub fn text_input(
    value: Signal<String>,
    focused: Signal<bool>,
    cursor: Signal<usize>,
) -> TextInputView {
    TextInputView {
        view: View::TextInput {
            value,
            focused,
            cursor,
            scroll_x: Signal::new(0.0),
            placeholder: String::new(),
            font_size: 16.0,
            bg_color: Color::WHITE,
            text_color: Color::BLACK,
            border_color: Color::rgb(0.7, 0.7, 0.7),
            corner_radius: 6.0,
            on_change: None,
            on_submit: None,
            style: Style {
                size: taffy::Size {
                    width: taffy::Dimension::Length(240.0),
                    height: taffy::Dimension::Length(36.0),
                },
                ..Default::default()
            },
        },
    }
}

/// Builder returned by [`text_area`]. Call `.into()` to finish.
pub struct TextAreaView {
    view: View,
}

impl TextAreaView {
    pub fn placeholder(mut self, s: impl Into<String>) -> Self {
        if let View::TextArea {
            ref mut placeholder,
            ..
        } = self.view
        {
            *placeholder = s.into();
        }
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        if let View::TextArea {
            ref mut font_size, ..
        } = self.view
        {
            *font_size = size;
        }
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        if let View::TextArea {
            ref mut bg_color, ..
        } = self.view
        {
            *bg_color = color;
        }
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        if let View::TextArea {
            text_color: ref mut c,
            ..
        } = self.view
        {
            *c = color;
        }
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        if let View::TextArea {
            border_color: ref mut c,
            ..
        } = self.view
        {
            *c = color;
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::TextArea {
            ref mut corner_radius,
            ..
        } = self.view
        {
            *corner_radius = r;
        }
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        if let View::TextArea { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Length(w);
        }
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        if let View::TextArea { ref mut style, .. } = self.view {
            style.size.height = taffy::Dimension::Length(h);
        }
        self
    }

    pub fn fill_width(mut self) -> Self {
        if let View::TextArea { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Percent(1.0);
        }
        self
    }

    pub fn on_change(mut self, f: impl Fn(String) + 'static) -> Self {
        if let View::TextArea {
            ref mut on_change, ..
        } = self.view
        {
            *on_change = Some(Arc::new(f));
        }
        self
    }
}

impl From<TextAreaView> for View {
    fn from(t: TextAreaView) -> Self {
        t.view
    }
}

/// A multi-line text input. `value`, `focused`, `cursor`, and `scroll_y` are shared signals
/// that must be owned by the component to persist across frames.
pub fn text_area(
    value: Signal<String>,
    focused: Signal<bool>,
    cursor: Signal<usize>,
    scroll_y: Signal<f32>,
) -> TextAreaView {
    TextAreaView {
        view: View::TextArea {
            value,
            focused,
            cursor,
            scroll_y,
            placeholder: String::new(),
            font_size: 16.0,
            bg_color: Color::WHITE,
            text_color: Color::BLACK,
            border_color: Color::rgb(0.7, 0.7, 0.7),
            corner_radius: 6.0,
            on_change: None,
            style: Style {
                size: taffy::Size {
                    width: taffy::Dimension::Length(240.0),
                    height: taffy::Dimension::Length(120.0),
                },
                ..Default::default()
            },
        },
    }
}

/// An invisible spacer that fills remaining space along the parent's main axis.
pub fn spacer() -> View {
    View::Spacer
}

pub fn opacity(alpha: f32, child: impl Into<View>) -> View {
    View::Opacity {
        child: Box::new(child.into()),
        alpha,
    }
}

/// Builder returned by [`scroll`]. Call `.into()` to finish.
pub struct ScrollView {
    view: View,
}

impl ScrollView {
    /// Override the viewport size. By default the scroll region fills its parent.
    pub fn size(mut self, width: f32, height: f32) -> Self {
        if let View::Scroll { ref mut style, .. } = self.view {
            style.size = taffy::Size {
                width: taffy::Dimension::Length(width),
                height: taffy::Dimension::Length(height),
            };
        }
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        if let View::Scroll { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Length(w);
        }
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        if let View::Scroll { ref mut style, .. } = self.view {
            style.size.height = taffy::Dimension::Length(h);
        }
        self
    }

    pub fn fill_width(mut self) -> Self {
        if let View::Scroll { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Percent(1.0);
        }
        self
    }

    pub fn grow(mut self) -> Self {
        if let View::Scroll { ref mut style, .. } = self.view {
            style.flex_grow = 1.0;
            style.size.height = taffy::Dimension::Auto;
        }
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        if let View::Scroll { ref mut style, .. } = self.view {
            style.padding = taffy::Rect::length(p);
        }
        self
    }
}

impl From<ScrollView> for View {
    fn from(s: ScrollView) -> Self {
        s.view
    }
}

pub struct ZStackView {
    view: View,
}

impl ZStackView {
    pub fn bg(mut self, color: Color) -> Self {
        if let View::ZStack {
            ref mut bg_color, ..
        } = self.view
        {
            *bg_color = Some(color);
        }
        self
    }
    pub fn border(mut self, color: Color, width: f32) -> Self {
        if let View::ZStack {
            ref mut border_color,
            ref mut border_width,
            ..
        } = self.view
        {
            *border_color = Some(color);
            *border_width = width;
        }
        self
    }
    pub fn radius(mut self, r: f32) -> Self {
        if let View::ZStack {
            ref mut corner_radius,
            ..
        } = self.view
        {
            *corner_radius = r;
        }
        self
    }
    pub fn shadow(mut self, s: Shadow) -> Self {
        if let View::ZStack {
            shadow: ref mut sh, ..
        } = self.view
        {
            *sh = Some(s);
        }
        self
    }
    pub fn size(mut self, width: f32, height: f32) -> Self {
        if let View::ZStack { ref mut style, .. } = self.view {
            style.size = taffy::Size {
                width: taffy::Dimension::Length(width),
                height: taffy::Dimension::Length(height),
            };
        }
        self
    }
}

impl From<ZStackView> for View {
    fn from(z: ZStackView) -> Self {
        z.view
    }
}

pub fn zstack(children: Vec<View>) -> ZStackView {
    ZStackView {
        view: View::ZStack {
            children,
            style: Style {
                size: taffy::Size {
                    width: taffy::Dimension::Percent(1.0),
                    height: taffy::Dimension::Percent(1.0),
                },
                ..Default::default()
            },
            bg_color: None,
            border_color: None,
            border_width: 0.0,
            corner_radius: 0.0,
            shadow: None,
        },
    }
}

/// Builder returned by [`virtual_list`].
pub struct VirtualListView {
    view: View,
}

impl VirtualListView {
    pub fn width(mut self, w: f32) -> Self {
        if let View::VirtualList { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Length(w);
        }
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        if let View::VirtualList {
            ref mut style,
            ref mut viewport_height,
            ..
        } = self.view
        {
            style.size.height = taffy::Dimension::Length(h);
            *viewport_height = h;
        }
        self
    }

    pub fn fill_width(mut self) -> Self {
        if let View::VirtualList { ref mut style, .. } = self.view {
            style.size.width = taffy::Dimension::Percent(1.0);
        }
        self
    }
}

impl From<VirtualListView> for View {
    fn from(v: VirtualListView) -> Self {
        v.view
    }
}

pub fn virtual_list(
    item_count: usize,
    row_height: f32,
    offset_y: Signal<f32>,
    viewport_height: f32,
    build_row: impl Fn(usize) -> View + 'static,
) -> VirtualListView {
    VirtualListView {
        view: View::VirtualList {
            item_count,
            row_height,
            offset_y,
            build_row: Box::new(build_row),
            viewport_height,
            style: Style {
                size: taffy::Size {
                    width: taffy::Dimension::Percent(1.0),
                    height: taffy::Dimension::Length(viewport_height),
                },
                overflow: taffy::Point {
                    x: taffy::Overflow::Hidden,
                    y: taffy::Overflow::Hidden,
                },
                ..Default::default()
            },
        },
    }
}

/// A scrollable viewport. `offset_x` and `offset_y` are reactive signals that
/// the platform updates on mouse wheel events. Cloneable so the caller can
/// pass copies to event handlers or child components.
pub fn scroll(
    child: View,
    offset_x: Signal<f32>,
    offset_y: Signal<f32>,
    max_scroll: Signal<(f32, f32)>,
) -> ScrollView {
    ScrollView {
        view: View::Scroll {
            child: Box::new(child),
            offset_x,
            offset_y,
            max_scroll,
            style: Style {
                size: taffy::Size {
                    width: taffy::Dimension::Percent(1.0),
                    height: taffy::Dimension::Percent(1.0),
                },
                overflow: taffy::Point {
                    x: taffy::Overflow::Hidden,
                    y: taffy::Overflow::Hidden,
                },
                align_items: Some(taffy::AlignItems::FlexStart),
                justify_content: Some(taffy::JustifyContent::FlexStart),
                ..Default::default()
            },
        },
    }
}
