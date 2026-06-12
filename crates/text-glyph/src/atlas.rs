// Single R8Unorm texture; increase if the atlas fills up for large font sizes.
// PERF: Atlas never evicts glyphs — long sessions with many distinct characters
// (e.g. CJK) will exhaust the 1024×1024 budget silently. A LRU eviction policy
// or dynamic atlas growth (doubling to 2048) would handle these cases.
const ATLAS_SIZE: u32 = 1024;

/// UV coordinates and pixel metrics for one glyph in the atlas.
#[derive(Clone, Copy, Debug)]
pub struct GlyphUv {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
    pub width: u32,
    pub height: u32,
    pub left: i32,
    pub top: i32,
}

/// Shelf-packing glyph atlas backed by a single GPU texture.
///
/// Glyphs are packed left-to-right on the current shelf. When a glyph does
/// not fit horizontally a new shelf is started at `shelf_y + shelf_h`.
pub struct GlyphAtlas {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    shelf_x: u32,
    shelf_y: u32,
    shelf_h: u32,
}

impl GlyphAtlas {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyph_atlas"),
            size: wgpu::Extent3d { width: ATLAS_SIZE, height: ATLAS_SIZE, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Self { texture, view, sampler, shelf_x: 0, shelf_y: 0, shelf_h: 0 }
    }

    /// Pack a glyph bitmap into the atlas, upload to GPU, return UV rect.
    pub fn pack(
        &mut self,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
        data: &[u8],
        left: i32,
        top: i32,
    ) -> GlyphUv {
        if width == 0 || height == 0 {
            return GlyphUv { u0: 0.0, v0: 0.0, u1: 0.0, v1: 0.0, width: 0, height: 0, left, top };
        }

        if self.shelf_x + width > ATLAS_SIZE {
            self.shelf_y += self.shelf_h;
            self.shelf_x = 0;
            self.shelf_h = 0;
        }

        let x = self.shelf_x;
        let y = self.shelf_y;
        self.shelf_x += width;
        self.shelf_h = self.shelf_h.max(height);

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );

        let s = ATLAS_SIZE as f32;
        GlyphUv {
            u0: x as f32 / s,
            v0: y as f32 / s,
            u1: (x + width) as f32 / s,
            v1: (y + height) as f32 / s,
            width,
            height,
            left,
            top,
        }
    }
}
