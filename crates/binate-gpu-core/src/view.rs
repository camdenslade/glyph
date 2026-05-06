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

/// The UI tree. Each variant is a node; `Column` and `Row` are containers,
/// the rest are leaves that produce rendered output.
pub enum View {
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
        style: Style,
    },
    Button {
        label: String,
        on_click: Box<dyn Fn()>,
        bg_color: Color,
        text_color: Color,
        corner_radius: f32,
        font_size: f32,
        style: Style,
    },
    Column {
        children: Vec<View>,
        style: Style,
    },
    Row {
        children: Vec<View>,
        style: Style,
    },
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
            style: Style::default(),
        },
    }
}

pub fn button(label: impl Into<String>, on_click: impl Fn() + 'static) -> ButtonView {
    ButtonView {
        view: View::Button {
            label: label.into(),
            on_click: Box::new(on_click),
            bg_color: Color::rgb(0.85, 0.85, 0.85),
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

pub fn column(children: Vec<View>) -> View {
    View::Column {
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
    }
}

pub fn row(children: Vec<View>) -> View {
    View::Row {
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
    }
}

pub fn rect(color: Color) -> View {
    View::Rect {
        color,
        style: Style::default(),
    }
}
