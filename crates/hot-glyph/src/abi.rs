/// C-ABI bridge types between the host process and a guest cdylib.
///
/// Everything in this file is `#[repr(C)]` so it survives a dlopen boundary.
/// The guest never touches `Signal<T>`, `Box<dyn Fn()>`, or `View` directly —
/// those are Rust-ABI types that only live in the host. The guest produces a
/// `CViewDesc` tree and receives opaque signal handles it reads/writes through
/// the function-pointer table (`GlyphSignalTable`).
// NOTE: Adding new variants to CViewTag is a breaking ABI change — guest and
// host must be rebuilt together. Keeping a version field in GlyphSignalTable
// (or a separate `hot_glyph_abi_version() -> u32` symbol) would let the loader
// detect mismatches and refuse to load rather than silently misinterpreting memory.
use std::os::raw::{c_char, c_void};

// POD mirrors of glyph-core types

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CShadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub color: CColor,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CTheme {
    pub background: CColor,
    pub surface: CColor,
    pub primary: CColor,
    pub on_primary: CColor,
    pub text: CColor,
    pub text_muted: CColor,
    pub border: CColor,
    pub border_focused: CColor,
    pub radius: f32,
    pub font_size: f32,
}

// Callback pairs — replace Box<dyn Fn()> and Box<dyn Fn(bool)>

/// Equivalent to `Box<dyn Fn()>`. The guest allocates `data` and the host
/// calls `fn_ptr(data)` on click. After the view is dropped the host calls
/// `free_fn(data)` to release guest memory.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CCallback0 {
    pub fn_ptr: extern "C" fn(data: *mut c_void),
    pub free_fn: extern "C" fn(data: *mut c_void),
    pub data: *mut c_void,
}

unsafe impl Send for CCallback0 {}
unsafe impl Sync for CCallback0 {}

/// Equivalent to `Box<dyn Fn(bool)>`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CCallback1Bool {
    pub fn_ptr: extern "C" fn(data: *mut c_void, val: u8),
    pub free_fn: extern "C" fn(data: *mut c_void),
    pub data: *mut c_void,
}

unsafe impl Send for CCallback1Bool {}
unsafe impl Sync for CCallback1Bool {}

/// Equivalent to `Box<dyn Fn(String)>`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CCallback1Str {
    pub fn_ptr: extern "C" fn(data: *mut c_void, val: *const c_char),
    pub free_fn: extern "C" fn(data: *mut c_void),
    pub data: *mut c_void,
}

unsafe impl Send for CCallback1Str {}
unsafe impl Sync for CCallback1Str {}

// CViewDesc — the C-safe view tree

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CViewTag {
    Text = 0,
    Button = 1,
    Column = 2,
    Row = 3,
    ZStack = 4,
    Scroll = 5,
    Image = 6,
    TextInput = 7,
    Rect = 8,
    Flexible = 9,
    Spacer = 10,
}

/// A heap-allocated, null-terminated UTF-8 string owned by the guest.
/// The host must call `hot_glyph_free_str` (exported by the guest) after use.
pub type CStr = *mut c_char;

/// An array of `CViewDesc` owned by the guest.
#[repr(C)]
pub struct CChildren {
    pub ptr: *mut CViewDesc,
    pub len: usize,
}

// Text
#[repr(C)]
pub struct CTextData {
    pub content: CStr,
    pub font_size: f32,
    pub color: CColor,
    /// 0 = Regular, 1 = Bold
    pub weight: u8,
    /// 0 = Left, 1 = Center, 2 = Right
    pub align: u8,
    /// 0 = no wrap, 1 = wrap
    pub wrap: u8,
    pub _pad: u8,
    pub max_width: f32, // 0.0 = unconstrained
}

// Button
#[repr(C)]
pub struct CButtonData {
    pub label: CStr,
    pub on_click: CCallback0,
    pub has_on_hover: u8,
    pub _pad: [u8; 7],
    pub on_hover: CCallback1Bool,
    pub bg_color: CColor,
    pub has_hover_bg: u8,
    pub _pad2: [u8; 7],
    pub hover_bg_color: CColor,
    pub text_color: CColor,
    pub corner_radius: f32,
    pub font_size: f32,
    /// 0 = no wrap, 1 = wrap
    pub wrap: u8,
    pub _pad3: [u8; 3],
}

// Column / Row — share the same data shape
#[repr(C)]
pub struct CContainerData {
    pub children: CChildren,
    pub gap: f32,
    pub padding: f32,
    pub align_items: u8, // 0=Start,1=Center,2=End,3=Stretch
    pub justify: u8,     // 0=Start,1=Center,2=End,3=SpaceBetween,4=SpaceAround,5=SpaceEvenly
    pub has_bg: u8,
    pub has_border: u8,
    pub has_shadow: u8,
    pub clip: u8,
    pub _pad: [u8; 2],
    pub bg_color: CColor,
    pub border_color: CColor,
    pub border_width: f32,
    pub corner_radius: f32,
    pub shadow: CShadow,
    pub width: f32, // 0.0 = percent(1.0) default; -1.0 = auto
    pub height: f32,
    pub grow: f32,
}

// ZStack
#[repr(C)]
pub struct CZStackData {
    pub children: CChildren,
}

// Scroll — signal handles are opaque host pointers passed back at create time
#[repr(C)]
pub struct CScrollData {
    pub child: *mut CViewDesc,
    pub offset_x_handle: *mut c_void,
    pub offset_y_handle: *mut c_void,
    pub width: f32,
    pub height: f32,
}

// Image
#[repr(C)]
pub struct CImageData {
    pub path: CStr,
    pub corner_radius: f32,
    pub width: f32,
    pub height: f32,
}

// TextInput
#[repr(C)]
pub struct CTextInputData {
    pub value_handle: *mut c_void,
    pub focused_handle: *mut c_void,
    pub placeholder: CStr,
    pub font_size: f32,
    pub bg_color: CColor,
    pub text_color: CColor,
    pub border_color: CColor,
    pub corner_radius: f32,
    pub has_on_submit: u8,
    pub _pad: [u8; 7],
    pub on_submit: CCallback1Str,
    pub width: f32,
    pub height: f32,
}

// Rect
#[repr(C)]
pub struct CRectData {
    pub color: CColor,
    pub width: f32,
    pub height: f32,
}

// Flexible wrapper
#[repr(C)]
pub struct CFlexibleData {
    pub child: *mut CViewDesc,
    pub grow: f32,
    pub shrink: f32,
}

/// Tagged union describing a single node in the view tree.
/// The `data` field is interpreted based on `tag`.
#[repr(C)]
pub struct CViewDesc {
    pub tag: CViewTag,
    pub _pad: [u8; 4],
    /// Pointer to one of the C*Data structs above, heap-allocated by the guest.
    /// The host reads it then calls the guest's `hot_glyph_free_node` to release.
    pub data: *mut c_void,
}

// Signal table — passed to glyph_create_state so the guest can access host signals

/// Function-pointer table the host fills in and passes to `glyph_create_state`.
/// The guest stores the pointers it needs; the host keeps the table alive for
/// the lifetime of the process.
#[repr(C)]
pub struct GlyphSignalTable {
    /// Allocate a new i32 signal on the host with initial value. Returns opaque handle.
    pub new_i32: extern "C" fn(initial: i32) -> *mut c_void,
    pub get_i32: extern "C" fn(handle: *mut c_void) -> i32,
    pub set_i32: extern "C" fn(handle: *mut c_void, val: i32),

    /// Allocate a new f32 signal on the host with initial value. Returns opaque handle.
    pub new_f32: extern "C" fn(initial: f32) -> *mut c_void,
    pub get_f32: extern "C" fn(handle: *mut c_void) -> f32,
    pub set_f32: extern "C" fn(handle: *mut c_void, val: f32),

    /// Allocate a new bool signal on the host. Returns opaque handle.
    pub new_bool: extern "C" fn(initial: u8) -> *mut c_void,
    pub get_bool: extern "C" fn(handle: *mut c_void) -> u8,
    pub set_bool: extern "C" fn(handle: *mut c_void, val: u8),

    /// Allocate a new String signal on the host. `initial` is a null-terminated
    /// UTF-8 string owned by the caller. Returns opaque handle.
    pub new_str: extern "C" fn(initial: *const c_char) -> *mut c_void,
    /// Write the current string value into `buf` (max `cap` bytes incl. NUL).
    /// Returns number of bytes written (not including NUL).
    pub get_str: extern "C" fn(handle: *mut c_void, buf: *mut c_char, cap: usize) -> usize,
    /// Set string value. `val` is a null-terminated UTF-8 string.
    pub set_str: extern "C" fn(handle: *mut c_void, val: *const c_char),
}

// Guest dylib symbol names (looked up by the host via libloading)

pub const SYM_CREATE_STATE: &[u8] = b"glyph_create_state\0";
pub const SYM_BUILD_VIEW: &[u8] = b"glyph_build_view\0";
pub const SYM_DESTROY_STATE: &[u8] = b"glyph_destroy_state\0";
pub const SYM_FREE_NODE: &[u8] = b"hot_glyph_free_node\0";
pub const SYM_FREE_STR: &[u8] = b"hot_glyph_free_str\0";

pub type FnCreateState = unsafe extern "C" fn(signals: *mut GlyphSignalTable) -> *mut c_void;
pub type FnBuildView =
    unsafe extern "C" fn(state: *mut c_void, theme: *const CTheme) -> *mut CViewDesc;
pub type FnDestroyState = unsafe extern "C" fn(state: *mut c_void);
pub type FnFreeNode = unsafe extern "C" fn(node: *mut CViewDesc);
pub type FnFreeStr = unsafe extern "C" fn(s: *mut c_char);
