use std::sync::Arc;

/// Shared wgpu device and queue. One `GpuContext` is created per process and
/// shared across all windows via `Arc`. Each window has its own `Renderer`
/// (surface, pipelines, atlas) but they all submit to the same device.
pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub adapter:  wgpu::Adapter,
    pub device:   wgpu::Device,
    pub queue:    wgpu::Queue,
}

impl GpuContext {
    /// Create a context using a temporary surface to ensure the chosen adapter
    /// is compatible with the window's display. The surface is discarded after
    /// adapter selection; each window creates its own surface via `create_surface`.
    pub async fn new_with_window(window: Arc<winit::window::Window>) -> Arc<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create a temporary surface so we can pick an adapter guaranteed to
        // support this display (important on Windows with multiple GPUs).
        let temp_surface = instance.create_surface(Arc::clone(&window)).expect("surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&temp_surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("no wgpu adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("no wgpu device");

        Arc::new(Self { instance, adapter, device, queue })
    }

    /// Create a surface for `window` and configure it at `width x height`.
    /// The surface format is negotiated from the adapter's reported capabilities.
    /// Returns the surface and the format chosen for it.
    pub fn create_surface(
        &self,
        window: Arc<winit::window::Window>,
        width: u32,
        height: u32,
    ) -> (wgpu::Surface<'static>, wgpu::SurfaceConfiguration) {
        let surface = self.instance.create_surface(window).expect("surface");
        let caps = surface.get_capabilities(&self.adapter);
        let format = caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);
        let cfg = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&self.device, &cfg);
        (surface, cfg)
    }
}
