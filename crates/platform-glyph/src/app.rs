use core_glyph::{
    clear_redraw, needs_redraw, tick_tweens, FlatView, FlatViewKind, Signal, Theme, View, ViewTree,
};
use std::path::PathBuf;
use render_glyph::{GpuContext, Renderer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, ModifiersState, NamedKey},
    window::{Cursor, CursorIcon, Window, WindowId},
};

// ---------------------------------------------------------------------------
// WindowOpener — cheaply cloneable handle for opening new windows from closures
// WindowCloser — cheaply cloneable handle for closing the current window
// ---------------------------------------------------------------------------

type BuildViewFn = Box<dyn Fn(&WindowOpener, &WindowCloser) -> (Theme, View) + Send>;

pub struct WindowRequest {
    pub build_view: BuildViewFn,
    pub title: String,
    pub width: f64,
    pub height: f64,
    pub theme: Theme,
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
        build_view: impl Fn(&WindowOpener, &WindowCloser) -> (Theme, View) + Send + 'static,
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

/// Clone this into button callbacks to close the window it was created for.
///
/// The closer is bound to a specific window: calling `close()` queues
/// that window for removal at the next event-loop tick.
#[derive(Clone)]
pub struct WindowCloser {
    /// The id of the window this closer is bound to.  Populated by
    /// `open_window` immediately after the OS window is created.
    id: Arc<Mutex<Option<WindowId>>>,
    /// Shared queue on `App` — draining this closes windows.
    queue: Arc<Mutex<Vec<WindowId>>>,
}

impl WindowCloser {
    fn new(queue: Arc<Mutex<Vec<WindowId>>>) -> Self {
        Self {
            id: Arc::new(Mutex::new(None)),
            queue,
        }
    }

    fn set_id(&self, id: WindowId) {
        *self.id.lock().unwrap() = Some(id);
    }

    /// Request that this window be closed at the next event loop tick.
    pub fn close(&self) {
        if let Some(id) = *self.id.lock().unwrap() {
            self.queue.lock().unwrap().push(id);
        }
    }
}

// ---------------------------------------------------------------------------
// Per-window state
// ---------------------------------------------------------------------------

/// Lightweight cursor/hover info extracted from the flat list after each redraw.
struct HitItem {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    kind: HitKind,
}

enum HitKind {
    Button(bool),
    Text,
}

#[derive(Default)]
struct TextEditState {
    focused_flat_index: Option<usize>,
    selection_anchor: Option<usize>,
    selection: Option<(usize, usize)>,
    composing: Option<(usize, String)>,
}

/// A scrollable region extracted from the flat list, used for momentum scrolling.
struct ScrollItem {
    /// Content-space position of this scroll region.
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    offset_x: Signal<f32>,
    offset_y: Signal<f32>,
    max_x: f32,
    max_y: f32,
    /// Signals for each enclosing scroll region, in order from outermost to innermost.
    /// Used to compute the live screen-space position: content_pos - sum(enclosing offsets).
    enclosing: Vec<(Signal<f32>, Signal<f32>)>,
}

struct WindowState {
    window: Arc<Window>,
    renderer: Renderer,
    build_view: BuildViewFn,
    closer: WindowCloser,
    theme: Theme,
    cursor_pos: (f32, f32),
    frame: u32,
    hit_items: Vec<HitItem>,
    scroll_items: Vec<ScrollItem>,
    scroll_vx: f32,
    scroll_vy: f32,
    last_scroll: Option<Instant>,
    flat_cache: Vec<FlatView>,
    scaled_cache: Vec<FlatView>,
    modifiers: ModifiersState,
    text_edit: TextEditState,
    /// Set when a scroll event arrives; cleared after each redraw. When true and
    /// no VirtualList row range changed, we skip ViewTree::build and re-render
    /// scaled_cache directly (scroll offsets are read live from signals by the renderer).
    scroll_dirty: bool,
    /// Cached VirtualList row ranges: (offset_y, first_row, last_row) per list.
    vlist_ranges: Vec<f32>,
}

impl WindowState {
    /// Call `build_view`, update `self.theme` from the returned theme, return the view.
    fn build(&mut self, opener: &WindowOpener) -> View {
        let (theme, view) = (self.build_view)(opener, &self.closer);
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
    ctx: Option<Arc<GpuContext>>,
    windows: HashMap<WindowId, WindowState>,
    opener: WindowOpener,
    queue: Arc<Mutex<Vec<WindowRequest>>>,
    /// Windows requested to close via `WindowCloser::close()`.
    pending_close: Arc<Mutex<Vec<WindowId>>>,
    initial: Option<WindowRequest>,
    last_tick: Option<Instant>,
    pending_fonts: Vec<Vec<u8>>,
    pending_font_files: Vec<PathBuf>,
}

/// Builder for configuring an `App` before running it.
pub struct AppBuilder {
    build_view: Box<dyn Fn(&WindowOpener, &WindowCloser) -> (Theme, View) + Send + 'static>,
    theme: Theme,
    title: String,
    width: f64,
    height: f64,
    fonts: Vec<Vec<u8>>,
    font_files: Vec<PathBuf>,
}

impl AppBuilder {
    /// Add a font from raw bytes. The font's family name (embedded in the font
    /// file) becomes available as `FontFamily::Name("...")`.
    pub fn font(mut self, data: Vec<u8>) -> Self {
        self.fonts.push(data);
        self
    }

    /// Add a font from a file path.
    pub fn font_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.font_files.push(path.into());
        self
    }

    pub fn run(self) {
        let (opener, queue) = WindowOpener::new();
        let pending_close = Arc::new(Mutex::new(Vec::<WindowId>::new()));
        let initial = WindowRequest {
            build_view: self.build_view,
            title: self.title,
            width: self.width,
            height: self.height,
            theme: self.theme,
        };
        let event_loop = EventLoop::new().expect("event loop");
        let mut app = App {
            ctx: None,
            windows: HashMap::new(),
            opener,
            queue,
            pending_close,
            initial: Some(initial),
            last_tick: None,
            pending_fonts: self.fonts,
            pending_font_files: self.font_files,
        };
        event_loop.run_app(&mut app).expect("event loop run");
    }
}

impl App {
    /// Quick-start: single window, no font customization.
    pub fn run(
        build_view: impl Fn(&WindowOpener, &WindowCloser) -> (Theme, View) + Send + 'static,
        theme: Theme,
        title: impl Into<String>,
        width: f64,
        height: f64,
    ) {
        App::builder(build_view, theme, title, width, height).run();
    }

    /// Returns a builder for registering custom fonts before the event loop starts.
    pub fn builder(
        build_view: impl Fn(&WindowOpener, &WindowCloser) -> (Theme, View) + Send + 'static,
        theme: Theme,
        title: impl Into<String>,
        width: f64,
        height: f64,
    ) -> AppBuilder {
        AppBuilder {
            build_view: Box::new(build_view),
            theme,
            title: title.into(),
            width,
            height,
            fonts: Vec::new(),
            font_files: Vec::new(),
        }
    }

    fn apply_pending_fonts(&self, renderer: &mut Renderer) {
        for data in &self.pending_fonts {
            renderer.load_font(data.clone());
        }
        for path in &self.pending_font_files {
            if let Err(e) = renderer.load_font_file(path) {
                eprintln!("glyph: failed to load font {:?}: {}", path, e);
            }
        }
    }

    fn open_window(&mut self, req: WindowRequest, event_loop: &ActiveEventLoop) {
        let ctx = self
            .ctx
            .as_ref()
            .expect("GpuContext not initialised")
            .clone();
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
        let mut renderer = Renderer::new(Arc::clone(&ctx), surface, surface_cfg);
        self.apply_pending_fonts(&mut renderer);
        let id = window.id();
        let closer = WindowCloser::new(Arc::clone(&self.pending_close));
        closer.set_id(id);
        self.windows.insert(
            id,
            WindowState {
                window,
                renderer,
                build_view: req.build_view,
                closer,
                theme: req.theme,
                cursor_pos: (0.0, 0.0),
                frame: 0,
                hit_items: Vec::new(),
                scroll_items: Vec::new(),
                scroll_vx: 0.0,
                scroll_vy: 0.0,
                last_scroll: None,
                flat_cache: Vec::new(),
                scaled_cache: Vec::new(),
                modifiers: ModifiersState::empty(),
                text_edit: TextEditState::default(),
                scroll_dirty: false,
                vlist_ranges: Vec::new(),
            },
        );
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
            let mut renderer = Renderer::new(ctx, surface, surface_cfg);
            self.apply_pending_fonts(&mut renderer);
            let id = window.id();
            let closer = WindowCloser::new(Arc::clone(&self.pending_close));
            closer.set_id(id);
            self.windows.insert(
                id,
                WindowState {
                    window,
                    renderer,
                    build_view: req.build_view,
                    closer,
                    theme: req.theme,
                    cursor_pos: (0.0, 0.0),
                    frame: 0,
                    hit_items: Vec::new(),
                    scroll_items: Vec::new(),
                    scroll_vx: 0.0,
                    scroll_vy: 0.0,
                    last_scroll: None,
                    flat_cache: Vec::new(),
                    scaled_cache: Vec::new(),
                    modifiers: ModifiersState::empty(),
                    text_edit: TextEditState::default(),
                    scroll_dirty: false,
                    vlist_ranges: Vec::new(),
                },
            );
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        let dt = self
            .last_tick
            .map_or(0.0, |t| now.duration_since(t).as_secs_f32())
            .min(0.05); // cap dt so a stalled frame doesn't cause a huge jump
        self.last_tick = Some(now);

        if tick_tweens(dt) {
            for ws in self.windows.values() {
                ws.window.request_redraw();
            }
        }

        // Apply mouse-wheel momentum (LineDelta only — trackpad uses native macOS momentum).
        // Exponential decay with tau=350ms: decay = exp(-dt/0.35)
        let mut any_scrolling = false;
        for ws in self.windows.values_mut() {
            let speed = ws.scroll_vx.abs().max(ws.scroll_vy.abs());
            if speed > 1.0 {
                any_scrolling = true;
                let (cur_x, cur_y) = ws.cursor_pos;
                let only_scroll = !needs_redraw();
                let dx = ws.scroll_vx * dt;
                let dy = ws.scroll_vy * dt;
                apply_scroll(&ws.scroll_items, cur_x, cur_y, dx, dy);
                let decay = (-dt / 0.35).exp();
                ws.scroll_vx *= decay;
                ws.scroll_vy *= decay;
                clear_redraw();
                if only_scroll {
                    ws.scroll_dirty = true;
                }
                ws.window.request_redraw();
            } else {
                ws.scroll_vx = 0.0;
                ws.scroll_vy = 0.0;
            }
        }

        // Stay in Poll mode while scrolling or for 100ms after the last scroll event.
        // This ensures we render at display rate during trackpad momentum (which macOS
        // delivers as continued PixelDelta events) rather than at event-delivery rate.
        let recently_scrolled = self.windows.values().any(|ws| {
            ws.last_scroll
                .is_some_and(|t| now.duration_since(t).as_millis() < 100)
        });
        if any_scrolling || recently_scrolled {
            for ws in self.windows.values() {
                ws.window.request_redraw();
            }
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        } else {
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        }

        let pending: Vec<WindowRequest> = self.queue.lock().unwrap().drain(..).collect();
        for req in pending {
            self.open_window(req, event_loop);
        }

        // Process window-close requests from WindowCloser::close().
        let to_close: Vec<WindowId> = self.pending_close.lock().unwrap().drain(..).collect();
        for id in to_close {
            self.windows.remove(&id);
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
            }

            WindowEvent::Resized(size) => {
                if let Some(ws) = self.windows.get_mut(&id) {
                    ws.renderer.resize(size.width, size.height);
                    ws.window.request_redraw();
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                let Some(ws) = self.windows.get_mut(&id) else {
                    return;
                };
                let scale = ws.scale();
                let (px, py) = (position.x as f32 / scale, position.y as f32 / scale);
                ws.cursor_pos = (px, py);

                // Fast path: check if cursor is near any interactive element.
                // Only do a full rebuild (for on_hover callbacks) if needed.
                let mut icon = CursorIcon::Default;
                let mut needs_rebuild = false;
                for item in &ws.hit_items {
                    let hit = px >= item.x
                        && px <= item.x + item.w
                        && py >= item.y
                        && py <= item.y + item.h;
                    match &item.kind {
                        HitKind::Button(has_hover) => {
                            if hit {
                                icon = CursorIcon::Pointer;
                            }
                            if *has_hover {
                                needs_rebuild = true;
                            }
                        }
                        HitKind::Text => {
                            if hit {
                                icon = CursorIcon::Text;
                            }
                        }
                    }
                }
                ws.window.set_cursor(Cursor::Icon(icon));

                if needs_rebuild {
                    // Use flat_cache to fire hover callbacks without a full ViewTree rebuild.
                    // flat_cache positions are content-space; subtract current scroll offsets.
                    let mut changed = false;
                    let mut scroll_stack: Vec<(f32, f32)> = Vec::new();
                    let mut pending_scroll: Option<(f32, f32)> = None;
                    for fv in &ws.flat_cache {
                        match &fv.kind {
                            FlatViewKind::ScrollRegion { offset_x, offset_y, .. } => {
                                pending_scroll = Some((offset_x.get(), offset_y.get()));
                                continue;
                            }
                            FlatViewKind::ClipStart { .. } => {
                                scroll_stack.push(pending_scroll.take().unwrap_or((0.0, 0.0)));
                                continue;
                            }
                            FlatViewKind::ClipEnd => { scroll_stack.pop(); continue; }
                            FlatViewKind::OpacityStart { .. } | FlatViewKind::OpacityEnd => continue,
                            _ => {}
                        }
                        let sox: f32 = scroll_stack.iter().map(|(ox, _)| ox).sum();
                        let soy: f32 = scroll_stack.iter().map(|(_, oy)| oy).sum();
                        let l = fv.layout.location.x - sox;
                        let t = fv.layout.location.y - soy;
                        let hit = px >= l
                            && px <= l + fv.layout.size.width
                            && py >= t
                            && py <= t + fv.layout.size.height;
                        if let FlatViewKind::Button {
                            on_hover: Some(on_hover),
                            ..
                        } = &fv.kind
                        {
                            on_hover(hit);
                            changed = true;
                        }
                    }
                    // Clear the redraw flag set by hover signals so scroll frames
                    // can still use the fast path after hover processing.
                    clear_redraw();
                    if changed {
                        ws.window.request_redraw();
                    }
                }
            }

            WindowEvent::CursorLeft { .. } => {
                let Some(ws) = self.windows.get_mut(&id) else {
                    return;
                };
                ws.window.set_cursor(Cursor::Icon(CursorIcon::Default));
                let has_hover_btns = ws
                    .hit_items
                    .iter()
                    .any(|i| matches!(&i.kind, HitKind::Button(true)));
                if has_hover_btns {
                    let mut changed = false;
                    for fv in &ws.flat_cache {
                        if let FlatViewKind::Button {
                            on_hover: Some(on_hover),
                            ..
                        } = &fv.kind
                        {
                            on_hover(false);
                            changed = true;
                        }
                    }
                    clear_redraw();
                    if changed {
                        ws.window.request_redraw();
                    }
                }
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                let Some(ws) = self.windows.get_mut(&id) else {
                    return;
                };
                ws.modifiers = modifiers.state();
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let Some(ws) = self.windows.get_mut(&id) else {
                    return;
                };
                let (cur_x, cur_y) = ws.cursor_pos;
                // Only mark as a pure scroll frame (fast-path eligible) if no other
                // signals changed before this event (e.g. tweens, button clicks).
                let only_scroll = !needs_redraw();
                match delta {
                    // Trackpad: macOS delivers PixelDelta in logical points (not device pixels),
                    // and includes its own momentum phase after finger lift. Apply directly.
                    MouseScrollDelta::PixelDelta(pos) => {
                        let dx = pos.x as f32;
                        let dy = pos.y as f32;
                        apply_scroll(&ws.scroll_items, cur_x, cur_y, dx, dy);
                        ws.scroll_vx = 0.0;
                        ws.scroll_vy = 0.0;
                    }
                    // Mouse wheel: discrete line steps — add our own momentum.
                    MouseScrollDelta::LineDelta(x, y) => {
                        let dx = x * 40.0;
                        let dy = y * 40.0;
                        apply_scroll(&ws.scroll_items, cur_x, cur_y, dx, dy);
                        ws.scroll_vx = ws.scroll_vx * 0.8 + dx * 6.0;
                        ws.scroll_vy = ws.scroll_vy * 0.8 + dy * 6.0;
                    }
                }
                // apply_scroll set needs_redraw; clear it since we'll redraw anyway.
                // Only mark scroll_dirty if no other signals were dirty beforehand.
                clear_redraw();
                ws.last_scroll = Some(Instant::now());
                if only_scroll {
                    ws.scroll_dirty = true;
                }
                ws.window.request_redraw();
            }

            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                let Some(ws) = self.windows.get_mut(&id) else {
                    return;
                };
                let (cx, cy) = ws.cursor_pos;
                let (w, h) = (ws.lw(), ws.lh());
                let view = ws.build(&opener);
                // Stay in logical pixels. Layout positions are content-space; subtract
                // the current scroll offset (tracked via scroll_stack) to get screen coords.
                let flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());
                let pressed = state == ElementState::Pressed;
                let mut clicked = false;
                let mut hit_text_input = false;
                let mut scroll_stack: Vec<(f32, f32)> = Vec::new();
                let mut pending_scroll: Option<(f32, f32)> = None;
                for (idx, fv) in flat.iter().enumerate() {
                    // Track scroll regions so we can offset hit rects correctly.
                    match &fv.kind {
                        FlatViewKind::ScrollRegion { offset_x, offset_y, .. } => {
                            pending_scroll = Some((offset_x.get(), offset_y.get()));
                            continue;
                        }
                        FlatViewKind::ClipStart { .. } => {
                            scroll_stack.push(pending_scroll.take().unwrap_or((0.0, 0.0)));
                            continue;
                        }
                        FlatViewKind::ClipEnd => {
                            scroll_stack.pop();
                            continue;
                        }
                        FlatViewKind::OpacityStart { .. } | FlatViewKind::OpacityEnd => continue,
                        _ => {}
                    }
                    let sox: f32 = scroll_stack.iter().map(|(ox, _)| ox).sum();
                    let soy: f32 = scroll_stack.iter().map(|(_, oy)| oy).sum();
                    let l = fv.layout.location.x - sox;
                    let t = fv.layout.location.y - soy;
                    let hit = cx >= l
                        && cx <= l + fv.layout.size.width
                        && cy >= t
                        && cy <= t + fv.layout.size.height;
                    match &fv.kind {
                        FlatViewKind::Button {
                            on_click, on_press, ..
                        } => {
                            if hit {
                                if let Some(op) = on_press {
                                    op(pressed);
                                }
                                if pressed && !clicked {
                                    on_click();
                                    clicked = true;
                                }
                            } else if !pressed {
                                if let Some(op) = on_press {
                                    op(false);
                                }
                            }
                        }
                        FlatViewKind::TextInput {
                            focused,
                            cursor,
                            value,
                            font_size,
                            ..
                        } if pressed => {
                            focused.set(hit);
                            ws.window.set_ime_allowed(hit);
                            if hit {
                                hit_text_input = true;
                                ws.frame = 0;
                                let val = value.get();
                                let pad = 8.0;
                                let click_offset = (cx - l - pad).max(0.0);
                                let byte_idx = ws.renderer.cursor_for_x(
                                    &val,
                                    *font_size,
                                    click_offset,
                                );
                                cursor.set(byte_idx);
                                ws.text_edit.focused_flat_index = Some(idx);
                                ws.text_edit.composing = None;
                                if ws.modifiers.shift_key() {
                                    let anchor =
                                        ws.text_edit.selection_anchor.unwrap_or(cursor.get());
                                    ws.text_edit.selection_anchor = Some(anchor);
                                    ws.text_edit.selection =
                                        normalized_selection(anchor, byte_idx);
                                } else {
                                    ws.text_edit.selection_anchor = None;
                                    ws.text_edit.selection = None;
                                }
                            }
                        }
                        FlatViewKind::TextArea {
                            focused,
                            cursor,
                            value,
                            font_size,
                            scroll_y,
                            ..
                        } if pressed => {
                            focused.set(hit);
                            if hit {
                                ws.frame = 0;
                                let val = value.get();
                                let line_height = font_size * 1.4;
                                let pad = 8.0;
                                let rel_y = cy - t - pad + scroll_y.get();
                                let line_idx = (rel_y / line_height).floor().max(0.0) as usize;
                                let lines: Vec<&str> = val.split('\n').collect();
                                let line_idx = line_idx.min(lines.len().saturating_sub(1));
                                let mut byte_offset: usize =
                                    lines[..line_idx].iter().map(|l| l.len() + 1).sum();
                                let rel_x = (cx - l - pad).max(0.0);
                                let line_cursor = ws.renderer.cursor_for_x(
                                    lines[line_idx],
                                    *font_size,
                                    rel_x,
                                );
                                byte_offset += line_cursor;
                                cursor.set(byte_offset.min(val.len()));
                            }
                        }
                        _ => {}
                    }
                }
                if pressed && !hit_text_input {
                    ws.text_edit = TextEditState::default();
                    ws.window.set_ime_allowed(false);
                }
                if needs_redraw() {
                    clear_redraw();
                }
                ws.window.request_redraw();
            }

            WindowEvent::Ime(ime) => {
                let Some(ws) = self.windows.get_mut(&id) else {
                    return;
                };
                let (w, h) = (ws.lw(), ws.lh());
                let view = ws.build(&opener);
                let flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());
                let Some(idx) = ws.text_edit.focused_flat_index else {
                    return;
                };
                let Some(fv) = flat.get(idx) else {
                    return;
                };
                if let FlatViewKind::TextInput {
                    value,
                    focused,
                    cursor,
                    on_change,
                    ..
                } = &fv.kind
                {
                    if !focused.get() {
                        return;
                    }
                    match ime {
                        winit::event::Ime::Preedit(text, _) => {
                            let mut s = value.get();
                            let mut cur = cursor.get().min(s.len());
                            if !text.is_empty() && ws.text_edit.composing.is_none()
                                && delete_selection(&mut s, &mut cur, ws.text_edit.selection) {
                                    value.set(s);
                                    cursor.set(cur);
                                    ws.text_edit.selection = None;
                                    ws.text_edit.selection_anchor = None;
                                }
                            ws.text_edit.composing = if text.is_empty() {
                                None
                            } else {
                                Some((cur, text))
                            };
                            ws.window.request_redraw();
                        }
                        winit::event::Ime::Commit(text) => {
                            let mut s = value.get();
                            let mut cur = cursor.get().min(s.len());
                            delete_selection(&mut s, &mut cur, ws.text_edit.selection);
                            s.insert_str(cur, &text);
                            cur += text.len();
                            value.set(s.clone());
                            cursor.set(cur);
                            ws.text_edit.selection = None;
                            ws.text_edit.selection_anchor = None;
                            ws.text_edit.composing = None;
                            if let Some(f) = on_change {
                                f(s);
                            }
                            if needs_redraw() {
                                clear_redraw();
                            }
                            ws.window.request_redraw();
                        }
                        winit::event::Ime::Enabled => {}
                        winit::event::Ime::Disabled => {
                            ws.text_edit.composing = None;
                            ws.window.request_redraw();
                        }
                    }
                }
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                let Some(ws) = self.windows.get_mut(&id) else {
                    return;
                };
                let (w, h) = (ws.lw(), ws.lh());
                let view = ws.build(&opener);
                let flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());

                // Collect text-input and text-area indices for Tab cycling.
                let input_indices: Vec<usize> = flat
                    .iter()
                    .enumerate()
                    .filter(|(_, fv)| {
                        matches!(
                            &fv.kind,
                            FlatViewKind::TextInput { .. } | FlatViewKind::TextArea { .. }
                        )
                    })
                    .map(|(i, _)| i)
                    .collect();

                // Check if any TextInput is focused.
                let mut handled = false;
                for (pos, &idx) in input_indices.iter().enumerate() {
                    match &flat[idx].kind {
                        FlatViewKind::TextInput {
                            value,
                            focused,
                            cursor,
                            on_change,
                            on_submit,
                            ..
                        } => {
                            if !focused.get() {
                                continue;
                            }
                            ws.text_edit.focused_flat_index = Some(idx);
                            let mut s = value.get();
                            let mut cur = cursor.get().min(s.len());
                            let mut changed = false;
                            let command = ws.modifiers.super_key() || ws.modifiers.control_key();
                            match &logical_key {
                                Key::Character(ch)
                                    if command && ch.as_str().eq_ignore_ascii_case("a") =>
                                {
                                    ws.text_edit.selection_anchor = Some(0);
                                    ws.text_edit.selection = normalized_selection(0, s.len());
                                    cursor.set(s.len());
                                }
                                Key::Named(NamedKey::Backspace) => {
                                    if delete_selection(&mut s, &mut cur, ws.text_edit.selection) {
                                        value.set(s.clone());
                                        cursor.set(cur);
                                        ws.text_edit.selection = None;
                                        ws.text_edit.selection_anchor = None;
                                        changed = true;
                                    } else if cur > 0 {
                                        let prev = s[..cur]
                                            .char_indices()
                                            .next_back()
                                            .map(|(i, _)| i)
                                            .unwrap_or(0);
                                        s.remove(prev);
                                        cur = prev;
                                        value.set(s.clone());
                                        cursor.set(cur);
                                        changed = true;
                                    }
                                }
                                Key::Named(NamedKey::Delete) => {
                                    if delete_selection(&mut s, &mut cur, ws.text_edit.selection) {
                                        value.set(s.clone());
                                        cursor.set(cur);
                                        ws.text_edit.selection = None;
                                        ws.text_edit.selection_anchor = None;
                                        changed = true;
                                    } else if cur < s.len() {
                                        s.remove(cur);
                                        value.set(s.clone());
                                        changed = true;
                                    }
                                }
                                Key::Named(NamedKey::ArrowLeft) if cur > 0 => {
                                    let prev = s[..cur]
                                        .char_indices()
                                        .next_back()
                                        .map(|(i, _)| i)
                                        .unwrap_or(0);
                                    cursor.set(prev);
                                    update_selection_for_move(
                                        &mut ws.text_edit,
                                        cur,
                                        prev,
                                        ws.modifiers.shift_key(),
                                    );
                                }
                                Key::Named(NamedKey::ArrowRight) if cur < s.len() => {
                                    let next = s[cur..]
                                        .char_indices()
                                        .nth(1)
                                        .map(|(i, _)| cur + i)
                                        .unwrap_or(s.len());
                                    cursor.set(next);
                                    update_selection_for_move(
                                        &mut ws.text_edit,
                                        cur,
                                        next,
                                        ws.modifiers.shift_key(),
                                    );
                                }
                                Key::Named(NamedKey::Space) => {
                                    delete_selection(&mut s, &mut cur, ws.text_edit.selection);
                                    s.insert(cur, ' ');
                                    cur += 1;
                                    value.set(s.clone());
                                    cursor.set(cur);
                                    ws.text_edit.selection = None;
                                    ws.text_edit.selection_anchor = None;
                                    changed = true;
                                }
                                Key::Named(NamedKey::Home) => {
                                    cursor.set(0);
                                    update_selection_for_move(
                                        &mut ws.text_edit,
                                        cur,
                                        0,
                                        ws.modifiers.shift_key(),
                                    );
                                }
                                Key::Named(NamedKey::End) => {
                                    cursor.set(s.len());
                                    update_selection_for_move(
                                        &mut ws.text_edit,
                                        cur,
                                        s.len(),
                                        ws.modifiers.shift_key(),
                                    );
                                }
                                Key::Named(NamedKey::Escape) => {
                                    focused.set(false);
                                    ws.text_edit = TextEditState::default();
                                    ws.window.set_ime_allowed(false);
                                }
                                Key::Named(NamedKey::Tab) => {
                                    focused.set(false);
                                    let next_idx = input_indices[(pos + 1) % input_indices.len()];
                                    set_focused_at(&flat, next_idx, true);
                                    ws.window.set_ime_allowed(true);
                                    ws.text_edit = TextEditState {
                                        focused_flat_index: Some(next_idx),
                                        ..TextEditState::default()
                                    };
                                }
                                Key::Named(NamedKey::Enter) => {
                                    if let Some(f) = on_submit {
                                        f(value.get());
                                    }
                                    focused.set(false);
                                    ws.text_edit = TextEditState::default();
                                    ws.window.set_ime_allowed(false);
                                }
                                Key::Character(ch) if !command => {
                                    delete_selection(&mut s, &mut cur, ws.text_edit.selection);
                                    s.insert_str(cur, ch.as_str());
                                    cur += ch.len();
                                    value.set(s.clone());
                                    cursor.set(cur);
                                    ws.text_edit.selection = None;
                                    ws.text_edit.selection_anchor = None;
                                    ws.text_edit.composing = None;
                                    changed = true;
                                }
                                _ => {}
                            }
                            if changed {
                                if let Some(f) = on_change {
                                    f(s);
                                }
                            }
                            handled = true;
                            break;
                        }
                        FlatViewKind::TextArea {
                            value,
                            focused,
                            cursor,
                            scroll_y,
                            on_change,
                            font_size,
                            ..
                        } => {
                            if !focused.get() {
                                continue;
                            }
                            let mut s = value.get();
                            let mut cur = cursor.get().min(s.len());
                            let mut changed = false;
                            match &logical_key {
                                Key::Named(NamedKey::Backspace) if cur > 0 => {
                                    let prev = s[..cur]
                                        .char_indices()
                                        .next_back()
                                        .map(|(i, _)| i)
                                        .unwrap_or(0);
                                    s.remove(prev);
                                    cur = prev;
                                    value.set(s.clone());
                                    cursor.set(cur);
                                    changed = true;
                                }
                                Key::Named(NamedKey::Delete) if cur < s.len() => {
                                    s.remove(cur);
                                    value.set(s.clone());
                                    changed = true;
                                }
                                Key::Named(NamedKey::ArrowLeft) if cur > 0 => {
                                    let prev = s[..cur]
                                        .char_indices()
                                        .next_back()
                                        .map(|(i, _)| i)
                                        .unwrap_or(0);
                                    cursor.set(prev);
                                }
                                Key::Named(NamedKey::ArrowRight) if cur < s.len() => {
                                    let next = s[cur..]
                                        .char_indices()
                                        .nth(1)
                                        .map(|(i, _)| cur + i)
                                        .unwrap_or(s.len());
                                    cursor.set(next);
                                }
                                Key::Named(NamedKey::ArrowUp) => {
                                    let line_height = font_size * 1.4;
                                    let (line_idx, col_off) = byte_to_line_col(&s, cur);
                                    if line_idx > 0 {
                                        cur = line_col_to_byte(&s, line_idx - 1, col_off);
                                        cursor.set(cur);
                                        let new_oy = (scroll_y.get() - line_height).max(0.0);
                                        scroll_y.set(new_oy);
                                    }
                                }
                                Key::Named(NamedKey::ArrowDown) => {
                                    let line_height = font_size * 1.4;
                                    let lines: Vec<&str> = s.split('\n').collect();
                                    let (line_idx, col_off) = byte_to_line_col(&s, cur);
                                    if line_idx + 1 < lines.len() {
                                        cur = line_col_to_byte(&s, line_idx + 1, col_off);
                                        cursor.set(cur);
                                        scroll_y.set(scroll_y.get() + line_height);
                                    }
                                }
                                Key::Named(NamedKey::Space) => {
                                    s.insert(cur, ' ');
                                    cur += 1;
                                    value.set(s.clone());
                                    cursor.set(cur);
                                    changed = true;
                                }
                                Key::Named(NamedKey::Home) => {
                                    cursor.set(0);
                                }
                                Key::Named(NamedKey::End) => {
                                    cursor.set(s.len());
                                }
                                Key::Named(NamedKey::Escape) => {
                                    focused.set(false);
                                }
                                Key::Named(NamedKey::Tab) => {
                                    focused.set(false);
                                    let next_idx = input_indices[(pos + 1) % input_indices.len()];
                                    set_focused_at(&flat, next_idx, true);
                                }
                                Key::Named(NamedKey::Enter) => {
                                    s.insert(cur, '\n');
                                    cur += 1;
                                    value.set(s.clone());
                                    cursor.set(cur);
                                    changed = true;
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
                                if let Some(f) = on_change {
                                    f(s);
                                }
                            }
                            handled = true;
                            break;
                        }
                        _ => {}
                    }
                }
                let _ = handled;
                if needs_redraw() {
                    clear_redraw();
                }
                ws.window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                let Some(ws) = self.windows.get_mut(&id) else {
                    return;
                };
                ws.frame = ws.frame.wrapping_add(1);
                let cursor_visible = (ws.frame / 30) % 2 == 0;
                let scale = ws.scale();

                // Fast path: on scroll-only frames, skip ViewTree::build entirely.
                // The renderer reads scroll offsets from signals live, so scaled_cache
                // is still correct. We must still do a full rebuild if:
                //   - no cached layout yet
                //   - a VirtualList's visible row range has changed (new rows need layout)
                //   - any other signal changed (tweens, clicks, text input, etc.)
                let skip_rebuild = ws.scroll_dirty && !ws.scaled_cache.is_empty() && {
                    let new_ranges = vlist_ranges_from_flat(&ws.flat_cache);
                    new_ranges == ws.vlist_ranges
                };
                ws.scroll_dirty = false;

                if skip_rebuild {
                    ws.renderer.render(&ws.scaled_cache, cursor_visible, ws.theme.background, scale);
                    return;
                }

                let (w, h) = (ws.lw(), ws.lh());
                let view = ws.build(&opener);
                let mut flat = ViewTree::build(view, &ws.theme, w, h, &mut ws.renderer.measurer());
                decorate_text_input_state(&mut flat, &ws.text_edit);
                let any_focused = flat.iter().any(|fv| {
                    matches!(&fv.kind, FlatViewKind::TextInput { focused, .. } if focused.get())
                    || matches!(&fv.kind, FlatViewKind::TextArea { focused, .. } if focused.get())
                });
                if any_focused {
                    ws.window.request_redraw();
                }
                {
                    let mut scroll_stack: Vec<(f32, f32)> = Vec::new();
                    let mut pending: Option<(f32, f32)> = None;
                    let mut hit_items: Vec<HitItem> = Vec::new();
                    for fv in &flat {
                        match &fv.kind {
                            FlatViewKind::ScrollRegion {
                                offset_x, offset_y, ..
                            } => {
                                pending = Some((offset_x.get(), offset_y.get()));
                            }
                            FlatViewKind::ClipStart { .. } => {
                                scroll_stack.push(pending.take().unwrap_or((0.0, 0.0)));
                            }
                            FlatViewKind::ClipEnd => {
                                scroll_stack.pop();
                            }
                            _ => {
                                let sox: f32 = scroll_stack.iter().map(|(ox, _)| ox).sum();
                                let soy: f32 = scroll_stack.iter().map(|(_, oy)| oy).sum();
                                let x = fv.layout.location.x - sox;
                                let y = fv.layout.location.y - soy;
                                let w = fv.layout.size.width;
                                let h = fv.layout.size.height;
                                let item = match &fv.kind {
                                    FlatViewKind::Button { on_hover, .. } => Some(HitItem {
                                        x,
                                        y,
                                        w,
                                        h,
                                        kind: HitKind::Button(on_hover.is_some()),
                                    }),
                                    FlatViewKind::TextInput { .. }
                                    | FlatViewKind::TextArea { .. } => Some(HitItem {
                                        x,
                                        y,
                                        w,
                                        h,
                                        kind: HitKind::Text,
                                    }),
                                    _ => None,
                                };
                                if let Some(item) = item {
                                    hit_items.push(item);
                                }
                            }
                        }
                    }
                    ws.hit_items = hit_items;
                }
                // Build scroll_items. Store content-space positions and enclosing signal refs
                // so apply_scroll can compute live screen-space hit rects even after the page scrolls.
                ws.scroll_items = {
                    let mut items = Vec::new();
                    // Stack of (offset_x_signal, offset_y_signal) for enclosing scroll regions.
                    let mut enclosing_stack: Vec<(Signal<f32>, Signal<f32>)> = Vec::new();
                    let mut pending: Option<(Signal<f32>, Signal<f32>, f32, f32, core_glyph::TaffyLayout)> = None;
                    for fv in &flat {
                        match &fv.kind {
                            FlatViewKind::ScrollRegion { offset_x, offset_y, max_x, max_y, .. } => {
                                pending = Some((offset_x.clone(), offset_y.clone(), *max_x, *max_y, fv.layout));
                            }
                            FlatViewKind::ClipStart { .. } => {
                                if let Some((offset_x, offset_y, max_x, max_y, l)) = pending.take() {
                                    items.push(ScrollItem {
                                        cx: l.location.x,
                                        cy: l.location.y,
                                        w: l.size.width,
                                        h: l.size.height,
                                        enclosing: enclosing_stack.clone(),
                                        offset_x: offset_x.clone(),
                                        offset_y: offset_y.clone(),
                                        max_x,
                                        max_y,
                                    });
                                    enclosing_stack.push((offset_x, offset_y));
                                } else {
                                    // Non-scroll clip (e.g. overflow:hidden container) — push dummy.
                                    enclosing_stack.push((Signal::new(0.0), Signal::new(0.0)));
                                }
                            }
                            FlatViewKind::ClipEnd => { enclosing_stack.pop(); }
                            _ => {}
                        }
                    }
                    items
                };
                ws.vlist_ranges = vlist_ranges_from_flat(&flat);
                ws.flat_cache = flat.clone();
                ws.scaled_cache = scale_flat(flat, scale);
                clear_redraw();
                ws.renderer
                    .render(&ws.scaled_cache, cursor_visible, ws.theme.background, scale);
            }

            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// TextArea cursor helpers
// ---------------------------------------------------------------------------

fn byte_to_line_col(s: &str, byte: usize) -> (usize, usize) {
    let before = &s[..byte.min(s.len())];
    let line_idx = before.chars().filter(|&c| c == '\n').count();
    let col = before.rfind('\n').map(|p| byte - p - 1).unwrap_or(byte);
    (line_idx, col)
}

fn line_col_to_byte(s: &str, target_line: usize, col: usize) -> usize {
    let mut offset = 0;
    for (li, line) in s.split('\n').enumerate() {
        if li == target_line {
            return offset + col.min(line.len());
        }
        offset += line.len() + 1;
    }
    s.len()
}

fn set_focused_at(flat: &[FlatView], idx: usize, focused: bool) {
    match &flat[idx].kind {
        FlatViewKind::TextInput { focused: f, .. } => f.set(focused),
        FlatViewKind::TextArea { focused: f, .. } => f.set(focused),
        _ => {}
    }
}

fn normalized_selection(anchor: usize, cursor: usize) -> Option<(usize, usize)> {
    if anchor == cursor {
        None
    } else {
        Some((anchor.min(cursor), anchor.max(cursor)))
    }
}

fn update_selection_for_move(
    edit: &mut TextEditState,
    old_cursor: usize,
    new_cursor: usize,
    extend: bool,
) {
    if extend {
        let anchor = edit.selection_anchor.unwrap_or(old_cursor);
        edit.selection_anchor = Some(anchor);
        edit.selection = normalized_selection(anchor, new_cursor);
    } else {
        edit.selection_anchor = None;
        edit.selection = None;
    }
    edit.composing = None;
}

fn delete_selection(s: &mut String, cursor: &mut usize, selection: Option<(usize, usize)>) -> bool {
    let Some((start, end)) = selection else {
        return false;
    };
    let start = start.min(s.len());
    let end = end.min(s.len());
    if start >= end || !s.is_char_boundary(start) || !s.is_char_boundary(end) {
        return false;
    }
    s.replace_range(start..end, "");
    *cursor = start;
    true
}

fn decorate_text_input_state(flat: &mut [FlatView], edit: &TextEditState) {
    let Some(idx) = edit.focused_flat_index else {
        return;
    };
    let Some(fv) = flat.get_mut(idx) else {
        return;
    };
    if let FlatViewKind::TextInput {
        selection,
        composing,
        ..
    } = &mut fv.kind
    {
        *selection = edit.selection;
        *composing = edit.composing.clone();
    }
}

// ---------------------------------------------------------------------------
// DPI scaling: layout runs in logical pixels; renderer works in physical pixels.
// Scale every position, size, and font size in the flat list before rendering.
// ---------------------------------------------------------------------------

fn scale_flat(flat: Vec<FlatView>, scale: f32) -> Vec<FlatView> {
    if (scale - 1.0).abs() < f32::EPSILON {
        return flat;
    }
    flat.into_iter()
        .map(|fv| {
            let l = &fv.layout;
            let mut layout = *l;
            layout.location.x *= scale;
            layout.location.y *= scale;
            layout.size.width *= scale;
            layout.size.height *= scale;
            let kind = match fv.kind {
                FlatViewKind::Text {
                    content,
                    font_size,
                    color,
                    weight,
                    align,
                    wrap,
                    family,
                } => FlatViewKind::Text {
                    content,
                    font_size: font_size * scale,
                    color,
                    weight,
                    align,
                    wrap,
                    family,
                },
                FlatViewKind::Button {
                    label,
                    on_click,
                    on_hover,
                    on_press,
                    bg_color,
                    hover_bg_color,
                    press_bg_color,
                    text_color,
                    corner_radius,
                    font_size,
                    wrap,
                    family,
                } => FlatViewKind::Button {
                    label,
                    on_click,
                    on_hover,
                    on_press,
                    bg_color,
                    hover_bg_color,
                    press_bg_color,
                    text_color,
                    corner_radius: corner_radius * scale,
                    font_size: font_size * scale,
                    wrap,
                    family,
                },
                FlatViewKind::TextInput {
                    value,
                    focused,
                    cursor,
                    scroll_x,
                    placeholder,
                    font_size,
                    bg_color,
                    text_color,
                    border_color,
                    corner_radius,
                    on_change,
                    on_submit,
                    selection,
                    composing,
                } => FlatViewKind::TextInput {
                    value,
                    focused,
                    cursor,
                    scroll_x,
                    placeholder,
                    font_size: font_size * scale,
                    bg_color,
                    text_color,
                    border_color,
                    corner_radius: corner_radius * scale,
                    on_change,
                    on_submit,
                    selection,
                    composing,
                },
                FlatViewKind::ContainerRect {
                    bg_color,
                    border_color,
                    border_width,
                    corner_radius,
                    shadow,
                } => {
                    let shadow = shadow.map(|s| core_glyph::Shadow {
                        offset_x: s.offset_x * scale,
                        offset_y: s.offset_y * scale,
                        blur: s.blur * scale,
                        color: s.color,
                    });
                    FlatViewKind::ContainerRect {
                        bg_color,
                        border_color,
                        border_width: border_width * scale,
                        corner_radius: corner_radius * scale,
                        shadow,
                    }
                }
                FlatViewKind::ClipStart {
                    x,
                    y,
                    width,
                    height,
                    is_virtual_list,
                } => FlatViewKind::ClipStart {
                    x: x * scale,
                    y: y * scale,
                    width: width * scale,
                    height: height * scale,
                    is_virtual_list,
                },
                FlatViewKind::Image {
                    path,
                    corner_radius,
                } => FlatViewKind::Image {
                    path,
                    corner_radius: corner_radius * scale,
                },
                FlatViewKind::TextArea {
                    value,
                    focused,
                    cursor,
                    scroll_y,
                    placeholder,
                    font_size,
                    bg_color,
                    text_color,
                    border_color,
                    corner_radius,
                    on_change,
                } => FlatViewKind::TextArea {
                    value,
                    focused,
                    cursor,
                    scroll_y,
                    placeholder,
                    font_size: font_size * scale,
                    bg_color,
                    text_color,
                    border_color,
                    corner_radius: corner_radius * scale,
                    on_change,
                },
                FlatViewKind::Rect { color, corner_radius } => FlatViewKind::Rect {
                    color,
                    corner_radius: corner_radius * scale,
                },
                // ScrollRegion is logical-pixel metadata only; renderer ignores it.
                FlatViewKind::ScrollRegion {
                    offset_x,
                    offset_y,
                    max_x,
                    max_y,
                    is_virtual_list,
                } => FlatViewKind::ScrollRegion {
                    offset_x,
                    offset_y,
                    max_x,
                    max_y,
                    is_virtual_list,
                },
                other => other,
            };
            FlatView { kind, layout }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// VirtualList range tracking: compute (offset_y, first_row, last_row) for each
// ScrollRegion that precedes a VirtualList clip so we can detect when the visible
// set of rows changes and a full rebuild is needed.
// ---------------------------------------------------------------------------

/// Snapshot the VirtualList offset values at the time of the last full rebuild.
/// Stored in `ws.vlist_ranges` after each rebuild. On the next frame, the live
/// signal values are read again and compared — if any changed, skip_rebuild is false.
fn vlist_ranges_from_flat(flat: &[FlatView]) -> Vec<f32> {
    flat.iter()
        .filter_map(|fv| {
            if let FlatViewKind::ScrollRegion { offset_y, is_virtual_list: true, .. } = &fv.kind {
                Some(offset_y.get())
            } else {
                None
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Nested scroll dispatch: innermost container wins, bubbles to outer if at limit.
// ---------------------------------------------------------------------------

fn apply_scroll(items: &[ScrollItem], cx: f32, cy: f32, dx: f32, dy: f32) {
    // Iterate in reverse so the last-emitted (innermost/deepest) container is
    // checked first. The innermost container whose bounds contain the cursor
    // absorbs the event unconditionally — no bubbling to outer containers.
    for item in items.iter().rev() {
        // Compute live screen-space position by subtracting current enclosing offsets.
        let sox: f32 = item.enclosing.iter().map(|(ox, _)| ox.get()).sum();
        let soy: f32 = item.enclosing.iter().map(|(_, oy)| oy.get()).sum();
        let sx = item.cx - sox;
        let sy = item.cy - soy;
        if cx >= sx && cx <= sx + item.w && cy >= sy && cy <= sy + item.h {
            let cur_x = item.offset_x.get();
            let cur_y = item.offset_y.get();
            let nx = (cur_x - dx).clamp(0.0, item.max_x);
            let ny = (cur_y - dy).clamp(0.0, item.max_y);
            item.offset_x.set(nx);
            item.offset_y.set(ny);
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// Scroll dispatch — only used by HotApp's MouseWheel handler.
// ---------------------------------------------------------------------------

#[cfg(feature = "hot-reload")]
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
        View::Scroll {
            child,
            offset_x,
            offset_y,
            ..
        } => {
            *flat_idx += 1; // skip ScrollRegion
            if let Some(fv) = flat.get(*flat_idx) {
                if let FlatViewKind::ClipStart {
                    x,
                    y,
                    width,
                    height,
                    ..
                } = &fv.kind
                {
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
        View::Column {
            children,
            bg_color,
            border_color,
            shadow,
            clip,
            ..
        }
        | View::Row {
            children,
            bg_color,
            border_color,
            shadow,
            clip,
            ..
        } => {
            if bg_color.is_some() || border_color.is_some() || shadow.is_some() {
                *flat_idx += 1;
            }
            if *clip {
                *flat_idx += 1;
            }
            for child in children {
                dispatch_scroll(child, theme, flat, cx, cy, dx, dy, flat_idx);
            }
            if *clip {
                *flat_idx += 1;
            }
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
        View::Button { .. }
        | View::Rect { .. }
        | View::Text { .. }
        | View::TextInput { .. }
        | View::Image { .. }
        | View::TextArea { .. } => {
            *flat_idx += 1;
        }
        View::VirtualList {
            item_count,
            row_height,
            offset_y,
            viewport_height,
            ..
        } => {
            *flat_idx += 1; // skip ScrollRegion
            if let Some(fv) = flat.get(*flat_idx) {
                if let FlatViewKind::ClipStart {
                    x,
                    y,
                    width,
                    height,
                    ..
                } = &fv.kind
                {
                    if cx >= *x && cx <= x + width && cy >= *y && cy <= y + height {
                        let max_scroll =
                            ((*item_count as f32) * row_height - viewport_height).max(0.0);
                        offset_y.set((offset_y.get() - dy).clamp(0.0, max_scroll));
                    }
                }
            }
            // Skip forward past all flat entries belonging to this VirtualList
            // (ClipStart + rows + ClipEnd). We track nesting depth to handle
            // nested clips correctly.
            *flat_idx += 1; // consume ClipStart
            let mut depth = 1usize;
            while *flat_idx < flat.len() && depth > 0 {
                match &flat[*flat_idx].kind {
                    FlatViewKind::ClipStart { .. } => {
                        depth += 1;
                        *flat_idx += 1;
                    }
                    FlatViewKind::ClipEnd => {
                        depth -= 1;
                        *flat_idx += 1;
                    }
                    _ => {
                        *flat_idx += 1;
                    }
                }
            }
        }
        View::Flexible { child, .. } => {
            dispatch_scroll(child, theme, flat, cx, cy, dx, dy, flat_idx);
        }
        View::Opacity { child, .. } => {
            *flat_idx += 1; // OpacityStart
            dispatch_scroll(child, theme, flat, cx, cy, dx, dy, flat_idx);
            *flat_idx += 1; // OpacityEnd
        }
        View::Spacer => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_selection_removes_valid_byte_range_and_moves_cursor() {
        let mut value = "hello".to_string();
        let mut cursor = 4;

        assert!(delete_selection(&mut value, &mut cursor, Some((1, 4))));
        assert_eq!(value, "ho");
        assert_eq!(cursor, 1);
    }

    #[test]
    fn delete_selection_rejects_non_char_boundaries() {
        let mut value = "éx".to_string();
        let mut cursor = value.len();

        assert!(!delete_selection(&mut value, &mut cursor, Some((1, 2))));
        assert_eq!(value, "éx");
        assert_eq!(cursor, 3);
    }

    #[test]
    fn delete_selection_noop_when_none() {
        let mut value = "hello".to_string();
        let mut cursor = 3;
        assert!(!delete_selection(&mut value, &mut cursor, None));
        assert_eq!(value, "hello");
        assert_eq!(cursor, 3);
    }

    // --- normalized_selection ---

    #[test]
    fn normalized_selection_returns_none_when_equal() {
        assert_eq!(normalized_selection(3, 3), None);
    }

    #[test]
    fn normalized_selection_orders_low_high() {
        assert_eq!(normalized_selection(5, 2), Some((2, 5)));
        assert_eq!(normalized_selection(2, 5), Some((2, 5)));
    }

    // --- update_selection_for_move ---

    #[test]
    fn update_selection_extend_creates_selection() {
        let mut edit = TextEditState::default();
        update_selection_for_move(&mut edit, 0, 3, true);
        assert_eq!(edit.selection_anchor, Some(0));
        assert_eq!(edit.selection, Some((0, 3)));
    }

    #[test]
    fn update_selection_no_extend_clears_selection() {
        let mut edit = TextEditState {
            selection: Some((1, 4)),
            selection_anchor: Some(1),
            ..Default::default()
        };
        update_selection_for_move(&mut edit, 4, 5, false);
        assert_eq!(edit.selection, None);
        assert_eq!(edit.selection_anchor, None);
    }

    #[test]
    fn update_selection_extend_preserves_anchor() {
        let mut edit = TextEditState {
            selection_anchor: Some(2),
            ..Default::default()
        };
        update_selection_for_move(&mut edit, 4, 6, true);
        assert_eq!(edit.selection_anchor, Some(2));
        assert_eq!(edit.selection, Some((2, 6)));
    }

    // --- scale_flat ---

    fn rect_flat(x: f32, y: f32, w: f32, h: f32, corner_radius: f32) -> FlatView {
        use core_glyph::FlatView;
        let mut layout = core_glyph::TaffyLayout::default();
        layout.location.x = x;
        layout.location.y = y;
        layout.size.width = w;
        layout.size.height = h;
        FlatView {
            kind: FlatViewKind::Rect { color: core_glyph::Color::WHITE, corner_radius },
            layout,
        }
    }

    #[test]
    fn scale_flat_scales_position_and_size() {
        let flat = vec![rect_flat(10.0, 20.0, 100.0, 50.0, 4.0)];
        let scaled = scale_flat(flat, 2.0);
        assert_eq!(scaled[0].layout.location.x, 20.0);
        assert_eq!(scaled[0].layout.location.y, 40.0);
        assert_eq!(scaled[0].layout.size.width, 200.0);
        assert_eq!(scaled[0].layout.size.height, 100.0);
    }

    #[test]
    fn scale_flat_scales_corner_radius() {
        let flat = vec![rect_flat(0.0, 0.0, 50.0, 50.0, 8.0)];
        let scaled = scale_flat(flat, 2.0);
        assert!(matches!(scaled[0].kind, FlatViewKind::Rect { corner_radius, .. } if corner_radius == 16.0));
    }

    #[test]
    fn scale_flat_identity_at_1x() {
        let flat = vec![rect_flat(5.0, 10.0, 80.0, 40.0, 2.0)];
        let scaled = scale_flat(flat, 1.0);
        assert_eq!(scaled[0].layout.location.x, 5.0);
        assert_eq!(scaled[0].layout.size.width, 80.0);
    }

    // --- apply_scroll ---

    fn scroll_item(x: f32, y: f32, w: f32, h: f32, max_y: f32) -> ScrollItem {
        ScrollItem {
            cx: x, cy: y, w, h,
            offset_x: core_glyph::Signal::new(0.0f32),
            offset_y: core_glyph::Signal::new(0.0f32),
            max_x: 0.0,
            max_y,
            enclosing: Vec::new(),
        }
    }

    #[test]
    fn apply_scroll_updates_offset_when_cursor_inside() {
        let item = scroll_item(0.0, 0.0, 400.0, 300.0, 500.0);
        let items = vec![item];
        apply_scroll(&items, 200.0, 150.0, 0.0, -20.0);
        assert_eq!(items[0].offset_y.get(), 20.0);
    }

    #[test]
    fn apply_scroll_clamps_to_max() {
        let item = scroll_item(0.0, 0.0, 400.0, 300.0, 100.0);
        let items = vec![item];
        apply_scroll(&items, 200.0, 150.0, 0.0, -9999.0);
        assert_eq!(items[0].offset_y.get(), 100.0);
    }

    #[test]
    fn apply_scroll_clamps_to_zero() {
        let item = scroll_item(0.0, 0.0, 400.0, 300.0, 100.0);
        item.offset_y.set(50.0);
        let items = vec![item];
        apply_scroll(&items, 200.0, 150.0, 0.0, 9999.0);
        assert_eq!(items[0].offset_y.get(), 0.0);
    }

    #[test]
    fn apply_scroll_ignores_cursor_outside_bounds() {
        let item = scroll_item(0.0, 0.0, 400.0, 300.0, 500.0);
        let items = vec![item];
        apply_scroll(&items, 500.0, 150.0, 0.0, -20.0); // cursor outside
        assert_eq!(items[0].offset_y.get(), 0.0);
    }

    #[test]
    fn apply_scroll_innermost_container_wins() {
        let outer = scroll_item(0.0, 0.0, 400.0, 300.0, 500.0);
        let inner = scroll_item(50.0, 50.0, 200.0, 150.0, 500.0);
        // inner is pushed last so it's "innermost" in reverse iteration
        let items = vec![outer, inner];
        apply_scroll(&items, 100.0, 100.0, 0.0, -20.0);
        assert_eq!(items[1].offset_y.get(), 20.0); // inner absorbed
        assert_eq!(items[0].offset_y.get(), 0.0);  // outer untouched
    }
}

// ---------------------------------------------------------------------------
// HotApp (hot-reload variant, unchanged from before)
// ---------------------------------------------------------------------------

#[cfg(feature = "hot-reload")]
pub struct HotApp {
    loader: hot_glyph::HotLoader,
    theme: Theme,
    title: String,
    width: f64,
    height: f64,
    state: Option<HotAppState>,
}

#[cfg(feature = "hot-reload")]
struct HotAppState {
    window: Arc<Window>,
    renderer: Renderer,
    cursor_pos: (f32, f32),
    frame: u32,
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
        let loader = hot_glyph::HotLoader::new(src_dir.as_ref(), lib_path.as_ref(), package_name);
        let event_loop = EventLoop::new().expect("event loop");
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let mut app = HotApp {
            loader,
            theme,
            title: title.into(),
            width,
            height,
            state: None,
        };
        event_loop.run_app(&mut app).expect("event loop run");
    }
}

#[cfg(feature = "hot-reload")]
impl ApplicationHandler for HotApp {
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
        let ctx = pollster::block_on(GpuContext::new_with_window(Arc::clone(&window)));
        let (surface, surface_cfg) =
            ctx.create_surface(Arc::clone(&window), size.width.max(1), size.height.max(1));
        let renderer = Renderer::new(ctx, surface, surface_cfg);
        self.state = Some(HotAppState {
            window,
            renderer,
            cursor_pos: (0.0, 0.0),
            frame: 0,
        });
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
                    let flat =
                        ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    let mut changed = false;
                    for fv in &flat {
                        let l = fv.layout.location.x;
                        let t = fv.layout.location.y;
                        let hit = px >= l
                            && px <= l + fv.layout.size.width
                            && py >= t
                            && py <= t + fv.layout.size.height;
                        if let FlatViewKind::Button {
                            on_hover: Some(on_hover),
                            ..
                        } = &fv.kind
                        {
                            on_hover(hit);
                            changed = true;
                        }
                    }
                    if changed {
                        state.window.request_redraw();
                    }
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x * 20.0, y * 20.0),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                let (cx, cy) = state.cursor_pos;
                let scale = state.window.scale_factor() as f32;
                let w = state.renderer.surface_cfg.width as f32 / scale;
                let h = state.renderer.surface_cfg.height as f32 / scale;
                if let Some(view) = self.loader.build_view(&self.theme) {
                    let flat =
                        ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    if let Some(view2) = self.loader.build_view(&self.theme) {
                        let mut idx = 0;
                        dispatch_scroll(&view2, &self.theme, &flat, cx, cy, dx, dy, &mut idx);
                    }
                }
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
                let scale = state.window.scale_factor() as f32;
                let w = state.renderer.surface_cfg.width as f32 / scale;
                let h = state.renderer.surface_cfg.height as f32 / scale;
                if let Some(view) = self.loader.build_view(&self.theme) {
                    let flat =
                        ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    for fv in &flat {
                        let l = fv.layout.location.x;
                        let t = fv.layout.location.y;
                        let hit = cx >= l
                            && cx <= l + fv.layout.size.width
                            && cy >= t
                            && cy <= t + fv.layout.size.height;
                        match &fv.kind {
                            FlatViewKind::Button { on_click, .. } => {
                                if hit {
                                    on_click();
                                }
                            }
                            FlatViewKind::TextInput { focused, .. } => {
                                focused.set(hit);
                                if hit {
                                    state.frame = 0;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                if needs_redraw() {
                    clear_redraw();
                }
                state.window.request_redraw();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                let scale = state.window.scale_factor() as f32;
                let w = state.renderer.surface_cfg.width as f32 / scale;
                let h = state.renderer.surface_cfg.height as f32 / scale;
                if let Some(view) = self.loader.build_view(&self.theme) {
                    let flat =
                        ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    for fv in &flat {
                        if let FlatViewKind::TextInput {
                            value,
                            focused,
                            on_submit,
                            ..
                        } = &fv.kind
                        {
                            if !focused.get() {
                                continue;
                            }
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
                }
                if needs_redraw() {
                    clear_redraw();
                }
                state.window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                state.frame = state.frame.wrapping_add(1);
                let cursor_visible = (state.frame / 30) % 2 == 0;
                let scale = state.window.scale_factor() as f32;
                let w = state.renderer.surface_cfg.width as f32 / scale;
                let h = state.renderer.surface_cfg.height as f32 / scale;
                if let Some(view) = self.loader.build_view(&self.theme) {
                    let flat =
                        ViewTree::build(view, &self.theme, w, h, &mut state.renderer.measurer());
                    let any_focused = flat.iter().any(|fv| {
                        matches!(&fv.kind, FlatViewKind::TextInput { focused, .. } if focused.get())
                    });
                    if any_focused {
                        state.window.request_redraw();
                    }
                    let flat = scale_flat(flat, scale);
                    state
                        .renderer
                        .render(&flat, cursor_visible, self.theme.background, scale);
                }
            }

            _ => {}
        }
    }
}
