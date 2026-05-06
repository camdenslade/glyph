use crate::pipeline::{RectPipeline, RectVertex, TextPipeline, TextVertex};
use binate_gpu_core::{Color, FlatView, FlatViewKind, FontWeight, TextAlign};
use binate_gpu_text::{GlyphAtlas, TextRenderer, measure_text};
use wgpu::util::DeviceExt;

/// Owns all wgpu state and drives the two-pass render loop.
pub struct Renderer {
    pub device:        wgpu::Device,
    pub queue:         wgpu::Queue,
    pub surface:       wgpu::Surface<'static>,
    pub surface_cfg:   wgpu::SurfaceConfiguration,
    rect_pipeline:     RectPipeline,
    text_pipeline:     TextPipeline,
    atlas:             GlyphAtlas,
    text_renderer:     TextRenderer,
    atlas_bind_group:  wgpu::BindGroup,
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
        let atlas = GlyphAtlas::new(&device);
        let text_renderer = TextRenderer::new();

        let atlas_bind_group = text_pipeline.make_atlas_bind_group(&device, &atlas.view, &atlas.sampler);

        Self {
            device,
            queue,
            surface,
            surface_cfg,
            rect_pipeline,
            text_pipeline,
            atlas,
            text_renderer,
            atlas_bind_group,
        }
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
    }

    /// Draw a frame. Rect verts are submitted first, then text verts, so text
    /// always composites over backgrounds within the same render pass.
    pub fn render(&mut self, views: &[FlatView<'_>]) {
        let w = self.surface_cfg.width as f32;
        let h = self.surface_cfg.height as f32;

        let mut rect_verts: Vec<RectVertex> = Vec::new();
        let mut text_verts: Vec<TextVertex> = Vec::new();

        push_rect(&mut rect_verts, 0.0, 0.0, w, h, Color::WHITE, 0.0);

        for fv in views {
            let l = fv.layout.location.x;
            let t = fv.layout.location.y;
            let fw = fv.layout.size.width;
            let fh = fv.layout.size.height;

            match &fv.kind {
                FlatViewKind::Rect { color } => {
                    push_rect(&mut rect_verts, l, t, fw, fh, *color, 0.0);
                }
                FlatViewKind::Button { label, bg_color, text_color, corner_radius, font_size, .. } => {
                    push_rect(&mut rect_verts, l, t, fw, fh, *bg_color, *corner_radius);
                    let quads = self.text_renderer.shape(
                        &mut self.atlas,
                        &self.queue,
                        label,
                        *font_size,
                        *text_color,
                        FontWeight::Regular,
                        TextAlign::Center,
                        l + 12.0,
                        t + 12.0,
                        fw - 24.0,
                    );
                    self.atlas_bind_group = self.text_pipeline.make_atlas_bind_group(
                        &self.device,
                        &self.atlas.view,
                        &self.atlas.sampler,
                    );
                    for q in quads {
                        push_glyph(&mut text_verts, q.x, q.y, q.uv, q.color);
                    }
                }
                FlatViewKind::Text { content, font_size, color, weight, align } => {
                    let quads = self.text_renderer.shape(
                        &mut self.atlas,
                        &self.queue,
                        content,
                        *font_size,
                        *color,
                        *weight,
                        *align,
                        l,
                        t,
                        fw.max(w),
                    );
                    self.atlas_bind_group = self.text_pipeline.make_atlas_bind_group(
                        &self.device,
                        &self.atlas.view,
                        &self.atlas.sampler,
                    );
                    for q in quads {
                        push_glyph(&mut text_verts, q.x, q.y, q.uv, q.color);
                    }
                }
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

            if !rect_verts.is_empty() {
                let vbuf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&rect_verts),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                rp.set_pipeline(&self.rect_pipeline.pipeline);
                rp.set_bind_group(0, &self.rect_pipeline.bind_group, &[]);
                rp.set_vertex_buffer(0, vbuf.slice(..));
                rp.draw(0..rect_verts.len() as u32, 0..1);
            }

            if !text_verts.is_empty() {
                let vbuf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&text_verts),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                rp.set_pipeline(&self.text_pipeline.pipeline);
                rp.set_bind_group(0, &self.text_pipeline.screen_bg, &[]);
                rp.set_bind_group(1, &self.atlas_bind_group, &[]);
                rp.set_vertex_buffer(0, vbuf.slice(..));
                rp.draw(0..text_verts.len() as u32, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

fn push_rect(verts: &mut Vec<RectVertex>, x: f32, y: f32, w: f32, h: f32, color: Color, radius: f32) {
    let c = [color.r, color.g, color.b, color.a];
    let r = [x, y, x + w, y + h];
    // Two triangles (CCW)
    verts.extend_from_slice(&[
        RectVertex { pos: [x,     y    ], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x + w, y    ], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x,     y + h], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x + w, y    ], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x + w, y + h], color: c, rect: r, radius, _pad: 0.0 },
        RectVertex { pos: [x,     y + h], color: c, rect: r, radius, _pad: 0.0 },
    ]);
}

fn push_glyph(
    verts: &mut Vec<TextVertex>,
    x: f32,
    y: f32,
    uv: binate_gpu_text::GlyphUv,
    color: Color,
) {
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
