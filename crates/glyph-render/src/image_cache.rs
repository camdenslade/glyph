use std::collections::HashMap;
use wgpu::util::DeviceExt;

pub struct GpuImage {
    pub view:    wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

/// Loads images from disk on first use and caches their GPU textures.
pub struct ImageCache {
    entries: HashMap<String, GpuImage>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    /// Upload the image at `path` to the GPU if not already cached.
    pub fn preload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, path: &str) {
        if self.entries.contains_key(path) {
            return;
        }
        let Ok(img) = image::open(path) else { return };
        let img = img.into_rgba8();
        let (w, h) = img.dimensions();
        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some(path),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &img,
        );
        let view = texture.create_view(&Default::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        self.entries.insert(path.to_string(), GpuImage { view, sampler });
    }

    pub fn get(&self, path: &str) -> Option<&GpuImage> {
        self.entries.get(path)
    }
}
