use crate::component::Component;
use crate::signal::Signal;
use taffy::Style;

/// Linear RGBA color with components in [0, 1].
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub color: Color,
}

impl Shadow {
    pub fn new(offset_x: f32, offset_y: f32, blur: f32, color: Color) -> Self {
        Self { offset_x, offset_y, blur, color }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontWeight {
    Regular,
    Bold,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextAlign {
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
        style: Style,
    },
    Text {
        content: String,
        font_size: f32,
        color: Color,
        weight: FontWeight,
        align: TextAlign,
        wrap: bool,
        style: Style,
    },
    Button {
        label: String,
        on_click: Box<dyn Fn()>,
        on_hover: Option<Box<dyn Fn(bool)>>,
        bg_color: Color,
        hover_bg_color: Option<Color>,
        text_color: Color,
        corner_radius: f32,
        font_size: f32,
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
    },
    /// A clipped, scrollable region. The child is laid out at its natural size;
    /// `offset_x` and `offset_y` shift content within the viewport.
    Scroll {
        child: Box<View>,
        offset_x: Signal<f32>,
        offset_y: Signal<f32>,
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
    /// `focused` is true when this field has keyboard focus.
    TextInput {
        value: Signal<String>,
        focused: Signal<bool>,
        placeholder: String,
        font_size: f32,
        bg_color: Color,
        text_color: Color,
        border_color: Color,
        corner_radius: f32,
        on_submit: Option<Box<dyn Fn(String)>>,
        style: Style,
    },
    /// Invisible flexible spacer. Fills remaining space along the parent axis.
    Spacer,
}

/// Builder returned by [`text`]. Call `.into()` or `.into_view()` to finish.
pub struct TextView {
    view: View,
}

impl TextView {
    pub fn color(mut self, color: Color) -> Self {
        if let View::Text { color: ref mut c, .. } = self.view {
            *c = color;
        }
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        if let View::Text { weight: ref mut w, .. } = self.view {
            *w = weight;
        }
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        if let View::Text { align: ref mut a, .. } = self.view {
            *a = align;
        }
        self
    }

    pub fn wrap(mut self) -> Self {
        if let View::Text { wrap: ref mut w, .. } = self.view {
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
        if let View::Button { ref mut bg_color, .. } = self.view {
            *bg_color = color;
        }
        self
    }

    pub fn hover_bg(mut self, color: Color) -> Self {
        if let View::Button { ref mut hover_bg_color, .. } = self.view {
            *hover_bg_color = Some(color);
        }
        self
    }

    pub fn on_hover(mut self, f: impl Fn(bool) + 'static) -> Self {
        if let View::Button { ref mut on_hover, .. } = self.view {
            *on_hover = Some(Box::new(f));
        }
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        if let View::Button { text_color: ref mut c, .. } = self.view {
            *c = color;
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::Button { ref mut corner_radius, .. } = self.view {
            *corner_radius = r;
        }
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        if let View::Button { font_size: ref mut s, .. } = self.view {
            *s = size;
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
            style: Style::default(),
        },
    }
}

pub fn button(label: impl Into<String>, on_click: impl Fn() + 'static) -> ButtonView {
    ButtonView {
        view: View::Button {
            label: label.into(),
            on_click: Box::new(on_click),
            on_hover: None,
            bg_color: Color::rgb(0.85, 0.85, 0.85),
            hover_bg_color: None,
            text_color: Color::BLACK,
            corner_radius: 8.0,
            font_size: 16.0,
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

    pub fn grow(mut self) -> Self {
        if let View::Column { ref mut style, .. } = self.view {
            style.flex_grow = 1.0;
        }
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        if let View::Column { ref mut bg_color, .. } = self.view {
            *bg_color = Some(color);
        }
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        if let View::Column { ref mut border_color, ref mut border_width, .. } = self.view {
            *border_color = Some(color);
            *border_width = width;
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::Column { ref mut corner_radius, .. } = self.view {
            *corner_radius = r;
        }
        self
    }

    pub fn shadow(mut self, s: Shadow) -> Self {
        if let View::Column { shadow: ref mut sh, .. } = self.view {
            *sh = Some(s);
        }
        self
    }

    pub fn clip(mut self) -> Self {
        if let View::Column { clip: ref mut c, .. } = self.view {
            *c = true;
        }
        self
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

    pub fn grow(mut self) -> Self {
        if let View::Row { ref mut style, .. } = self.view {
            style.flex_grow = 1.0;
        }
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        if let View::Row { ref mut bg_color, .. } = self.view {
            *bg_color = Some(color);
        }
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        if let View::Row { ref mut border_color, ref mut border_width, .. } = self.view {
            *border_color = Some(color);
            *border_width = width;
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::Row { ref mut corner_radius, .. } = self.view {
            *corner_radius = r;
        }
        self
    }

    pub fn shadow(mut self, s: Shadow) -> Self {
        if let View::Row { shadow: ref mut sh, .. } = self.view {
            *sh = Some(s);
        }
        self
    }

    pub fn clip(mut self) -> Self {
        if let View::Row { clip: ref mut c, .. } = self.view {
            *c = true;
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
                align_items: Some(taffy::AlignItems::Center),
                justify_content: Some(taffy::JustifyContent::Center),
                size: taffy::Size {
                    width: taffy::Dimension::Percent(1.0),
                    height: taffy::Dimension::Percent(1.0),
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
                align_items: Some(taffy::AlignItems::Center),
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
    scroll(inner, Signal::new(0.0), offset_y)
}

pub fn rect(color: Color) -> View {
    View::Rect {
        color,
        style: Style::default(),
    }
}

/// Wraps `child` so it grows to fill remaining space along the parent's main axis.
/// `grow` is the flex-grow factor (1.0 = fill equally with other flexible children).
pub fn flexible(child: impl Into<View>) -> View {
    View::Flexible { child: Box::new(child.into()), grow: 1.0, shrink: 1.0 }
}

/// Like [`flexible`] but with an explicit grow factor.
pub fn flex(child: impl Into<View>, grow: f32) -> View {
    View::Flexible { child: Box::new(child.into()), grow, shrink: 1.0 }
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
        if let View::Image { ref mut corner_radius, .. } = self.view {
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
        if let View::TextInput { ref mut placeholder, .. } = self.view {
            *placeholder = s.into();
        }
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        if let View::TextInput { ref mut font_size, .. } = self.view {
            *font_size = size;
        }
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        if let View::TextInput { ref mut bg_color, .. } = self.view {
            *bg_color = color;
        }
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        if let View::TextInput { text_color: ref mut c, .. } = self.view {
            *c = color;
        }
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        if let View::TextInput { border_color: ref mut c, .. } = self.view {
            *c = color;
        }
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        if let View::TextInput { ref mut corner_radius, .. } = self.view {
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

    pub fn on_submit(mut self, f: impl Fn(String) + 'static) -> Self {
        if let View::TextInput { ref mut on_submit, .. } = self.view {
            *on_submit = Some(Box::new(f));
        }
        self
    }
}

impl From<TextInputView> for View {
    fn from(t: TextInputView) -> Self {
        t.view
    }
}

/// A single-line text input. `value` and `focused` are shared signals — clone
/// them to read the current value or observe focus from other components.
pub fn text_input(value: Signal<String>, focused: Signal<bool>) -> TextInputView {
    TextInputView {
        view: View::TextInput {
            value,
            focused,
            placeholder: String::new(),
            font_size: 16.0,
            bg_color: Color::WHITE,
            text_color: Color::BLACK,
            border_color: Color::rgb(0.7, 0.7, 0.7),
            corner_radius: 6.0,
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

/// An invisible spacer that fills remaining space along the parent's main axis.
pub fn spacer() -> View {
    View::Spacer
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
}

impl From<ScrollView> for View {
    fn from(s: ScrollView) -> Self {
        s.view
    }
}

pub fn zstack(children: Vec<View>) -> View {
    View::ZStack {
        children,
        style: Style {
            size: taffy::Size {
                width: taffy::Dimension::Percent(1.0),
                height: taffy::Dimension::Percent(1.0),
            },
            ..Default::default()
        },
    }
}

/// A scrollable viewport. `offset_x` and `offset_y` are reactive signals that
/// the platform updates on mouse wheel events. Cloneable so the caller can
/// pass copies to event handlers or child components.
pub fn scroll(child: View, offset_x: Signal<f32>, offset_y: Signal<f32>) -> ScrollView {
    ScrollView {
        view: View::Scroll {
            child: Box::new(child),
            offset_x,
            offset_y,
            style: Style {
                size: taffy::Size {
                    width: taffy::Dimension::Percent(1.0),
                    height: taffy::Dimension::Percent(1.0),
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
