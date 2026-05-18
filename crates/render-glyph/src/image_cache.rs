use std::collections::HashMap;
use std::path::{Path, PathBuf};
use wgpu::util::DeviceExt;

pub struct GpuImage {
    pub view:    wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

/// Loads images from disk on first use and caches their GPU textures.
pub struct ImageCache {
    entries:  HashMap<String, GpuImage>,
    asset_dir: PathBuf,
}

impl ImageCache {
    pub fn new() -> Self {
        // Resolve assets relative to the executable so the app works regardless
        // of the working directory at launch.
        let asset_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("."));
        Self { entries: HashMap::new(), asset_dir }
    }

    fn resolve(&self, path: &str) -> PathBuf {
        let p = Path::new(path);
        if p.is_absolute() {
            return p.to_path_buf();
        }
        // Try exe-relative first, then cwd-relative, then src-relative.
        let exe_rel = self.asset_dir.join(p);
        if exe_rel.exists() {
            return exe_rel;
        }
        if let Ok(cwd) = std::env::current_dir() {
            let cwd_rel = cwd.join(p);
            if cwd_rel.exists() {
                return cwd_rel;
            }
            // Also try crates/demo-glyph/src/ for development assets
            let src_rel = cwd.join("crates/demo-glyph/src").join(p);
            if src_rel.exists() {
                return src_rel;
            }
        }
        exe_rel
    }

    /// Upload the image at `path` to the GPU if not already cached.
    /// Relative paths are resolved relative to the executable's directory.
    pub fn preload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, path: &str) {
        if self.entries.contains_key(path) {
            return;
        }
        let resolved = self.resolve(path);
        let Ok(img) = image::open(&resolved) else { return };
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
