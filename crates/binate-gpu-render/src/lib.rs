//! GPU render pipelines and the main `Renderer`.
//!
//! Two wgpu pipelines are used per frame: the rect pipeline draws filled
//! rounded rectangles via an SDF fragment shader, and the text pipeline
//! draws glyph quads sampled from the atlas texture. Rects are submitted
//! first so text always composites on top.

mod pipeline;
mod renderer;

pub use renderer::Renderer;
