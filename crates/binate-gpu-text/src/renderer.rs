use crate::atlas::{GlyphAtlas, GlyphUv};
use binate_gpu_core::{Color, FontWeight, TextAlign};
use cosmic_text::{Align, Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Style, SwashCache, Weight};
use std::collections::HashMap;

/// Returns the shaped (width, height) of `text` at `font_size` constrained to `max_width`.
pub fn measure_text(font_system: &mut FontSystem, text: &str, font_size: f32, max_width: f32) -> (f32, f32) {
    let metrics = Metrics::new(font_size, font_size * 1.2);
    let mut buffer = Buffer::new(font_system, metrics);
    buffer.set_size(font_system, Some(max_width), None);
    buffer.set_text(
        font_system,
        text,
        Attrs::new().family(Family::SansSerif).style(Style::Normal),
        Shaping::Advanced,
    );
    buffer.shape_until_scroll(font_system, false);

    let width = buffer.layout_runs().map(|r| r.line_w).fold(0.0_f32, f32::max);
    let height = buffer.layout_runs().count() as f32 * font_size * 1.2;
    (width, height)
}

pub struct GlyphQuad {
    pub x: f32,
    pub y: f32,
    pub uv: GlyphUv,
    pub color: Color,
}

pub struct TextRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    uv_cache: HashMap<cosmic_text::CacheKey, GlyphUv>,
}

impl TextRenderer {
    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }

    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            uv_cache: HashMap::new(),
        }
    }

    /// Shape text and return positioned glyph quads.
    /// Glyphs not yet in the atlas are uploaded via `atlas`.
    pub fn shape(
        &mut self,
        atlas: &mut GlyphAtlas,
        queue: &wgpu::Queue,
        text: &str,
        font_size: f32,
        color: Color,
        weight: FontWeight,
        align: TextAlign,
        x: f32,
        y: f32,
        max_width: f32,
    ) -> Vec<GlyphQuad> {
        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        buffer.set_size(&mut self.font_system, Some(max_width), None);
        let attrs = Attrs::new()
            .family(Family::SansSerif)
            .weight(match weight {
                FontWeight::Bold => Weight::BOLD,
                FontWeight::Regular => Weight::NORMAL,
            })
            .style(Style::Normal);
        buffer.set_text(&mut self.font_system, text, attrs, Shaping::Advanced);
        let cosmic_align = match align {
            TextAlign::Left => Align::Left,
            TextAlign::Center => Align::Center,
            TextAlign::Right => Align::Right,
        };
        for line in buffer.lines.iter_mut() {
            line.set_align(Some(cosmic_align));
        }
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut quads = Vec::new();
        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                let physical = glyph.physical((x, y + run.line_y), 1.0);
                let cache_key = physical.cache_key;

                let uv = if let Some(&uv) = self.uv_cache.get(&cache_key) {
                    uv
                } else {
                    let image_opt = self.swash_cache.get_image(&mut self.font_system, cache_key);
                    let uv = if let Some(image) = image_opt {
                        let (w, h) = (image.placement.width, image.placement.height);
                        atlas.pack(queue, w, h, &image.data, image.placement.left, image.placement.top)
                    } else {
                        GlyphUv { u0: 0.0, v0: 0.0, u1: 0.0, v1: 0.0, width: 0, height: 0, left: 0, top: 0 }
                    };
                    self.uv_cache.insert(cache_key, uv);
                    uv
                };

                if uv.width == 0 {
                    continue;
                }

                quads.push(GlyphQuad {
                    x: physical.x as f32 + uv.left as f32,
                    y: physical.y as f32 - uv.top as f32,
                    uv,
                    color,
                });
            }
        }
        quads
    }
}
