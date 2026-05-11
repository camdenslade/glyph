use crate::gpu_context::GpuContext;
use crate::image_cache::ImageCache;
use crate::pipeline::{ImagePipeline, ImageVertex, RectPipeline, RectVertex, ShadowPipeline, ShadowVertex, TextPipeline, TextVertex};
use glyph_core::{Color, FlatView, FlatViewKind, FontWeight, TextAlign};
use glyph_text::{GlyphAtlas, TextRenderer, measure_text};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Per-window GPU resources. The device and queue live in the shared
/// `GpuContext`; the surface, pipelines, atlas, and image cache are
/// per-window and owned here.
pub struct Renderer {
    ctx:               Arc<GpuContext>,
    pub surface:       wgpu::Surface<'static>,
    pub surface_cfg:   wgpu::SurfaceConfiguration,
    rect_pipeline:     RectPipeline,
    shadow_pipeline:   ShadowPipeline,
    text_pipeline:     TextPipeline,
    image_pipeline:    ImagePipeline,
    atlas:             GlyphAtlas,
    text_renderer:     TextRenderer,
    atlas_bind_group:  wgpu::BindGroup,
    image_cache:       ImageCache,
}

struct ImageCall {
    path: String,
    verts: Vec<ImageVertex>,
}

// A scissored draw batch. Each ClipStart flushes the previous batch and
// begins a new one with updated scissor bounds.
struct DrawBatch {
    scissor: Option<[u32; 4]>,
    cursor_visible: bool,
    shadow_verts: Vec<ShadowVertex>,
    rect_verts: Vec<RectVertex>,
    text_verts: Vec<TextVertex>,
    image_calls: Vec<ImageCall>,
}

impl DrawBatch {
    fn new(scissor: Option<[u32; 4]>, cursor_visible: bool) -> Self {
        Self { scissor, cursor_visible, shadow_verts: Vec::new(), rect_verts: Vec::new(), text_verts: Vec::new(), image_calls: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.shadow_verts.is_empty() && self.rect_verts.is_empty() && self.text_verts.is_empty() && self.image_calls.is_empty()
    }
}

impl Renderer {
    pub fn new(
        ctx: Arc<GpuContext>,
        surface: wgpu::Surface<'static>,
        surface_cfg: wgpu::SurfaceConfiguration,
    ) -> Self {
        let device = &ctx.device;
        let format = surface_cfg.format;
        let width = surface_cfg.width as f32;
        let height = surface_cfg.height as f32;

        let rect_pipeline   = RectPipeline::new(device, format, width, height);
        let shadow_pipeline = ShadowPipeline::new(device, format, width, height);
        let text_pipeline   = TextPipeline::new(device, format, width, height);
        let image_pipeline  = ImagePipeline::new(device, format, width, height);
        let atlas           = GlyphAtlas::new(device);
        let text_renderer   = TextRenderer::new();
        let atlas_bind_group = text_pipeline.make_atlas_bind_group(device, &atlas.view, &atlas.sampler);
        let image_cache     = ImageCache::new();

        Self { ctx, surface, surface_cfg, rect_pipeline, shadow_pipeline, text_pipeline, image_pipeline, atlas, text_renderer, atlas_bind_group, image_cache }
    }

    /// Returns a closure suitable for passing to `ViewTree::build` as the measure function.
    pub fn measurer(&mut self) -> impl FnMut(&str, f32, f32) -> (f32, f32) + '_ {
        |text, font_size, max_width| {
            measure_text(self.text_renderer.font_system_mut(), text, font_size, max_width)
        }
    }

    /// Given a string and a click x offset (relative to the text origin), return
    /// the byte index of the character boundary closest to that x position.
    pub fn cursor_for_x(&mut self, text: &str, font_size: f32, click_x: f32) -> usize {
        let mut best_idx = 0;
        let mut best_dist = f32::MAX;
        // Check position before each char and after the last one.
        let boundaries: Vec<usize> = std::iter::once(0)
            .chain(text.char_indices().map(|(i, _)| i))
            .chain(std::iter::once(text.len()))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        for idx in boundaries {
            let prefix_w = if idx == 0 {
                0.0
            } else {
                measure_text(self.text_renderer.font_system_mut(), &text[..idx], font_size, 4096.0).0
            };
            let dist = (prefix_w - click_x).abs();
            if dist < best_dist {
                best_dist = dist;
                best_idx = idx;
            }
        }
        best_idx
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.surface_cfg.width = width;
        self.surface_cfg.height = height;
        self.surface.configure(&self.ctx.device, &self.surface_cfg);
        self.rect_pipeline.update_screen(&self.ctx.queue, width as f32, height as f32);
        self.shadow_pipeline.update_screen(&self.ctx.queue, width as f32, height as f32);
        self.text_pipeline.update_screen(&self.ctx.queue, width as f32, height as f32);
        self.image_pipeline.update_screen(&self.ctx.queue, width as f32, height as f32);
    }

    /// Draw a frame. `cursor_visible` controls whether text input cursors are
    /// shown this frame — callers toggle it on a timer for blink effect.
    /// `bg` is the window clear color (should be `theme.background`).
    pub fn render(&mut self, views: Vec<FlatView>, cursor_visible: bool, bg: Color) {
        let sw = self.surface_cfg.width as f32;
        let sh = self.surface_cfg.height as f32;

        let mut batches: Vec<DrawBatch> = Vec::new();
        let mut clip_stack: Vec<Option<[u32; 4]>> = vec![None];
        let mut opacity_stack: Vec<f32> = Vec::new();
        let mut current = DrawBatch::new(None, cursor_visible);

        let current_alpha = |stack: &Vec<f32>| -> f32 {
            stack.iter().fold(1.0_f32, |a, &b| a * b)
        };
        let with_alpha = |mut c: Color, stack: &Vec<f32>| -> Color {
            c.a *= current_alpha(stack);
            c
        };

        for fv in views {
            match fv.kind {
                FlatViewKind::OpacityStart { alpha } => {
                    opacity_stack.push(alpha);
                }
                FlatViewKind::OpacityEnd => {
                    opacity_stack.pop();
                }
                FlatViewKind::ClipStart { x, y, width, height } => {
                    batches.push(current);
                    let scissor = scissor_rect(x, y, width, height, sw, sh);
                    clip_stack.push(Some(scissor));
                    current = DrawBatch::new(Some(scissor), cursor_visible);
                }
                FlatViewKind::ClipEnd => {
                    batches.push(current);
                    clip_stack.pop();
                    current = DrawBatch::new(*clip_stack.last().unwrap_or(&None), cursor_visible);
                }
                _ => {
                    let l = fv.layout.location.x;
                    let t = fv.layout.location.y;
                    let fw = fv.layout.size.width;
                    let fh = fv.layout.size.height;

                    match fv.kind {
                        FlatViewKind::Rect { color } => {
                            push_rect(&mut current.rect_verts, l, t, fw, fh, with_alpha(color, &opacity_stack), 0.0);
                        }
                        FlatViewKind::Button { label, bg_color, hover_bg_color, text_color, corner_radius, font_size, .. } => {
                            let draw_bg = with_alpha(hover_bg_color.unwrap_or(bg_color), &opacity_stack);
                            let text_color = with_alpha(text_color, &opacity_stack);
                            push_rect(&mut current.rect_verts, l, t, fw, fh, draw_bg, corner_radius);
                            let (_, th) = self.text_renderer.measure(&label, font_size, fw);
                            let text_y = t + (fh - th) * 0.5;
                            let quads = self.text_renderer.shape(
                                &mut self.atlas, &self.ctx.queue, &label, font_size,
                                text_color, FontWeight::Regular, TextAlign::Center,
                                l, text_y, fw,
                            );
                            self.atlas_bind_group = self.text_pipeline.make_atlas_bind_group(
                                &self.ctx.device, &self.atlas.view, &self.atlas.sampler,
                            );
                            for q in quads {
                                push_glyph(&mut current.text_verts, q.x, q.y, q.uv, q.color);
                            }
                        }
                        FlatViewKind::Text { content, font_size, color, weight, align, wrap } => {
                            let color = with_alpha(color, &opacity_stack);
                            let max_w = if wrap { fw } else { fw.max(sw) };
                            let quads = self.text_renderer.shape(
                                &mut self.atlas, &self.ctx.queue, &content, font_size,
                                color, weight, align, l, t, max_w,
                            );
                            self.atlas_bind_group = self.text_pipeline.make_atlas_bind_group(
                                &self.ctx.device, &self.atlas.view, &self.atlas.sampler,
                            );
                            for q in quads {
                                push_glyph(&mut current.text_verts, q.x, q.y, q.uv, q.color);
                            }
                        }
                        FlatViewKind::TextInput {
                            value, focused, cursor, placeholder,
                            font_size, bg_color, text_color, border_color, corner_radius, ..
                        } => {
                            let a = current_alpha(&opacity_stack);
                            let bg_color   = Color { a: bg_color.a * a,   ..bg_color };
                            let text_color = Color { a: text_color.a * a, ..text_color };
                            let border_color = Color { a: border_color.a * a, ..border_color };
                            let is_focused = focused.get();
                            let val = value.get();

                            push_rect(&mut current.rect_verts, l, t, fw, fh, bg_color, corner_radius);
                            push_rect(&mut current.rect_verts, l - 1.0, t - 1.0, fw + 2.0, fh + 2.0,
                                if is_focused { Color { a: a, ..Color::rgb(0.0, 0.47, 1.0) } } else { border_color },
                                corner_radius + 1.0,
                            );
                            push_rect(&mut current.rect_verts, l, t, fw, fh, bg_color, corner_radius);

                            let pad = 8.0;
                            let text_x = l + pad;
                            let text_y = t + (fh - font_size) / 2.0;

                            let (display, color) = if val.is_empty() {
                                (placeholder.clone(), Color { a: 0.6 * a, ..Color::rgb(0.6, 0.6, 0.6) })
                            } else {
                                (val.clone(), text_color)
                            };

                            if !display.is_empty() {
                                let quads = self.text_renderer.shape(
                                    &mut self.atlas, &self.ctx.queue, &display, font_size,
                                    color, FontWeight::Regular, TextAlign::Left,
                                    text_x, text_y, fw - pad * 2.0,
                                );
                                self.atlas_bind_group = self.text_pipeline.make_atlas_bind_group(
                                    &self.ctx.device, &self.atlas.view, &self.atlas.sampler,
                                );
                                for q in quads {
                                    push_glyph(&mut current.text_verts, q.x, q.y, q.uv, q.color);
                                }
                            }

                            if is_focused && current.cursor_visible {
                                let cur = cursor.get().min(val.len());
                                let prefix = &val[..cur];
                                let (prefix_w, _) = if prefix.is_empty() {
                                    (0.0_f32, 0.0_f32)
                                } else {
                                    self.text_renderer.measure(prefix, font_size, fw)
                                };
                                let cursor_x = (text_x + prefix_w).min(l + fw - pad);
                                let cursor_h = font_size * 1.2;
                                let cursor_y = t + (fh - cursor_h) / 2.0;
                                push_rect(&mut current.rect_verts, cursor_x, cursor_y, 2.0, cursor_h, text_color, 0.0);
                            }
                        }
                        FlatViewKind::TextArea {
                            value, focused, cursor, scroll_y, placeholder,
                            font_size, bg_color, text_color, border_color, corner_radius, ..
                        } => {
                            let a = current_alpha(&opacity_stack);
                            let bg_color   = Color { a: bg_color.a * a,   ..bg_color };
                            let text_color = Color { a: text_color.a * a, ..text_color };
                            let border_color = Color { a: border_color.a * a, ..border_color };
                            let is_focused = focused.get();
                            let val = value.get();
                            let oy = scroll_y.get();
                            let line_height = font_size * 1.4;

                            push_rect(&mut current.rect_verts, l, t, fw, fh, bg_color, corner_radius);
                            push_rect(&mut current.rect_verts, l - 1.0, t - 1.0, fw + 2.0, fh + 2.0,
                                if is_focused { Color { a, ..Color::rgb(0.0, 0.47, 1.0) } } else { border_color },
                                corner_radius + 1.0,
                            );
                            push_rect(&mut current.rect_verts, l, t, fw, fh, bg_color, corner_radius);

                            let pad = 8.0;
                            let text_x = l + pad;
                            let base_y = t + pad - oy;

                            let (display, display_color) = if val.is_empty() {
                                (placeholder.as_str(), Color { a: 0.6 * a, ..Color::rgb(0.6, 0.6, 0.6) })
                            } else {
                                (val.as_str(), text_color)
                            };

                            let lines: Vec<&str> = display.split('\n').collect();
                            for (li, line) in lines.iter().enumerate() {
                                let ly = base_y + li as f32 * line_height;
                                if ly + line_height < t || ly > t + fh { continue; }
                                if !line.is_empty() {
                                    let quads = self.text_renderer.shape(
                                        &mut self.atlas, &self.ctx.queue, line, font_size,
                                        display_color, FontWeight::Regular, TextAlign::Left,
                                        text_x, ly, fw - pad * 2.0,
                                    );
                                    self.atlas_bind_group = self.text_pipeline.make_atlas_bind_group(
                                        &self.ctx.device, &self.atlas.view, &self.atlas.sampler,
                                    );
                                    for q in quads {
                                        push_glyph(&mut current.text_verts, q.x, q.y, q.uv, q.color);
                                    }
                                }
                            }

                            if is_focused && current.cursor_visible && !val.is_empty() {
                                let cur = cursor.get().min(val.len());
                                let before = &val[..cur];
                                let newlines_before = before.chars().filter(|&c| c == '\n').count();
                                let line_start = before.rfind('\n').map(|p| p + 1).unwrap_or(0);
                                let cur_line_text = &val[line_start..cur];
                                let (prefix_w, _) = if cur_line_text.is_empty() {
                                    (0.0_f32, 0.0_f32)
                                } else {
                                    self.text_renderer.measure(cur_line_text, font_size, fw)
                                };
                                let cursor_x = (text_x + prefix_w).min(l + fw - pad);
                                let cursor_y = base_y + newlines_before as f32 * line_height;
                                let cursor_h = font_size * 1.2;
                                push_rect(&mut current.rect_verts, cursor_x, cursor_y, 2.0, cursor_h, text_color, 0.0);
                            } else if is_focused && current.cursor_visible && val.is_empty() {
                                push_rect(&mut current.rect_verts, text_x, base_y, 2.0, font_size * 1.2, text_color, 0.0);
                            }
                        }
                        FlatViewKind::ContainerRect { bg_color, border_color, border_width, corner_radius, shadow } => {
                            let a = current_alpha(&opacity_stack);
                            if let Some(sh) = shadow {
                                let shadow_color = Color { a: sh.color.a * a, ..sh.color };
                                push_shadow(
                                    &mut current.shadow_verts,
                                    l + sh.offset_x, t + sh.offset_y, fw, fh,
                                    corner_radius, sh.blur, shadow_color,
                                );
                            }
                            if let Some(bc) = border_color {
                                push_rect(&mut current.rect_verts, l - border_width, t - border_width,
                                    fw + border_width * 2.0, fh + border_width * 2.0,
                                    Color { a: bc.a * a, ..bc }, corner_radius + border_width);
                            }
                            if let Some(bg) = bg_color {
                                push_rect(&mut current.rect_verts, l, t, fw, fh,
                                    Color { a: bg.a * a, ..bg }, corner_radius);
                            }
                        }
                        FlatViewKind::Image { path, corner_radius } => {
                            let mut verts = Vec::new();
                            push_image_quad(&mut verts, l, t, fw, fh, corner_radius);
                            current.image_calls.push(ImageCall { path, verts });
                        }
                        _ => {}
                    }
                }
            }
        }

        batches.push(current);

        // Preload all image textures before the render pass borrows the encoder.
        for batch in &batches {
            for img_call in &batch.image_calls {
                self.image_cache.preload(&self.ctx.device, &self.ctx.queue, &img_call.path);
            }
        }

        let output = self.surface.get_current_texture().expect("surface texture");
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.ctx.device.create_command_encoder(&Default::default());

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("frame"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: bg.r as f64, g: bg.g as f64, b: bg.b as f64, a: bg.a as f64 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            for batch in &batches {
                if batch.is_empty() {
                    continue;
                }

                if let Some([sx, sy, sw, sh]) = batch.scissor {
                    rp.set_scissor_rect(sx, sy, sw, sh);
                } else {
                    rp.set_scissor_rect(0, 0, self.surface_cfg.width, self.surface_cfg.height);
                }

                if !batch.shadow_verts.is_empty() {
                    let vbuf = self.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(&batch.shadow_verts),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    rp.set_pipeline(&self.shadow_pipeline.pipeline);
                    rp.set_bind_group(0, &self.shadow_pipeline.bind_group, &[]);
                    rp.set_vertex_buffer(0, vbuf.slice(..));
                    rp.draw(0..batch.shadow_verts.len() as u32, 0..1);
                }

                if !batch.rect_verts.is_empty() {
                    let vbuf = self.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(&batch.rect_verts),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    rp.set_pipeline(&self.rect_pipeline.pipeline);
                    rp.set_bind_group(0, &self.rect_pipeline.bind_group, &[]);
                    rp.set_vertex_buffer(0, vbuf.slice(..));
                    rp.draw(0..batch.rect_verts.len() as u32, 0..1);
                }

                if !batch.text_verts.is_empty() {
                    let vbuf = self.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(&batch.text_verts),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    rp.set_pipeline(&self.text_pipeline.pipeline);
                    rp.set_bind_group(0, &self.text_pipeline.screen_bg, &[]);
                    rp.set_bind_group(1, &self.atlas_bind_group, &[]);
                    rp.set_vertex_buffer(0, vbuf.slice(..));
                    rp.draw(0..batch.text_verts.len() as u32, 0..1);
                }

                for img_call in &batch.image_calls {
                    if let Some(gpu_img) = self.image_cache.get(&img_call.path) {
                        let img_bg = self.image_pipeline.make_bind_group(
                            &self.ctx.device, &gpu_img.view, &gpu_img.sampler,
                        );
                        let vbuf = self.ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytemuck::cast_slice(&img_call.verts),
                            usage: wgpu::BufferUsages::VERTEX,
                        });
                        rp.set_pipeline(&self.image_pipeline.pipeline);
                        rp.set_bind_group(0, &self.image_pipeline.screen_bg, &[]);
                        rp.set_bind_group(1, &img_bg, &[]);
                        rp.set_vertex_buffer(0, vbuf.slice(..));
                        rp.draw(0..img_call.verts.len() as u32, 0..1);
                    }
                }
            }
        }

        self.ctx.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

/// Clamp a float scissor rect to physical pixel bounds, guarding against
/// out-of-viewport regions that wgpu rejects.
fn scissor_rect(x: f32, y: f32, w: f32, h: f32, sw: f32, sh: f32) -> [u32; 4] {
    let sw = sw as u32;
    let sh = sh as u32;
    let x0 = (x.max(0.0) as u32).min(sw);
    let y0 = (y.max(0.0) as u32).min(sh);
    let x1 = ((x + w).max(0.0) as u32).min(sw);
    let y1 = ((y + h).max(0.0) as u32).min(sh);
    let rw = (x1.saturating_sub(x0)).max(1).min(sw.saturating_sub(x0));
    let rh = (y1.saturating_sub(y0)).max(1).min(sh.saturating_sub(y0));
    [x0, y0, rw, rh]
}

fn push_rect(verts: &mut Vec<RectVertex>, x: f32, y: f32, w: f32, h: f32, color: Color, radius: f32) {
    let c = [color.r, color.g, color.b, color.a];
    let r = [x, y, x + w, y + h];
    verts.extend_from_slice(&[
        RectVertex { pos: [x,     y    ], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x + w, y    ], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x,     y + h], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x + w, y    ], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x + w, y + h], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x,     y + h], color: c, rect: r, radius, _pad: 0.0 },
    ]);
}

fn push_shadow(verts: &mut Vec<ShadowVertex>, x: f32, y: f32, w: f32, h: f32, radius: f32, blur: f32, color: Color) {
    // Expand the quad by 4σ on each side so the Gaussian tail fully fades out.
    let sigma = blur * 0.5;
    let expand = sigma * 4.0;
    let (qx, qy) = (x - expand, y - expand);
    let (qw, qh) = (w + expand * 2.0, h + expand * 2.0);
    let rect = [x, y, x + w, y + h];
    let c = [color.r, color.g, color.b, color.a];
    let params = [radius, sigma];
    verts.extend_from_slice(&[
        ShadowVertex { pos: [qx,      qy     ], rect, params, color: c },
        ShadowVertex { pos: [qx + qw, qy     ], rect, params, color: c },
        ShadowVertex { pos: [qx,      qy + qh], rect, params, color: c },
        ShadowVertex { pos: [qx + qw, qy     ], rect, params, color: c },
        ShadowVertex { pos: [qx + qw, qy + qh], rect, params, color: c },
        ShadowVertex { pos: [qx,      qy + qh], rect, params, color: c },
    ]);
}

fn push_image_quad(verts: &mut Vec<ImageVertex>, x: f32, y: f32, w: f32, h: f32, radius: f32) {
    let r = [x, y, x + w, y + h];
    verts.extend_from_slice(&[
        ImageVertex { pos: [x,     y    ], uv: [0.0, 0.0], rect: r, radius, _pad: 0.0 },
        ImageVertex { pos: [x + w, y    ], uv: [1.0, 0.0], rect: r, radius, _pad: 0.0 },
        ImageVertex { pos: [x,     y + h], uv: [0.0, 1.0], rect: r, radius, _pad: 0.0 },
        ImageVertex { pos: [x + w, y    ], uv: [1.0, 0.0], rect: r, radius, _pad: 0.0 },
        ImageVertex { pos: [x + w, y + h], uv: [1.0, 1.0], rect: r, radius, _pad: 0.0 },
        ImageVertex { pos: [x,     y + h], uv: [0.0, 1.0], rect: r, radius, _pad: 0.0 },
    ]);
}

fn push_glyph(verts: &mut Vec<TextVertex>, x: f32, y: f32, uv: glyph_text::GlyphUv, color: Color) {
    let w = uv.width as f32;
    let h = uv.height as f32;
    let c = [color.r, color.g, color.b, color.a];
    verts.extend_from_slice(&[
        TextVertex { pos: [x,     y    ], uv: [uv.u0, uv.v0], color: c },
        TextVertex { pos: [x + w, y    ], uv: [uv.u1, uv.v0], color: c },
        TextVertex { pos: [x,     y + h], uv: [uv.u0, uv.v1], color: c },
        TextVertex { pos: [x + w, y    ], uv: [uv.u1, uv.v0], color: c },
        TextVertex { pos: [x + w, y + h], uv: [uv.u1, uv.v1], color: c },
        TextVertex { pos: [x,     y + h], uv: [uv.u0, uv.v1], color: c },
    ]);
}
