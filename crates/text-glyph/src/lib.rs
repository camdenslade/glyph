//! Text shaping and glyph atlas management.
//!
//! `TextRenderer` uses cosmic-text to shape a string into positioned glyphs.
//! Each glyph bitmap is packed into a `GlyphAtlas` (a single R8 GPU texture)
//! on first use and cached by `CacheKey` for subsequent frames.

mod atlas;
mod renderer;

pub use atlas::{GlyphAtlas, GlyphUv};
pub use cosmic_text::FontSystem;
pub use renderer::{GlyphQuad, TextRenderer, measure_text};
