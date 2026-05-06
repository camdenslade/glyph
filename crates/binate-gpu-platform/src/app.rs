use binate_gpu_core::{FlatViewKind, View, ViewTree, clear_redraw, needs_redraw};
use binate_gpu_render::Renderer;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

pub struct App<F: Fn() -> View> {
    build_view: F,
    state: Option<AppState>,
}

struct AppState {
    window: Arc<Window>,
    renderer: Renderer,
    cursor_pos: (f32, f32),
}

impl<F: Fn() -> View + 'static> App<F> {
    pub fn run(build_view: F) {
        let event_loop = EventLoop::new().expect("event loop");
        let mut app = App { build_view, state: None };
        event_loop.run_app(&mut app).expect("event loop run");
    }
}

impl<F: Fn() -> View + 'static> ApplicationHandler for App<F> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("binate-gpu")
                        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0)),
                )
                .expect("window"),
        );

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance
            .create_surface(Arc::clone(&window))
            .expect("surface");

        let renderer = pollster::block_on(Renderer::new(
            &instance,
            surface,
            size.width,
            size.height,
        ));

        self.state = Some(AppState { window, renderer, cursor_pos: (0.0, 0.0) });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => {
                state.renderer.resize(size.width, size.height);
                state.window.request_redraw();
            }

            WindowEvent::CursorMoved { position, .. } => {
                state.cursor_pos = (position.x as f32, position.y as f32);
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                let (cx, cy) = state.cursor_pos;
                let view = (self.build_view)();
                let w = state.renderer.surface_cfg.width as f32;
                let h = state.renderer.surface_cfg.height as f32;
                let flat = ViewTree::build(&view, w, h, &mut state.renderer.measurer());

                for fv in &flat {
                    if let FlatViewKind::Button { on_click, .. } = &fv.kind {
                        let l = fv.layout.location.x;
                        let t = fv.layout.location.y;
                        let r = l + fv.layout.size.width;
                        let b = t + fv.layout.size.height;
                        if cx >= l && cx <= r && cy >= t && cy <= b {
                            on_click();
                        }
                    }
                }
                if needs_redraw() {
                    clear_redraw();
                    state.window.request_redraw();
                }
            }

            WindowEvent::RedrawRequested => {
                let view = (self.build_view)();
                let w = state.renderer.surface_cfg.width as f32;
                let h = state.renderer.surface_cfg.height as f32;
                let flat = ViewTree::build(&view, w, h, &mut state.renderer.measurer());
                state.renderer.render(&flat);
            }

            _ => {}
        }
    }
}
