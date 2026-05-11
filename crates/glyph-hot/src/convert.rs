/// Convert a guest-allocated `CViewDesc` tree into a host-native `View` tree.
///
/// After conversion the caller must free the `CViewDesc` tree via the guest's
/// `glyph_hot_free_node` export. Signals are not owned by the `CViewDesc` tree
/// — they live in the `SignalRegistry` which outlives individual view builds.
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

use glyph_core::{
    Color, FontWeight, Shadow, Signal, TextAlign, View,
    button, column, image, rect, row, scroll, spacer, text, text_input, zstack, flexible,
};

use crate::abi::{
    CButtonData, CChildren, CColor, CContainerData, CFlexibleData, CImageData,
    CRectData, CScrollData, CShadow, CTextData, CTextInputData, CViewDesc,
    CViewTag, CZStackData, FnFreeNode, FnFreeStr, GlyphSignalTable,
};

// ---------------------------------------------------------------------------
// Signal registry — host owns all signals, guest gets opaque handles
// ---------------------------------------------------------------------------

pub enum SignalSlot {
    I32(Signal<i32>),
    F32(Signal<f32>),
    Bool(Signal<bool>),
    Str(Signal<String>),
}

/// Owns all signals created during a hot-reload session. Signals are keyed by
/// a stable integer ID that maps to a raw pointer (`*mut SignalSlot`) passed to
/// the guest as an opaque handle. The pointer remains valid for the lifetime of
/// the `SignalRegistry`.
pub struct SignalRegistry {
    /// Heap-allocated slots; kept alive behind Box so their addresses are stable.
    slots: Vec<Box<SignalSlot>>,
}

impl SignalRegistry {
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    fn alloc(&mut self, slot: SignalSlot) -> *mut c_void {
        let boxed = Box::new(slot);
        let ptr = &*boxed as *const SignalSlot as *mut c_void;
        self.slots.push(boxed);
        ptr
    }

    pub fn new_i32(&mut self, initial: i32) -> *mut c_void {
        self.alloc(SignalSlot::I32(Signal::new(initial)))
    }

    pub fn new_f32(&mut self, initial: f32) -> *mut c_void {
        self.alloc(SignalSlot::F32(Signal::new(initial)))
    }

    pub fn new_bool(&mut self, initial: bool) -> *mut c_void {
        self.alloc(SignalSlot::Bool(Signal::new(initial)))
    }

    pub fn new_str(&mut self, initial: String) -> *mut c_void {
        self.alloc(SignalSlot::Str(Signal::new(initial)))
    }

    pub fn signal_i32(&self, handle: *mut c_void) -> Option<&Signal<i32>> {
        if handle.is_null() { return None; }
        let slot = unsafe { &*(handle as *const SignalSlot) };
        if let SignalSlot::I32(s) = slot { Some(s) } else { None }
    }

    pub fn signal_f32(&self, handle: *mut c_void) -> Option<&Signal<f32>> {
        if handle.is_null() { return None; }
        let slot = unsafe { &*(handle as *const SignalSlot) };
        if let SignalSlot::F32(s) = slot { Some(s) } else { None }
    }

    pub fn signal_bool(&self, handle: *mut c_void) -> Option<&Signal<bool>> {
        if handle.is_null() { return None; }
        let slot = unsafe { &*(handle as *const SignalSlot) };
        if let SignalSlot::Bool(s) = slot { Some(s) } else { None }
    }

    pub fn signal_str(&self, handle: *mut c_void) -> Option<&Signal<String>> {
        if handle.is_null() { return None; }
        let slot = unsafe { &*(handle as *const SignalSlot) };
        if let SignalSlot::Str(s) = slot { Some(s) } else { None }
    }
}

// ---------------------------------------------------------------------------
// The GlyphSignalTable the host provides to the guest at create_state time
// ---------------------------------------------------------------------------

// SignalRegistry pointer stored in a thread-local so C trampolines (no &self) can reach it.
std::thread_local! {
    static REGISTRY: std::cell::RefCell<Option<*mut SignalRegistry>> = std::cell::RefCell::new(None);
}

/// Register the registry pointer in the thread-local so trampolines can reach it.
/// Must be called before `glyph_create_state`.
pub fn set_active_registry(reg: *mut SignalRegistry) {
    REGISTRY.with(|r| *r.borrow_mut() = Some(reg));
}

fn with_registry<R>(f: impl FnOnce(&mut SignalRegistry) -> R) -> R {
    REGISTRY.with(|r| {
        let ptr = r.borrow().expect("signal registry not set");
        f(unsafe { &mut *ptr })
    })
}

extern "C" fn trampoline_new_i32(initial: i32) -> *mut c_void {
    with_registry(|r| r.new_i32(initial))
}
extern "C" fn trampoline_get_i32(handle: *mut c_void) -> i32 {
    with_registry(|r| r.signal_i32(handle).map_or(0, |s| s.get()))
}
extern "C" fn trampoline_set_i32(handle: *mut c_void, val: i32) {
    with_registry(|r| { if let Some(s) = r.signal_i32(handle) { s.set(val); } });
}

extern "C" fn trampoline_new_f32(initial: f32) -> *mut c_void {
    with_registry(|r| r.new_f32(initial))
}
extern "C" fn trampoline_get_f32(handle: *mut c_void) -> f32 {
    with_registry(|r| r.signal_f32(handle).map_or(0.0, |s| s.get()))
}
extern "C" fn trampoline_set_f32(handle: *mut c_void, val: f32) {
    with_registry(|r| { if let Some(s) = r.signal_f32(handle) { s.set(val); } });
}

extern "C" fn trampoline_new_bool(initial: u8) -> *mut c_void {
    with_registry(|r| r.new_bool(initial != 0))
}
extern "C" fn trampoline_get_bool(handle: *mut c_void) -> u8 {
    with_registry(|r| r.signal_bool(handle).map_or(0, |s| s.get() as u8))
}
extern "C" fn trampoline_set_bool(handle: *mut c_void, val: u8) {
    with_registry(|r| { if let Some(s) = r.signal_bool(handle) { s.set(val != 0); } });
}

extern "C" fn trampoline_new_str(initial: *const c_char) -> *mut c_void {
    let s = unsafe { CStr::from_ptr(initial) }.to_string_lossy().into_owned();
    with_registry(|r| r.new_str(s))
}
extern "C" fn trampoline_get_str(handle: *mut c_void, buf: *mut c_char, cap: usize) -> usize {
    with_registry(|r| {
        let val = r.signal_str(handle).map_or_else(String::new, |s| s.get());
        let bytes = val.as_bytes();
        let n = bytes.len().min(cap.saturating_sub(1));
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf as *mut u8, n);
            *buf.add(n) = 0;
        }
        n
    })
}
extern "C" fn trampoline_set_str(handle: *mut c_void, val: *const c_char) {
    let s = unsafe { CStr::from_ptr(val) }.to_string_lossy().into_owned();
    with_registry(|r| { if let Some(sig) = r.signal_str(handle) { sig.set(s); } });
}

pub fn build_signal_table() -> GlyphSignalTable {
    GlyphSignalTable {
        new_i32:  trampoline_new_i32,
        get_i32:  trampoline_get_i32,
        set_i32:  trampoline_set_i32,
        new_f32:  trampoline_new_f32,
        get_f32:  trampoline_get_f32,
        set_f32:  trampoline_set_f32,
        new_bool: trampoline_new_bool,
        get_bool: trampoline_get_bool,
        set_bool: trampoline_set_bool,
        new_str:  trampoline_new_str,
        get_str:  trampoline_get_str,
        set_str:  trampoline_set_str,
    }
}

// ---------------------------------------------------------------------------
// CViewDesc → View
// ---------------------------------------------------------------------------

unsafe fn cstr_to_string(ptr: *mut c_char) -> String {
    if ptr.is_null() { return String::new(); }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

fn ccolor(c: CColor) -> Color {
    Color::rgba(c.r, c.g, c.b, c.a)
}

fn cshadow(s: CShadow) -> Shadow {
    Shadow::new(s.offset_x, s.offset_y, s.blur, ccolor(s.color))
}

unsafe fn children_to_views(
    kids: &CChildren,
    registry: &SignalRegistry,
    free_node: FnFreeNode,
    free_str: FnFreeStr,
) -> Vec<View> {
    (0..kids.len)
        .map(|i| {
            let child_ptr = kids.ptr.add(i);
            cdesc_to_view(child_ptr, registry, free_node, free_str)
        })
        .collect()
}

/// Convert a single `CViewDesc` node (and its subtree) into a `View`.
/// The caller retains ownership of `node` and must free it afterwards via
/// `free_node` — we only read, never free, inside this function.
pub unsafe fn cdesc_to_view(
    node: *mut CViewDesc,
    registry: &SignalRegistry,
    free_node: FnFreeNode,
    free_str: FnFreeStr,
) -> View {
    let desc = &*node;

    match desc.tag {
        CViewTag::Spacer => spacer(),

        CViewTag::Rect => {
            let d = &*(desc.data as *const CRectData);
            let mut v = rect(ccolor(d.color));
            if d.width != 0.0 { v = v.width(d.width); }
            if d.height != 0.0 { v = v.height(d.height); }
            v.into()
        }

        CViewTag::Text => {
            let d = &*(desc.data as *const CTextData);
            let content = cstr_to_string(d.content);
            let weight = if d.weight == 1 { FontWeight::Bold } else { FontWeight::Regular };
            let align = match d.align {
                1 => TextAlign::Center,
                2 => TextAlign::Right,
                _ => TextAlign::Left,
            };
            let mut tv = text(content, d.font_size)
                .color(ccolor(d.color))
                .weight(weight)
                .align(align);
            if d.wrap != 0 { tv = tv.wrap(); }
            if d.max_width != 0.0 { tv = tv.width(d.max_width); }
            tv.into_view()
        }

        CViewTag::Button => {
            let d = &*(desc.data as *const CButtonData);
            let label = cstr_to_string(d.label);

            let cb = d.on_click;
            let on_click = move || (cb.fn_ptr)(cb.data);

            let mut bv = button(label, on_click)
                .bg(ccolor(d.bg_color))
                .text_color(ccolor(d.text_color))
                .radius(d.corner_radius)
                .font_size(d.font_size);

            if d.has_hover_bg != 0 {
                bv = bv.hover_bg(ccolor(d.hover_bg_color));
            }
            if d.has_on_hover != 0 {
                let hcb = d.on_hover;
                bv = bv.on_hover(move |hit| (hcb.fn_ptr)(hcb.data, hit as u8));
            }
            bv.into_view()
        }

        CViewTag::Column | CViewTag::Row => {
            let d = &*(desc.data as *const CContainerData);
            let kids = children_to_views(&d.children, registry, free_node, free_str);

            let bg = if d.has_bg != 0 { Some(ccolor(d.bg_color)) } else { None };
            let border = if d.has_border != 0 { Some(ccolor(d.border_color)) } else { None };
            let shadow = if d.has_shadow != 0 { Some(cshadow(d.shadow)) } else { None };

            if desc.tag == CViewTag::Column {
                let mut cv = column(kids).gap(d.gap).padding(d.padding);
                if let Some(c) = bg { cv = cv.bg(c); }
                if let (Some(bc), w) = (border, d.border_width) { cv = cv.border(bc, w); }
                if let Some(s) = shadow { cv = cv.shadow(s); }
                if d.clip != 0 { cv = cv.clip(); }
                if d.corner_radius != 0.0 { cv = cv.radius(d.corner_radius); }
                if d.grow != 0.0 { cv = cv.grow(); }
                if d.width > 0.0 { cv = cv.width(d.width); }
                cv.into_view()
            } else {
                let mut rv = row(kids).gap(d.gap).padding(d.padding);
                if let Some(c) = bg { rv = rv.bg(c); }
                if let (Some(bc), w) = (border, d.border_width) { rv = rv.border(bc, w); }
                if let Some(s) = shadow { rv = rv.shadow(s); }
                if d.clip != 0 { rv = rv.clip(); }
                if d.corner_radius != 0.0 { rv = rv.radius(d.corner_radius); }
                if d.grow != 0.0 { rv = rv.grow(); }
                if d.width > 0.0 { rv = rv.width(d.width); }
                rv.into_view()
            }
        }

        CViewTag::ZStack => {
            let d = &*(desc.data as *const CZStackData);
            let kids = children_to_views(&d.children, registry, free_node, free_str);
            zstack(kids).into()
        }

        CViewTag::Scroll => {
            let d = &*(desc.data as *const CScrollData);
            let child = cdesc_to_view(d.child, registry, free_node, free_str);

            let ox = registry.signal_f32(d.offset_x_handle)
                .cloned()
                .unwrap_or_else(|| Signal::new(0.0f32));
            let oy = registry.signal_f32(d.offset_y_handle)
                .cloned()
                .unwrap_or_else(|| Signal::new(0.0f32));

            let mut sv = scroll(child, ox, oy, Signal::new((0.0f32, 0.0f32)));
            if d.width != 0.0 && d.height != 0.0 {
                sv = sv.size(d.width, d.height);
            }
            sv.into()
        }

        CViewTag::Image => {
            let d = &*(desc.data as *const CImageData);
            let path = cstr_to_string(d.path);
            image(path).size(d.width, d.height).radius(d.corner_radius).into()
        }

        CViewTag::TextInput => {
            let d = &*(desc.data as *const CTextInputData);
            let placeholder = cstr_to_string(d.placeholder);

            let value = registry
                .signal_str(d.value_handle)
                .cloned()
                .unwrap_or_else(|| Signal::new(String::new()));
            let focused = registry
                .signal_bool(d.focused_handle)
                .cloned()
                .unwrap_or_else(|| Signal::new(false));

            let mut ti = text_input(value, focused, Signal::new(0))
                .placeholder(placeholder)
                .font_size(d.font_size)
                .bg(ccolor(d.bg_color))
                .text_color(ccolor(d.text_color))
                .border_color(ccolor(d.border_color))
                .radius(d.corner_radius);

            if d.width != 0.0 { ti = ti.width(d.width); }

            if d.has_on_submit != 0 {
                let cb = d.on_submit;
                ti = ti.on_submit(move |s| {
                    let cs = std::ffi::CString::new(s).unwrap_or_default();
                    (cb.fn_ptr)(cb.data, cs.as_ptr());
                });
            }
            ti.into()
        }

        CViewTag::Flexible => {
            let d = &*(desc.data as *const CFlexibleData);
            let child = cdesc_to_view(d.child, registry, free_node, free_str);
            flexible(child)
        }
    }
}
