use glyph_core::{Color, Shadow};

pub fn shadow_sm() -> Shadow {
    Shadow::new(0.0, 1.0, 3.0, Color::rgba(0.0, 0.0, 0.0, 0.10))
}
pub fn shadow_md() -> Shadow {
    Shadow::new(0.0, 4.0, 12.0, Color::rgba(0.0, 0.0, 0.0, 0.12))
}
pub fn shadow_lg() -> Shadow {
    Shadow::new(0.0, 8.0, 24.0, Color::rgba(0.0, 0.0, 0.0, 0.14))
}
pub fn shadow_xl() -> Shadow {
    Shadow::new(0.0, 16.0, 40.0, Color::rgba(0.0, 0.0, 0.0, 0.18))
}
pub fn shadow_2xl() -> Shadow {
    Shadow::new(0.0, 24.0, 64.0, Color::rgba(0.0, 0.0, 0.0, 0.24))
}
pub fn shadow_dark_sm() -> Shadow {
    Shadow::new(0.0, 1.0, 4.0, Color::rgba(0.0, 0.0, 0.0, 0.30))
}
pub fn shadow_dark_md() -> Shadow {
    Shadow::new(0.0, 4.0, 16.0, Color::rgba(0.0, 0.0, 0.0, 0.40))
}
pub fn shadow_dark_lg() -> Shadow {
    Shadow::new(0.0, 8.0, 32.0, Color::rgba(0.0, 0.0, 0.0, 0.50))
}
pub fn shadow_colored(color: Color, blur: f32) -> Shadow {
    Shadow::new(0.0, blur * 0.33, blur, color)
}
