use glyph_core::{FlatView, FlatViewKind, Theme, View, ViewTree, clear_redraw, needs_redraw, tick_tweens};
use glyph_render::{GpuContext, Renderer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

// ---------------------------------------------------------------------------
// WindowOpener — cheaply cloneable handle for opening new windows from closures
// ---------------------------------------------------------------------------

pub struct WindowRequest {
    pub build_view: Box<dyn Fn(&WindowOpener) -> (Theme, View) + Send>,
    pub title:      String,
    pub width:      f64,
    pub height:     f64,
    pub theme:      Theme,
}

/// Clone this into button callbacks to open new windows.
#[derive(Clone)]
pub struct WindowOpener(Arc<Mutex<Vec<WindowRequest>>>);

impl WindowOpener {
    fn new() -> (Self, Arc<Mutex<Vec<WindowRequest>>>) {
        let q = Arc::new(Mutex::new(Vec::new()));
        (Self(Arc::clone(&q)), q)
    }

    pub fn open(
        &self,
        build_view: impl Fn(&WindowOpener) -> (Theme, View) + Send + 'static,
        title: impl Into<String>,
        width: f64,
        height: f64,
        theme: Theme,
    ) {
        self.0.lock().unwrap().push(WindowRequest {
            build_view: Box::new(build_view),
            title: title.into(),
            width,
            height,
            theme,
        });
    }
}

// ---------------------------------------------------------------------------
// Per-window state
// ---------------------------------------------------------------------------

struct WindowState {
    window:     Arc<Window>,
    renderer:   Renderer,
    build_view: Box<dyn Fn(&WindowOpener) -> (Theme, View)>,
    theme:      Theme,
    cursor_pos: (f32, f32), // logical pixels
    frame:      u32,
}

impl WindowState {
    /// Call `build_view`, update `self.theme` from the returned theme, return the view.
    fn build(&mut self, opener: &WindowOpener) -> View {
        let (theme, view) = (self.build_view)(opener);
        self.theme = theme;
        view
    }

    fn scale(&self) -> f32 {
        self.window.scale_factor() as f32
    }

    /// Layout width in logical pixels.
    fn lw(&self) -> f32 {
        self.renderer.surface_cfg.width as f32 / self.scale()
    }

    /// Layout height in logical pixels.
    fn lh(&self) -> f32 {
        self.renderer.surface_cfg.height as f32 / self.scale()
    }
}

// ---------------------------------------------------------------------------
// App — window manager
// ---------------------------------------------------------------------------

pub struct App {
    ctx:       Option<Arc<GpuContext>>,
    windows:   HashMap<WindowId, WindowState>,
    opener:    WindowOpener,
    queue:     Arc<Mutex<Vec<WindowRequest>>>,
    initial:   Option<WindowRequest>,
    last_tick: Option<Instant>,
}

impl App {
    pub fn run(
        build_view: impl Fn(&WindowOpener) -> (Theme, View) + Send + 'static,
        theme: Theme,
        title: impl Into<String>,
        width: f64,
        height: f64,
    ) {
        let (opener, queue) = WindowOpener::new();
        let initial = WindowRequest {
            build_view: Box::new(build_view),
            title: title.into(),
            width,
            height,
            theme,
        };
        let event_loop = EventLoop::new().expect("event loop");
        let mut app = App {
            ctx: None,
            windows: HashMap::new(),
            opener,
            queue,
            initial: Some(initial),
            last_tick: None,
        };
        event_loop.run_app(&mut app).expect("event loop run");
    }

    fn open_window(&mut self, req: WindowRequest, event_loop: &ActiveEventLoop) {
        let ctx = self.ctx.as_ref().expect("GpuContext not initialised").clone();
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&req.title)
                        .with_inner_size(winit::dpi::LogicalSize::new(req.width, req.height)),
                )
                .expect("window"),
        );
        let size = window.inner_size();
        let (surface, surface_cfg) =
            ctx.create_surface(Arc::clone(&window), size.width.max(1), size.height.max(1));
        let renderer = Renderer::new(Arc::clone(&ctx), surface, surface_cfg);
        let id = window.id();
        self.windows.insert(id, WindowState {
            window,
            renderer,
            build_view: req.build_view,
            theme: req.theme,
            cursor_pos: (0.0, 0.0),
            frame: 0,
        });
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.ctx.is_some() {
            return;
        }
        // Create the first window before the GPU context so the adapter can be
        // selected with surface compatibility — required on Windows with multiple GPUs.
        if let Some(req) = self.initial.take() {
            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title(&req.title)
                            .with_inner_size(winit::dpi::LogicalSize::new(req.width, req.height)),
                    )
                    .expect("window"),
            );
            let ctx = pollster::block_on(GpuContext::new_with_window(Arc::clone(&window)));
            self.ctx = Some(Arc::clone(&ctx));
            let size = window.inner_size();
            let (surface, surface_cfg) =
                ctx.create_surface(Arc::clone(&window), size.width.max(1), size.height.max(1));
            let renderer = Renderer::new(ctx, surface, surface_cfg);
            let id = window.id();
            self.windows.insert(id, WindowState {
                window,
                renderer,
                build_view: req.build_view,
                theme: req.theme,
                cursor_pos: (0.0, 0.0),
                frame: 0,
            });
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // Advance tweens.
        let now = Instant::now();
        let dt = self.last_tick.map_or(0.0, |t| now.duration_since(t).as_secs_f32());
        self.last_tick = Some(now);
        if tick_tweens(dt) {
            for ws in self.windows.values() {
                ws.window.request_redraw();
            }
        }

        let pending: Vec<WindowRequest> = self.queue.lock().unwrap().drain(..).collect();
        for req in pending {
            self.open_window(req, event_loop);
        }
        if self.windows.is_empty() && self.ctx.is_some() {
            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let opener = self.opener.clone();

        match event {
            WindowEvent::CloseRequested => {
                self.windows.remove(&id);
                if self.windows.is_empty() {
                    event_loop.exit();
                }
                return;
            }

            WindowEvent::Resized(size) => {
                if let Some(ws) = self.windows.get_mut(&id) {
                    ws.renderer.resize(size.width, size.height);
                    ws.window.request_redraw();
                }
                return;
            }

            WindowEvent::CursorMoved { position, .. } => {
                let Some(ws) = self.windows.get_mut(&id) else { return };
                let scale = ws.scale();
                let (px, py) = (position.x as f32 / scale, position.y as f32 / scale);
                ws.cursor_pos = (px, py);

                let view = ws.build(&opener);
                let (w, h) = (ws.lw(), ws.lh());
                let flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());
                let mut changed = false;
                for fv in &flat {
                    let l = fv.layout.location.x;
                    let t = fv.layout.location.y;
                    let hit = px >= l && px <= l + fv.layout.size.width
                           && py >= t && py <= t + fv.layout.size.height;
                    if let FlatViewKind::Button { on_hover: Some(on_hover), .. } = &fv.kind {
                        on_hover(hit);
                        changed = true;
                    }
                }
                if changed { ws.window.request_redraw(); }
                return;
            }

            WindowEvent::CursorLeft { .. } => {
                let Some(ws) = self.windows.get_mut(&id) else { return };
                let (w, h) = (ws.lw(), ws.lh());
                let view = ws.build(&opener);
                let flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());
                let mut changed = false;
                for fv in &flat {
                    if let FlatViewKind::Button { on_hover: Some(on_hover), .. } = &fv.kind {
                        on_hover(false);
                        changed = true;
                    }
                }
                if changed { ws.window.request_redraw(); }
                return;
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let Some(ws) = self.windows.get_mut(&id) else { return };
                let scale = ws.scale();
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x * 20.0, y * 20.0),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32 / scale, pos.y as f32 / scale),
                };
                let (cx, cy) = ws.cursor_pos;
                let (w, h) = (ws.lw(), ws.lh());
                let view = ws.build(&opener);
                let flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());
                let view2 = ws.build(&opener);
                let mut idx = 0;
                dispatch_scroll(&view2, &ws.theme, &flat, cx, cy, dx, dy, &mut idx);
                if needs_redraw() { clear_redraw(); ws.window.request_redraw(); }
                return;
            }

            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                let Some(ws) = self.windows.get_mut(&id) else { return };
                let (cx, cy) = ws.cursor_pos;
                let (w, h) = (ws.lw(), ws.lh());
                let view = ws.build(&opener);
                let flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());
                let mut clicked = false;
                for fv in &flat {
                    let l = fv.layout.location.x;
                    let t = fv.layout.location.y;
                    let hit = cx >= l && cx <= l + fv.layout.size.width
                           && cy >= t && cy <= t + fv.layout.size.height;
                    match &fv.kind {
                        FlatViewKind::Button { on_click, .. } => {
                            if hit && !clicked {
                                on_click();
                                clicked = true;
                            }
                        }
                        FlatViewKind::TextInput { focused, cursor, value, .. } => {
                            focused.set(hit);
                            if hit {
                                ws.frame = 0;
                                cursor.set(value.get().len());
                            }
                        }
                        _ => {}
                    }
                }
                if needs_redraw() { clear_redraw(); }
                ws.window.request_redraw();
                return;
            }

            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key, state: ElementState::Pressed, .. }, ..
            } => {
                let Some(ws) = self.windows.get_mut(&id) else { return };
                let (w, h) = (ws.lw(), ws.lh());
                let view = ws.build(&opener);
                let flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());

                // Collect text-input indices for Tab cycling.
                let input_indices: Vec<usize> = flat.iter().enumerate()
                    .filter(|(_, fv)| matches!(&fv.kind, FlatViewKind::TextInput { .. }))
                    .map(|(i, _)| i)
                    .collect();

                for (pos, &idx) in input_indices.iter().enumerate() {
                    let FlatViewKind::TextInput { value, focused, cursor, on_change, on_submit, .. } = &flat[idx].kind else { continue };
                    if !focused.get() { continue; }
                    let mut s = value.get();
                    let mut cur = cursor.get().min(s.len());
                    let mut changed = false;
                    match &logical_key {
                        Key::Named(NamedKey::Backspace) => {
                            if cur > 0 {
                                // find the char boundary before cur
                                let prev = s[..cur].char_indices().next_back().map(|(i, _)| i).unwrap_or(0);
                                s.remove(prev);
                                cur = prev;
                                value.set(s.clone());
                                cursor.set(cur);
                                changed = true;
                            }
                        }
                        Key::Named(NamedKey::Delete) => {
                            if cur < s.len() {
                                s.remove(cur);
                                value.set(s.clone());
                                changed = true;
                            }
                        }
                        Key::Named(NamedKey::ArrowLeft) => {
                            if cur > 0 {
                                let prev = s[..cur].char_indices().next_back().map(|(i, _)| i).unwrap_or(0);
                                cursor.set(prev);
                            }
                        }
                        Key::Named(NamedKey::ArrowRight) => {
                            if cur < s.len() {
                                let next = s[cur..].char_indices().nth(1).map(|(i, _)| cur + i).unwrap_or(s.len());
                                cursor.set(next);
                            }
                        }
                        Key::Named(NamedKey::Space) => {
                            s.insert(cur, ' ');
                            cur += 1;
                            value.set(s.clone());
                            cursor.set(cur);
                            changed = true;
                        }
                        Key::Named(NamedKey::Home) => { cursor.set(0); }
                        Key::Named(NamedKey::End) => { cursor.set(s.len()); }
                        Key::Named(NamedKey::Escape) => { focused.set(false); }
                        Key::Named(NamedKey::Tab) => {
                            focused.set(false);
                            let next = input_indices[(pos + 1) % input_indices.len()];
                            if let FlatViewKind::TextInput { focused: nf, .. } = &flat[next].kind {
                                nf.set(true);
                            }
                        }
                        Key::Named(NamedKey::Enter) => {
                            if let Some(f) = on_submit { f(value.get()); }
                            focused.set(false);
                        }
                        Key::Character(ch) => {
                            s.insert_str(cur, ch.as_str());
                            cur += ch.len();
                            value.set(s.clone());
                            cursor.set(cur);
                            changed = true;
                        }
                        _ => {}
                    }
                    if changed {
                        if let Some(f) = on_change { f(s); }
                    }
                    break;
                }
                if needs_redraw() { clear_redraw(); }
                ws.window.request_redraw();
                return;
            }

            WindowEvent::RedrawRequested => {
                let Some(ws) = self.windows.get_mut(&id) else { return };
                ws.frame = ws.frame.wrapping_add(1);
                let cursor_visible = (ws.frame / 30) % 2 == 0;
                let scale = ws.scale();
                let (w, h) = (ws.lw(), ws.lh());
                let view = ws.build(&opener);
                let flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());
                let any_focused = flat.iter().any(|fv| {
                    matches!(&fv.kind, FlatViewKind::TextInput { focused, .. } if focused.get())
                });
                if any_focused { ws.window.request_redraw(); }
                let flat = scale_flat(flat, scale);
                ws.renderer.render(flat, cursor_visible, ws.theme.background);
                return;
            }

            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// DPI scaling: layout runs in logical pixels; renderer works in physical pixels.
// Scale every position, size, and font size in the flat list before rendering.
// ---------------------------------------------------------------------------

fn scale_flat(flat: Vec<FlatView>, scale: f32) -> Vec<FlatView> {
    if (scale - 1.0).abs() < f32::EPSILON { return flat; }
    flat.into_iter().map(|fv| {
        let l = &fv.layout;
        let mut layout = *l;
        layout.location.x *= scale;
        layout.location.y *= scale;
        layout.size.width  *= scale;
        layout.size.height *= scale;
        let kind = match fv.kind {
            FlatViewKind::Text { content, font_size, color, weight, align, wrap } =>
                FlatViewKind::Text { content, font_size: font_size * scale, color, weight, align, wrap },
            FlatViewKind::Button { label, on_click, on_hover, bg_color, hover_bg_color, text_color, corner_radius, font_size } =>
                FlatViewKind::Button { label, on_click, on_hover, bg_color, hover_bg_color, text_color, corner_radius: corner_radius * scale, font_size: font_size * scale },
            FlatViewKind::TextInput { value, focused, cursor, placeholder, font_size, bg_color, text_color, border_color, corner_radius, on_change, on_submit } =>
                FlatViewKind::TextInput { value, focused, cursor, placeholder, font_size: font_size * scale, bg_color, text_color, border_color, corner_radius: corner_radius * scale, on_change, on_submit },
            FlatViewKind::ContainerRect { bg_color, border_color, border_width, corner_radius, shadow } => {
                let shadow = shadow.map(|s| glyph_core::Shadow {
                    offset_x: s.offset_x * scale,
                    offset_y: s.offset_y * scale,
                    blur: s.blur * scale,
                    color: s.color,
                });
                FlatViewKind::ContainerRect { bg_color, border_color, border_width: border_width * scale, corner_radius: corner_radius * scale, shadow }
            }
            FlatViewKind::ClipStart { x, y, width, height } =>
                FlatViewKind::ClipStart { x: x * scale, y: y * scale, width: width * scale, height: height * scale },
            FlatViewKind::Image { path, corner_radius } =>
                FlatViewKind::Image { path, corner_radius: corner_radius * scale },
            other => other,
        };
        FlatView { kind, layout }
    }).collect()
}

// ---------------------------------------------------------------------------
// Scroll dispatch (unchanged logic, moved out of impl)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// HotApp (hot-reload variant, unchanged from before)
// ---------------------------------------------------------------------------

#[cfg(feature = "hot-reload")]
pub struct HotApp {
    loader: glyph_hot::HotLoader,
    theme: Theme,
    title: String,
    width: f64,
    height: f64,
    state: Option<HotAppState>,
}

#[cfg(feature = "hot-reload")]
struct HotAppState {
    window:     Arc<Window>,
    renderer:   Renderer,
    cursor_pos: (f32, f32),
    frame:      u32,
}

#[cfg(feature = "hot-reload")]
impl HotApp {
    pub fn run(
        src_dir: impl AsRef<std::path::Path>,
        lib_path: impl AsRef<std::path::Path>,
        package_name: &str,
        theme: Theme,
        title: impl Into<String>,
        width: f64,
        height: f64,
    ) {
        let loader = glyph_hot::HotLoader::new(src_dir.as_ref(), lib_path.as_ref(), package_name);
        let event_loop = EventLoop::new().expect("event loop");
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let mut app = HotApp { loader, theme, title: title.into(), width, height, state: None };
        event_loop.run_app(&mut app).expect("event loop run");
    }
}

#[cfg(feature = "hot-reload")]
impl ApplicationHandler for HotApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() { return; }
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
        let ctx = pollster::block_on(GpuContext::new_with_window(Arc::clone(&window)));
        let (surface, surface_cfg) = ctx.create_surface(
            Arc::clone(&window), size.width.max(1), size.height.max(1),
        );
        let renderer = Renderer::new(ctx, surface, surface_cfg);
        self.state = Some(HotAppState { window, renderer, cursor_pos: (0.0, 0.0), frame: 0 });
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(state) = &mut self.state else { return };
        if self.loader.poll_reload() {
            state.window.request_redraw();
        }
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
                let scale = state.window.scale_factor() as f32;
                let (px, py) = (position.x as f32 / scale, position.y as f32 / scale);
                state.cursor_pos = (px, py);
                let w = state.renderer.surface_cfg.width as f32 / scale;
                let h = state.renderer.surface_cfg.height as f32 / scale;
                if let Some(view) = self.loader.build_view(&self.theme) {
                    let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    let mut changed = false;
                    for fv in &flat {
                        let l = fv.layout.location.x;
                        let t = fv.layout.location.y;
                        let hit = px >= l && px <= l + fv.layout.size.width
                               && py >= t && py <= t + fv.layout.size.height;
                        if let FlatViewKind::Button { on_hover: Some(on_hover), .. } = &fv.kind {
                            on_hover(hit);
                            changed = true;
                        }
                    }
                    if changed { state.window.request_redraw(); }
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x * 20.0, y * 20.0),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                let (cx, cy) = state.cursor_pos;
                let scale = state.window.scale_factor() as f32; let w = state.renderer.surface_cfg.width as f32 / scale;
                let h = state.renderer.surface_cfg.height as f32 / scale;
                if let Some(view) = self.loader.build_view(&self.theme) {
                    let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    if let Some(view2) = self.loader.build_view(&self.theme) {
                        let mut idx = 0;
                        dispatch_scroll(&view2, &self.theme, &flat, cx, cy, dx, dy, &mut idx);
                    }
                }
                if needs_redraw() { clear_redraw(); state.window.request_redraw(); }
            }

            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                let (cx, cy) = state.cursor_pos;
                let scale = state.window.scale_factor() as f32; let w = state.renderer.surface_cfg.width as f32 / scale;
                let h = state.renderer.surface_cfg.height as f32 / scale;
                if let Some(view) = self.loader.build_view(&self.theme) {
                    let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    for fv in &flat {
                        let l = fv.layout.location.x;
                        let t = fv.layout.location.y;
                        let hit = cx >= l && cx <= l + fv.layout.size.width
                               && cy >= t && cy <= t + fv.layout.size.height;
                        match &fv.kind {
                            FlatViewKind::Button { on_click, .. } => { if hit { on_click(); } }
                            FlatViewKind::TextInput { focused, .. } => {
                                focused.set(hit);
                                if hit { state.frame = 0; }
                            }
                            _ => {}
                        }
                    }
                }
                if needs_redraw() { clear_redraw(); }
                state.window.request_redraw();
            }

            WindowEvent::KeyboardInput { event: KeyEvent { logical_key, state: ElementState::Pressed, .. }, .. } => {
                let scale = state.window.scale_factor() as f32; let w = state.renderer.surface_cfg.width as f32 / scale;
                let h = state.renderer.surface_cfg.height as f32 / scale;
                if let Some(view) = self.loader.build_view(&self.theme) {
                    let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    for fv in &flat {
                        if let FlatViewKind::TextInput { value, focused, on_submit, .. } = &fv.kind {
                            if !focused.get() { continue; }
                            let mut s = value.get();
                            match &logical_key {
                                Key::Named(NamedKey::Backspace) => {
                                    if let Some((idx, _)) = s.char_indices().next_back() { s.truncate(idx); value.set(s); }
                                }
                                Key::Named(NamedKey::Delete)  => { value.set(String::new()); }
                                Key::Named(NamedKey::Escape)  => { focused.set(false); }
                                Key::Named(NamedKey::Tab)     => { focused.set(false); }
                                Key::Named(NamedKey::Enter)   => {
                                    if let Some(f) = on_submit { f(value.get()); }
                                    focused.set(false);
                                }
                                Key::Character(ch) => { s.push_str(ch.as_str()); value.set(s); }
                                _ => {}
                            }
                        }
                    }
                }
                if needs_redraw() { clear_redraw(); }
                state.window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                state.frame = state.frame.wrapping_add(1);
                let cursor_visible = (state.frame / 30) % 2 == 0;
                let scale = state.window.scale_factor() as f32; let w = state.renderer.surface_cfg.width as f32 / scale;
                let h = state.renderer.surface_cfg.height as f32 / scale;
                if let Some(view) = self.loader.build_view(&self.theme) {
                    let flat = ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    let any_focused = flat.iter().any(|fv| {
                        matches!(&fv.kind, FlatViewKind::TextInput { focused, .. } if focused.get())
                    });
                    if any_focused { state.window.request_redraw(); }
                    let flat = scale_flat(flat, scale);
                    state.renderer.render(flat, cursor_visible, self.theme.background);
                }
            }

            _ => {}
        }
    }
}
