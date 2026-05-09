use glyph_core::{FlatView, FlatViewKind, Theme, View, ViewTree, clear_redraw, needs_redraw};
use glyph_render::Renderer;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

pub struct App<F: Fn(&Theme) -> View> {
    build_view: F,
    theme: Theme,
    title: String,
    width: f64,
    height: f64,
    state: Option<AppState>,
}

struct AppState {
    window: Arc<Window>,
    renderer: Renderer,
    cursor_pos: (f32, f32),
    frame: u32,
}

impl<F: Fn(&Theme) -> View + 'static> App<F> {
    pub fn run(build_view: F) {
        Self::run_with_config(build_view, Theme::light(), "glyph", 800.0, 600.0)
    }

    pub fn run_with_theme(build_view: F, theme: Theme) {
        Self::run_with_config(build_view, theme, "glyph", 800.0, 600.0)
    }

    pub fn run_with_config(build_view: F, theme: Theme, title: impl Into<String>, width: f64, height: f64) {
        let event_loop = EventLoop::new().expect("event loop");
        let mut app = App { build_view, theme, title: title.into(), width, height, state: None };
        event_loop.run_app(&mut app).expect("event loop run");
    }
}

impl<F: Fn(&Theme) -> View + 'static> ApplicationHandler for App<F> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.title)
                        .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height)),
                )
                .expect("window"),
        );

        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(Arc::clone(&window)).expect("surface");
        let renderer = pollster::block_on(Renderer::new(&instance, surface, size.width, size.height));

        self.state = Some(AppState { window, renderer, cursor_pos: (0.0, 0.0), frame: 0 });
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
                let (px, py) = (position.x as f32, position.y as f32);
                state.cursor_pos = (px, py);

                let view = (self.build_view)(&self.theme);
                let w = state.renderer.surface_cfg.width as f32;
                let h = state.renderer.surface_cfg.height as f32;
                let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                let mut changed = false;
                for fv in &flat {
                    let l = fv.layout.location.x;
                    let t = fv.layout.location.y;
                    let r = l + fv.layout.size.width;
                    let b = t + fv.layout.size.height;
                    let hit = px >= l && px <= r && py >= t && py <= b;
                    if let FlatViewKind::Button { on_hover: Some(on_hover), .. } = &fv.kind {
                        on_hover(hit);
                        changed = true;
                    }
                }
                if changed {
                    state.window.request_redraw();
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x * 20.0, y * 20.0),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                let (cx, cy) = state.cursor_pos;
                let view = (self.build_view)(&self.theme);
                let w = state.renderer.surface_cfg.width as f32;
                let h = state.renderer.surface_cfg.height as f32;
                let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                let view2 = (self.build_view)(&self.theme);
                let mut idx = 0;
                dispatch_scroll(&view2, &self.theme, &flat, cx, cy, dx, dy, &mut idx);
                if needs_redraw() {
                    clear_redraw();
                    state.window.request_redraw();
                }
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                let (cx, cy) = state.cursor_pos;
                let view = (self.build_view)(&self.theme);
                let w = state.renderer.surface_cfg.width as f32;
                let h = state.renderer.surface_cfg.height as f32;
                let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());

                for fv in &flat {
                    let l = fv.layout.location.x;
                    let t = fv.layout.location.y;
                    let r = l + fv.layout.size.width;
                    let b = t + fv.layout.size.height;
                    let hit = cx >= l && cx <= r && cy >= t && cy <= b;

                    match &fv.kind {
                        FlatViewKind::Button { on_click, .. } => {
                            if hit { on_click(); }
                        }
                        FlatViewKind::TextInput { focused, .. } => {
                            focused.set(hit);
                            if hit { state.frame = 0; }
                        }
                        _ => {}
                    }
                }

                if needs_redraw() { clear_redraw(); }
                state.window.request_redraw();
            }

            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key, state: ElementState::Pressed, .. },
                ..
            } => {
                let view = (self.build_view)(&self.theme);
                let w = state.renderer.surface_cfg.width as f32;
                let h = state.renderer.surface_cfg.height as f32;
                let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());

                for fv in &flat {
                    if let FlatViewKind::TextInput { value, focused, on_submit, .. } = &fv.kind {
                        if !focused.get() { continue; }
                        let mut s = value.get();
                        match &logical_key {
                            Key::Named(NamedKey::Backspace) => {
                                if let Some((idx, _)) = s.char_indices().next_back() {
                                    s.truncate(idx);
                                    value.set(s);
                                }
                            }
                            Key::Named(NamedKey::Delete) => {
                                value.set(String::new());
                            }
                            Key::Named(NamedKey::Escape) => {
                                focused.set(false);
                            }
                            Key::Named(NamedKey::Tab) => {
                                focused.set(false);
                            }
                            Key::Named(NamedKey::Enter) => {
                                if let Some(f) = on_submit {
                                    f(value.get());
                                }
                                focused.set(false);
                            }
                            Key::Character(ch) => {
                                s.push_str(ch.as_str());
                                value.set(s);
                            }
                            _ => {}
                        }
                    }
                }

                if needs_redraw() { clear_redraw(); }
                state.window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                state.frame = state.frame.wrapping_add(1);
                let cursor_visible = (state.frame / 30) % 2 == 0;

                let view = (self.build_view)(&self.theme);
                let w = state.renderer.surface_cfg.width as f32;
                let h = state.renderer.surface_cfg.height as f32;
                let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());

                let any_focused = flat.iter().any(|fv| {
                    matches!(&fv.kind, FlatViewKind::TextInput { focused, .. } if focused.get())
                });
                if any_focused {
                    state.window.request_redraw();
                }

                state.renderer.render(flat, cursor_visible);
            }

            _ => {}
        }
    }
}

fn dispatch_scroll(
    view: &View,
    theme: &Theme,
    flat: &[FlatView],
    cx: f32,
    cy: f32,
    dx: f32,
    dy: f32,
    flat_idx: &mut usize,
) {
    match view {
        View::Scroll { child, offset_x, offset_y, .. } => {
            if let Some(fv) = flat.get(*flat_idx) {
                if let FlatViewKind::ClipStart { x, y, width, height } = &fv.kind {
                    if cx >= *x && cx <= x + width && cy >= *y && cy <= y + height {
                        offset_x.set((offset_x.get() - dx).max(0.0));
                        offset_y.set((offset_y.get() - dy).max(0.0));
                    }
                }
            }
            *flat_idx += 1;
            dispatch_scroll(child, theme, flat, cx, cy, dx, dy, flat_idx);
            *flat_idx += 1;
        }
        View::Column { children, bg_color, border_color, shadow, clip, .. }
        | View::Row { children, bg_color, border_color, shadow, clip, .. } => {
            if bg_color.is_some() || border_color.is_some() || shadow.is_some() {
                *flat_idx += 1;
            }
            if *clip { *flat_idx += 1; }
            for child in children {
                dispatch_scroll(child, theme, flat, cx, cy, dx, dy, flat_idx);
            }
            if *clip { *flat_idx += 1; }
        }
        View::ZStack { children, .. } => {
            for child in children {
                dispatch_scroll(child, theme, flat, cx, cy, dx, dy, flat_idx);
            }
        }
        View::Component(c) => {
            let rendered = c.render(theme);
            dispatch_scroll(&rendered, theme, flat, cx, cy, dx, dy, flat_idx);
        }
        View::Button { .. } | View::Rect { .. } | View::Text { .. }
        | View::TextInput { .. } | View::Image { .. } => {
            *flat_idx += 1;
        }
        View::Flexible { child, .. } => {
            dispatch_scroll(child, theme, flat, cx, cy, dx, dy, flat_idx);
        }
        View::Spacer => {}
    }
}
