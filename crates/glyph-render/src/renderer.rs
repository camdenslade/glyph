use crate::image_cache::ImageCache;
use crate::pipeline::{ImagePipeline, RectPipeline, RectVertex, TextPipeline, TextVertex};
use glyph_core::{Color, FlatView, FlatViewKind, FontWeight, TextAlign};
use glyph_text::{GlyphAtlas, TextRenderer, measure_text};
use wgpu::util::DeviceExt;

// Shadow is approximated by drawing a slightly expanded, blurred rect behind the container.
// True Gaussian blur requires a separate render pass; we approximate with a semi-transparent
// expanded rect which is cheap and visually acceptable for UI drop shadows.
const SHADOW_STEPS: u32 = 4;

/// Owns all wgpu state and drives the render loop.
pub struct Renderer {
    pub device:        wgpu::Device,
    pub queue:         wgpu::Queue,
    pub surface:       wgpu::Surface<'static>,
    pub surface_cfg:   wgpu::SurfaceConfiguration,
    rect_pipeline:     RectPipeline,
    text_pipeline:     TextPipeline,
    image_pipeline:    ImagePipeline,
    atlas:             GlyphAtlas,
    text_renderer:     TextRenderer,
    atlas_bind_group:  wgpu::BindGroup,
    image_cache:       ImageCache,
}

struct ImageCall {
    path: String,
    verts: Vec<TextVertex>,
}

// A scissored draw batch. Each ClipStart flushes the previous batch and
// begins a new one with updated scissor bounds.
struct DrawBatch {
    scissor: Option<[u32; 4]>,
    cursor_visible: bool,
    rect_verts: Vec<RectVertex>,
    text_verts: Vec<TextVertex>,
    image_calls: Vec<ImageCall>,
}

impl DrawBatch {
    fn new(scissor: Option<[u32; 4]>, cursor_visible: bool) -> Self {
        Self { scissor, cursor_visible, rect_verts: Vec::new(), text_verts: Vec::new(), image_calls: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.rect_verts.is_empty() && self.text_verts.is_empty() && self.image_calls.is_empty()
    }
}

impl Renderer {
    pub async fn new(
        instance: &wgpu::Instance,
        surface: wgpu::Surface<'static>,
        width: u32,
        height: u32,
    ) -> Self {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("no adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("no device");

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);

        let surface_cfg = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_cfg);

        let rect_pipeline = RectPipeline::new(&device, format, width as f32, height as f32);
        let text_pipeline = TextPipeline::new(&device, format, width as f32, height as f32);
        let image_pipeline = ImagePipeline::new(&device, format, width as f32, height as f32);
        let atlas = GlyphAtlas::new(&device);
        let text_renderer = TextRenderer::new();
        let atlas_bind_group = text_pipeline.make_atlas_bind_group(&device, &atlas.view, &atlas.sampler);
        let image_cache = ImageCache::new();

        Self { device, queue, surface, surface_cfg, rect_pipeline, text_pipeline, image_pipeline, atlas, text_renderer, atlas_bind_group, image_cache }
    }

    /// Returns a closure suitable for passing to `ViewTree::build` as the measure function.
    pub fn measurer(&mut self) -> impl FnMut(&str, f32, f32) -> (f32, f32) + '_ {
        |text, font_size, max_width| {
            measure_text(self.text_renderer.font_system_mut(), text, font_size, max_width)
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.surface_cfg.width = width;
        self.surface_cfg.height = height;
        self.surface.configure(&self.device, &self.surface_cfg);
        self.rect_pipeline.update_screen(&self.queue, width as f32, height as f32);
        self.text_pipeline.update_screen(&self.queue, width as f32, height as f32);
        self.image_pipeline.update_screen(&self.queue, width as f32, height as f32);
    }

    /// Draw a frame. `cursor_visible` controls whether text input cursors are
    /// shown this frame — callers toggle it on a timer for blink effect.
    pub fn render(&mut self, views: Vec<FlatView>, cursor_visible: bool) {
        let sw = self.surface_cfg.width as f32;
        let sh = self.surface_cfg.height as f32;

        let mut batches: Vec<DrawBatch> = Vec::new();
        let mut clip_stack: Vec<Option<[u32; 4]>> = vec![None];
        let mut current = DrawBatch::new(None, cursor_visible);

        push_rect(&mut current.rect_verts, 0.0, 0.0, sw, sh, Color::WHITE, 0.0);

        for fv in views {
            match fv.kind {
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
                            push_rect(&mut current.rect_verts, l, t, fw, fh, color, 0.0);
                        }
                        FlatViewKind::Button { label, bg_color, hover_bg_color, text_color, corner_radius, font_size, .. } => {
                            let draw_bg = hover_bg_color.unwrap_or(bg_color);
                            push_rect(&mut current.rect_verts, l, t, fw, fh, draw_bg, corner_radius);
                            let quads = self.text_renderer.shape(
                                &mut self.atlas, &self.queue, &label, font_size,
                                text_color, FontWeight::Regular, TextAlign::Center,
                                l + 12.0, t + 12.0, fw - 24.0,
                            );
                            self.atlas_bind_group = self.text_pipeline.make_atlas_bind_group(
                                &self.device, &self.atlas.view, &self.atlas.sampler,
                            );
                            for q in quads {
                                push_glyph(&mut current.text_verts, q.x, q.y, q.uv, q.color);
                            }
                        }
                        FlatViewKind::Text { content, font_size, color, weight, align, wrap } => {
                            let max_w = if wrap { fw } else { fw.max(sw) };
                            let quads = self.text_renderer.shape(
                                &mut self.atlas, &self.queue, &content, font_size,
                                color, weight, align, l, t, max_w,
                            );
                            self.atlas_bind_group = self.text_pipeline.make_atlas_bind_group(
                                &self.device, &self.atlas.view, &self.atlas.sampler,
                            );
                            for q in quads {
                                push_glyph(&mut current.text_verts, q.x, q.y, q.uv, q.color);
                            }
                        }
                        FlatViewKind::TextInput {
                            value, focused, placeholder,
                            font_size, bg_color, text_color, border_color, corner_radius, ..
                        } => {
                            let is_focused = focused.get();
                            let val = value.get();

                            // Background
                            push_rect(&mut current.rect_verts, l, t, fw, fh, bg_color, corner_radius);

                            // Border: draw a slightly larger rect behind background for a 1px border effect
                            push_rect(&mut current.rect_verts, l - 1.0, t - 1.0, fw + 2.0, fh + 2.0,
                                if is_focused { Color::rgb(0.2, 0.4, 0.9) } else { border_color },
                                corner_radius + 1.0,
                            );
                            // Redraw bg on top of border
                            push_rect(&mut current.rect_verts, l, t, fw, fh, bg_color, corner_radius);

                            let pad = 8.0;
                            let text_x = l + pad;
                            let text_y = t + (fh - font_size) / 2.0;

                            let (display, color) = if val.is_empty() {
                                (placeholder.clone(), Color::rgb(0.6, 0.6, 0.6))
                            } else {
                                (val.clone(), text_color)
                            };

                            if !display.is_empty() {
                                let quads = self.text_renderer.shape(
                                    &mut self.atlas, &self.queue, &display, font_size,
                                    color, FontWeight::Regular, TextAlign::Left,
                                    text_x, text_y, fw - pad * 2.0,
                                );
                                self.atlas_bind_group = self.text_pipeline.make_atlas_bind_group(
                                    &self.device, &self.atlas.view, &self.atlas.sampler,
                                );
                                for q in quads {
                                    push_glyph(&mut current.text_verts, q.x, q.y, q.uv, q.color);
                                }
                            }

                            // Cursor: only when focused, driven by cursor_visible passed in via batch
                            if is_focused && current.cursor_visible {
                                let (text_w, _) = if val.is_empty() {
                                    (0.0, 0.0)
                                } else {
                                    // Approximate cursor x by measuring text width
                                    let (w, h) = self.text_renderer.measure(val.as_str(), font_size, fw);
                                    (w, h)
                                };
                                let cursor_x = (text_x + text_w).min(l + fw - pad);
                                let cursor_h = font_size * 1.2;
                                let cursor_y = t + (fh - cursor_h) / 2.0;
                                push_rect(&mut current.rect_verts, cursor_x, cursor_y, 2.0, cursor_h, text_color, 0.0);
                            }
                        }
                        FlatViewKind::ContainerRect { bg_color, border_color, border_width, corner_radius, shadow } => {
                            if let Some(sh) = shadow {
                                // Approximate shadow with layered semi-transparent rects
                                for i in 0..SHADOW_STEPS {
                                    let t = (i + 1) as f32 / SHADOW_STEPS as f32;
                                    let expand = sh.blur * t;
                                    let alpha = sh.color.a * (1.0 - t) * 0.4;
                                    let sc = Color::rgba(sh.color.r, sh.color.g, sh.color.b, alpha);
                                    push_rect(
                                        &mut current.rect_verts,
                                        l + sh.offset_x - expand,
                                        t + sh.offset_y - expand,
                                        fw + expand * 2.0,
                                        fh + expand * 2.0,
                                        sc,
                                        corner_radius + expand,
                                    );
                                }
                            }
                            if let Some(bc) = border_color {
                                push_rect(&mut current.rect_verts, l - border_width, t - border_width,
                                    fw + border_width * 2.0, fh + border_width * 2.0, bc, corner_radius + border_width);
                            }
                            if let Some(bg) = bg_color {
                                push_rect(&mut current.rect_verts, l, t, fw, fh, bg, corner_radius);
                            }
                        }
                        FlatViewKind::Image { path, .. } => {
                            let mut verts = Vec::new();
                            push_image_quad(&mut verts, l, t, fw, fh);
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
                self.image_cache.preload(&self.device, &self.queue, &img_call.path);
            }
        }

        let output = self.surface.get_current_texture().expect("surface texture");
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("frame"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
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

                if !batch.rect_verts.is_empty() {
                    let vbuf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
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
                    let vbuf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
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
                            &self.device, &gpu_img.view, &gpu_img.sampler,
                        );
                        let vbuf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
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

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

/// Clamp a float scissor rect to physical pixel bounds, guarding against
/// out-of-viewport regions that wgpu rejects.
fn scissor_rect(x: f32, y: f32, w: f32, h: f32, sw: f32, sh: f32) -> [u32; 4] {
    let x0 = x.max(0.0).min(sw) as u32;
    let y0 = y.max(0.0).min(sh) as u32;
    let x1 = (x + w).max(0.0).min(sw) as u32;
    let y1 = (y + h).max(0.0).min(sh) as u32;
    [x0, y0, (x1 - x0).max(1), (y1 - y0).max(1)]
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

fn push_image_quad(verts: &mut Vec<TextVertex>, x: f32, y: f32, w: f32, h: f32) {
    let c = [1.0f32, 1.0, 1.0, 1.0];
    verts.extend_from_slice(&[
        TextVertex { pos: [x,     y    ], uv: [0.0, 0.0], color: c },
        TextVertex { pos: [x + w, y    ], uv: [1.0, 0.0], color: c },
        TextVertex { pos: [x,     y + h], uv: [0.0, 1.0], color: c },
        TextVertex { pos: [x + w, y    ], uv: [1.0, 0.0], color: c },
        TextVertex { pos: [x + w, y + h], uv: [1.0, 1.0], color: c },
        TextVertex { pos: [x,     y + h], uv: [0.0, 1.0], color: c },
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
