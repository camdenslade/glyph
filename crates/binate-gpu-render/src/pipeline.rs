use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

/// Vertex for the rect pipeline. `rect` carries the pixel-space bounds of the
/// full rectangle to every vertex so the SDF shader can compute corner distance
/// regardless of which corner the vertex sits at. `_pad` aligns the struct to
/// 16 bytes as required by wgpu's vertex buffer layout rules.
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct RectVertex {
    pub pos:    [f32; 2],
    pub color:  [f32; 4],
    pub rect:   [f32; 4], // pixel-space bounds: [x0, y0, x1, y1]
    pub radius: f32,
    pub _pad:   f32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct TextVertex {
    pub pos:   [f32; 2],
    pub uv:    [f32; 2],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ScreenUniform {
    pub size: [f32; 2],
}

/// Renders filled rectangles with optional SDF rounded corners.
pub struct RectPipeline {
    pub pipeline:   wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub screen_buf: wgpu::Buffer,
}

impl RectPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, width: f32, height: f32) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/rect.wgsl"));

        let screen_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("screen_uniform"),
            contents: bytemuck::cast_slice(&[ScreenUniform { size: [width, height] }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rect_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rect_bg"),
            layout: &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: screen_buf.as_entire_binding(),
            }],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("rect_layout"),
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("rect_pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<RectVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Float32x2,  // pos
                        1 => Float32x4,  // color
                        2 => Float32x4,  // rect bounds
                        3 => Float32,    // radius
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self { pipeline, bind_group, screen_buf }
    }

    pub fn update_screen(&self, queue: &wgpu::Queue, width: f32, height: f32) {
        queue.write_buffer(
            &self.screen_buf,
            0,
            bytemuck::cast_slice(&[ScreenUniform { size: [width, height] }]),
        );
    }
}

/// Renders glyph quads sampled from the R8 atlas texture.
pub struct TextPipeline {
    pub pipeline:      wgpu::RenderPipeline,
    pub screen_bg:     wgpu::BindGroup,
    pub screen_buf:    wgpu::Buffer,
    pub atlas_bgl:     wgpu::BindGroupLayout,
}

impl TextPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, width: f32, height: f32) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/text.wgsl"));

        let screen_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text_screen_uniform"),
            contents: bytemuck::cast_slice(&[ScreenUniform { size: [width, height] }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let screen_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("text_screen_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let screen_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("text_screen_bg"),
            layout: &screen_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: screen_buf.as_entire_binding(),
            }],
        });

        let atlas_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("text_atlas_bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text_layout"),
            bind_group_layouts: &[&screen_bgl, &atlas_bgl],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text_pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<TextVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self { pipeline, screen_bg, screen_buf, atlas_bgl }
    }

    pub fn update_screen(&self, queue: &wgpu::Queue, width: f32, height: f32) {
        queue.write_buffer(
            &self.screen_buf,
            0,
            bytemuck::cast_slice(&[ScreenUniform { size: [width, height] }]),
        );
    }

    pub fn make_atlas_bind_group(
        &self,
        device: &wgpu::Device,
        view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("text_atlas_bg"),
            layout: &self.atlas_bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(sampler) },
            ],
        })
    }
}
