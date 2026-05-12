/// `glyph_guest!` — generates the three C-ABI exports a guest cdylib must
/// expose so the host's `HotLoader` can load and drive it.
///
/// # Usage
///
/// ```rust
/// // In your guest cdylib crate (e.g. glyph-demo with crate-type = ["cdylib"]):
/// use hot_glyph::guest::glyph_guest;
///
/// struct MyApp { /* Signal handles from GlyphSignalTable */ }
///
/// impl MyApp {
///     pub fn new(signals: &hot_glyph::abi::GlyphSignalTable) -> Self { ... }
/// }
///
/// impl core_glyph::Component for MyApp {
///     fn render(&self, theme: &core_glyph::Theme) -> core_glyph::View { ... }
/// }
///
/// glyph_guest!(MyApp);
/// ```
///
/// The macro expands to:
/// - `glyph_create_state`  — calls `MyApp::new(signals)`, boxes it, leaks the box
/// - `glyph_build_view`    — calls `app.render(theme)`, serializes to `CViewDesc`
/// - `glyph_destroy_state` — reconstructs the box and drops it
/// - `hot_glyph_free_node` — frees a `CViewDesc` node allocated by the guest
/// - `hot_glyph_free_str`  — frees a C string allocated by the guest
///
/// `glyph_build_view` is the hot path called every frame. It converts the
/// `*const CTheme` into a `core_glyph::Theme`, calls `render`, then serializes
/// the returned `View` tree into a `CViewDesc` tree on the heap. The host reads
/// this tree, converts it back to a native `View`, then calls `hot_glyph_free_node`
/// to release the guest allocation.
#[allow(clippy::empty_line_after_doc_comments)]
#[macro_export]
macro_rules! glyph_guest {
    ($App:ty) => {
        mod __glyph_guest_impl {
            use super::*;
            use ::core_glyph::{Color, Component, FontWeight, Shadow, Signal, Theme, View};
            use ::hot_glyph::abi::*;
            use ::std::ffi::{CStr, CString};
            use ::std::os::raw::{c_char, c_void};

            // ----------------------------------------------------------------
            // Theme conversion
            // ----------------------------------------------------------------

            fn from_ccolor(c: CColor) -> Color {
                Color::rgba(c.r, c.g, c.b, c.a)
            }

            fn to_ccolor(c: Color) -> CColor {
                CColor {
                    r: c.r,
                    g: c.g,
                    b: c.b,
                    a: c.a,
                }
            }

            fn to_cshadow(s: Shadow) -> CShadow {
                CShadow {
                    offset_x: s.offset_x,
                    offset_y: s.offset_y,
                    blur: s.blur,
                    color: to_ccolor(s.color),
                }
            }

            fn ctheme_to_theme(ct: &CTheme) -> Theme {
                Theme {
                    background: from_ccolor(ct.background),
                    surface: from_ccolor(ct.surface),
                    primary: from_ccolor(ct.primary),
                    on_primary: from_ccolor(ct.on_primary),
                    text: from_ccolor(ct.text),
                    text_muted: from_ccolor(ct.text_muted),
                    border: from_ccolor(ct.border),
                    border_focused: from_ccolor(ct.border_focused),
                    radius: ct.radius,
                    font_size: ct.font_size,
                }
            }

            // ----------------------------------------------------------------
            // View → CViewDesc serialization
            // ----------------------------------------------------------------

            unsafe fn alloc_str(s: &str) -> *mut c_char {
                let cs = CString::new(s).unwrap_or_default();
                let len = cs.as_bytes_with_nul().len();
                let buf = ::std::alloc::alloc(::std::alloc::Layout::array::<u8>(len).unwrap())
                    as *mut c_char;
                ::std::ptr::copy_nonoverlapping(cs.as_ptr(), buf, len);
                buf
            }

            unsafe fn alloc_node(tag: CViewTag, data: *mut c_void) -> *mut CViewDesc {
                let node = Box::new(CViewDesc {
                    tag,
                    _pad: [0; 4],
                    data,
                });
                Box::into_raw(node)
            }

            unsafe fn serialize_children(views: Vec<View>, table: &GlyphSignalTable) -> CChildren {
                let mut ptrs: Vec<*mut CViewDesc> = views
                    .into_iter()
                    .map(|v| serialize_view(v, table))
                    .collect();
                ptrs.shrink_to_fit();
                let len = ptrs.len();
                let ptr = ptrs.as_mut_ptr();
                ::std::mem::forget(ptrs);

                // flatten pointer-of-pointers into a flat CViewDesc array
                // (each element is moved by value so we can free the outer pointer layer)
                let flat = ::std::alloc::alloc(
                    ::std::alloc::Layout::array::<CViewDesc>(len.max(1)).unwrap(),
                ) as *mut CViewDesc;
                for i in 0..len {
                    let child_ptr = *ptr.add(i);
                    ::std::ptr::copy_nonoverlapping(child_ptr, flat.add(i), 1);
                    // free the box shell (not the data — it was moved into flat)
                    let _ = Box::from_raw(child_ptr);
                }
                ::std::alloc::dealloc(
                    ptr as *mut u8,
                    ::std::alloc::Layout::array::<*mut CViewDesc>(len.max(1)).unwrap(),
                );

                CChildren { ptr: flat, len }
            }

            unsafe fn serialize_view(view: View, table: &GlyphSignalTable) -> *mut CViewDesc {
                // Shared noop callbacks for optional fields.
                extern "C" fn noop_free(_: *mut c_void) {}
                extern "C" fn noop_hover(_: *mut c_void, _: u8) {}
                extern "C" fn noop_submit(_: *mut c_void, _: *const c_char) {}

                match view {
                    View::Spacer => alloc_node(CViewTag::Spacer, ::std::ptr::null_mut()),

                    View::Rect { color, style } => {
                        let d = Box::new(CRectData {
                            color: to_ccolor(color),
                            width: match style.size.width {
                                ::taffy::Dimension::Length(l) => l,
                                _ => 0.0,
                            },
                            height: match style.size.height {
                                ::taffy::Dimension::Length(l) => l,
                                _ => 0.0,
                            },
                        });
                        alloc_node(CViewTag::Rect, Box::into_raw(d) as *mut c_void)
                    }

                    View::Text {
                        content,
                        font_size,
                        color,
                        weight,
                        align,
                        wrap,
                        style,
                    } => {
                        let d = Box::new(CTextData {
                            content: alloc_str(&content),
                            font_size,
                            color: to_ccolor(color),
                            weight: if weight == FontWeight::Bold { 1 } else { 0 },
                            align: match align {
                                ::core_glyph::TextAlign::Center => 1,
                                ::core_glyph::TextAlign::Right => 2,
                                _ => 0,
                            },
                            wrap: wrap as u8,
                            _pad: 0,
                            max_width: match style.size.width {
                                ::taffy::Dimension::Length(l) => l,
                                _ => 0.0,
                            },
                        });
                        alloc_node(CViewTag::Text, Box::into_raw(d) as *mut c_void)
                    }

                    View::Button {
                        label,
                        on_click,
                        on_hover,
                        bg_color,
                        hover_bg_color,
                        text_color,
                        corner_radius,
                        font_size,
                        wrap,
                        ..
                    } => {
                        // Leak the closure so the host can call it later.
                        let click_data = Box::into_raw(Box::new(on_click)) as *mut c_void;
                        extern "C" fn call_click(data: *mut c_void) {
                            unsafe { (*(data as *mut Box<dyn Fn()>))() }
                        }
                        extern "C" fn free_click(data: *mut c_void) {
                            unsafe { drop(Box::from_raw(data as *mut Box<dyn Fn()>)) }
                        }

                        extern "C" fn call_hover(data: *mut c_void, val: u8) {
                            unsafe { (*(data as *mut Box<dyn Fn(bool)>))(val != 0) }
                        }
                        extern "C" fn free_hover(data: *mut c_void) {
                            unsafe { drop(Box::from_raw(data as *mut Box<dyn Fn(bool)>)) }
                        }
                        let (has_hover, hover_cb) = if let Some(f) = on_hover {
                            let hdata = Box::into_raw(Box::new(f)) as *mut c_void;
                            (
                                1u8,
                                CCallback1Bool {
                                    fn_ptr: call_hover,
                                    free_fn: free_hover,
                                    data: hdata,
                                },
                            )
                        } else {
                            (
                                0u8,
                                CCallback1Bool {
                                    fn_ptr: noop_hover,
                                    free_fn: noop_free,
                                    data: ::std::ptr::null_mut(),
                                },
                            )
                        };

                        let (has_hover_bg, hbg) = if let Some(c) = hover_bg_color {
                            (1u8, to_ccolor(c))
                        } else {
                            (
                                0u8,
                                CColor {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.0,
                                },
                            )
                        };

                        let d = Box::new(CButtonData {
                            label: alloc_str(&label),
                            on_click: CCallback0 {
                                fn_ptr: call_click,
                                free_fn: free_click,
                                data: click_data,
                            },
                            has_on_hover: has_hover,
                            _pad: [0; 7],
                            on_hover: hover_cb,
                            bg_color: to_ccolor(bg_color),
                            has_hover_bg,
                            _pad2: [0; 7],
                            hover_bg_color: hbg,
                            text_color: to_ccolor(text_color),
                            corner_radius,
                            font_size,
                            wrap: wrap as u8,
                            _pad3: [0; 3],
                        });
                        alloc_node(CViewTag::Button, Box::into_raw(d) as *mut c_void)
                    }

                    View::Column {
                        children,
                        style,
                        bg_color,
                        border_color,
                        border_width,
                        corner_radius,
                        shadow,
                        clip,
                    } => {
                        let kids = serialize_children(children, table);
                        let (has_bg, bgc) = bg_color.map_or(
                            (
                                0,
                                CColor {
                                    r: 0.,
                                    g: 0.,
                                    b: 0.,
                                    a: 0.,
                                },
                            ),
                            |c| (1, to_ccolor(c)),
                        );
                        let (has_border, bdc) = border_color.map_or(
                            (
                                0,
                                CColor {
                                    r: 0.,
                                    g: 0.,
                                    b: 0.,
                                    a: 0.,
                                },
                            ),
                            |c| (1, to_ccolor(c)),
                        );
                        let (has_shadow, sh) = shadow.map_or(
                            (
                                0,
                                CShadow {
                                    offset_x: 0.,
                                    offset_y: 0.,
                                    blur: 0.,
                                    color: CColor {
                                        r: 0.,
                                        g: 0.,
                                        b: 0.,
                                        a: 0.,
                                    },
                                },
                            ),
                            |s| (1, to_cshadow(s)),
                        );
                        let grow = style.flex_grow;
                        let width = match style.size.width {
                            ::taffy::Dimension::Length(l) => l,
                            _ => 0.0,
                        };
                        let gap = match style.gap.height {
                            ::taffy::LengthPercentage::Length(l) => l,
                            _ => 0.0,
                        };
                        let padding = match style.padding.top {
                            ::taffy::LengthPercentage::Length(l) => l,
                            _ => 0.0,
                        };
                        let d = Box::new(CContainerData {
                            children: kids,
                            gap,
                            padding,
                            align_items: 1,
                            justify: 1,
                            has_bg,
                            has_border,
                            has_shadow,
                            clip: clip as u8,
                            _pad: [0; 2],
                            bg_color: bgc,
                            border_color: bdc,
                            border_width,
                            corner_radius,
                            shadow: sh,
                            width,
                            height: 0.0,
                            grow,
                        });
                        alloc_node(CViewTag::Column, Box::into_raw(d) as *mut c_void)
                    }

                    View::Row {
                        children,
                        style,
                        bg_color,
                        border_color,
                        border_width,
                        corner_radius,
                        shadow,
                        clip,
                    } => {
                        let kids = serialize_children(children, table);
                        let (has_bg, bgc) = bg_color.map_or(
                            (
                                0,
                                CColor {
                                    r: 0.,
                                    g: 0.,
                                    b: 0.,
                                    a: 0.,
                                },
                            ),
                            |c| (1, to_ccolor(c)),
                        );
                        let (has_border, bdc) = border_color.map_or(
                            (
                                0,
                                CColor {
                                    r: 0.,
                                    g: 0.,
                                    b: 0.,
                                    a: 0.,
                                },
                            ),
                            |c| (1, to_ccolor(c)),
                        );
                        let (has_shadow, sh) = shadow.map_or(
                            (
                                0,
                                CShadow {
                                    offset_x: 0.,
                                    offset_y: 0.,
                                    blur: 0.,
                                    color: CColor {
                                        r: 0.,
                                        g: 0.,
                                        b: 0.,
                                        a: 0.,
                                    },
                                },
                            ),
                            |s| (1, to_cshadow(s)),
                        );
                        let grow = style.flex_grow;
                        let width = match style.size.width {
                            ::taffy::Dimension::Length(l) => l,
                            _ => 0.0,
                        };
                        let gap = match style.gap.width {
                            ::taffy::LengthPercentage::Length(l) => l,
                            _ => 0.0,
                        };
                        let padding = match style.padding.top {
                            ::taffy::LengthPercentage::Length(l) => l,
                            _ => 0.0,
                        };
                        let d = Box::new(CContainerData {
                            children: kids,
                            gap,
                            padding,
                            align_items: 1,
                            justify: 1,
                            has_bg,
                            has_border,
                            has_shadow,
                            clip: clip as u8,
                            _pad: [0; 2],
                            bg_color: bgc,
                            border_color: bdc,
                            border_width,
                            corner_radius,
                            shadow: sh,
                            width,
                            height: 0.0,
                            grow,
                        });
                        alloc_node(CViewTag::Row, Box::into_raw(d) as *mut c_void)
                    }

                    View::ZStack { children, .. } => {
                        let kids = serialize_children(children, table);
                        let d = Box::new(CZStackData { children: kids });
                        alloc_node(CViewTag::ZStack, Box::into_raw(d) as *mut c_void)
                    }

                    View::Image {
                        path,
                        corner_radius,
                        style,
                    } => {
                        let w = match style.size.width {
                            ::taffy::Dimension::Length(l) => l,
                            _ => 0.0,
                        };
                        let h = match style.size.height {
                            ::taffy::Dimension::Length(l) => l,
                            _ => 0.0,
                        };
                        let d = Box::new(CImageData {
                            path: alloc_str(&path),
                            corner_radius,
                            width: w,
                            height: h,
                        });
                        alloc_node(CViewTag::Image, Box::into_raw(d) as *mut c_void)
                    }

                    View::TextInput {
                        value,
                        focused,
                        placeholder,
                        font_size,
                        bg_color,
                        text_color,
                        border_color,
                        corner_radius,
                        on_submit,
                        style,
                    } => {
                        extern "C" fn call_submit(data: *mut c_void, val: *const c_char) {
                            let s = unsafe { CStr::from_ptr(val) }
                                .to_string_lossy()
                                .into_owned();
                            unsafe { (*(data as *mut Box<dyn Fn(String)>))(s) }
                        }
                        extern "C" fn free_submit(data: *mut c_void) {
                            unsafe { drop(Box::from_raw(data as *mut Box<dyn Fn(String)>)) }
                        }
                        let (has_submit, scb) = if let Some(f) = on_submit {
                            let sdata = Box::into_raw(Box::new(f)) as *mut c_void;
                            (
                                1u8,
                                CCallback1Str {
                                    fn_ptr: call_submit,
                                    free_fn: free_submit,
                                    data: sdata,
                                },
                            )
                        } else {
                            (
                                0u8,
                                CCallback1Str {
                                    fn_ptr: noop_submit,
                                    free_fn: noop_free,
                                    data: ::std::ptr::null_mut(),
                                },
                            )
                        };

                        // Pass back signal pointers as opaque handles. as_raw_arc increments
                        // the refcount; the host's SignalRegistry already owns the canonical ref.
                        let value_handle = value.as_raw_arc() as *mut c_void;
                        let focused_handle = focused.as_raw_arc() as *mut c_void;

                        let w = match style.size.width {
                            ::taffy::Dimension::Length(l) => l,
                            _ => 0.0,
                        };
                        let h = match style.size.height {
                            ::taffy::Dimension::Length(l) => l,
                            _ => 0.0,
                        };
                        let d = Box::new(CTextInputData {
                            value_handle,
                            focused_handle,
                            placeholder: alloc_str(&placeholder),
                            font_size,
                            bg_color: to_ccolor(bg_color),
                            text_color: to_ccolor(text_color),
                            border_color: to_ccolor(border_color),
                            corner_radius,
                            has_on_submit: has_submit,
                            _pad: [0; 7],
                            on_submit: scb,
                            width: w,
                            height: h,
                        });
                        alloc_node(CViewTag::TextInput, Box::into_raw(d) as *mut c_void)
                    }

                    View::Scroll {
                        child,
                        offset_x,
                        offset_y,
                        style,
                    } => {
                        let child_ptr = serialize_view(*child, table);
                        let w = match style.size.width {
                            ::taffy::Dimension::Length(l) => l,
                            _ => 0.0,
                        };
                        let h = match style.size.height {
                            ::taffy::Dimension::Length(l) => l,
                            _ => 0.0,
                        };
                        let ox_handle = offset_x.as_raw_arc() as *mut c_void;
                        let oy_handle = offset_y.as_raw_arc() as *mut c_void;
                        let d = Box::new(CScrollData {
                            child: child_ptr,
                            offset_x_handle: ox_handle,
                            offset_y_handle: oy_handle,
                            width: w,
                            height: h,
                        });
                        alloc_node(CViewTag::Scroll, Box::into_raw(d) as *mut c_void)
                    }

                    View::Flexible {
                        child,
                        grow,
                        shrink,
                    } => {
                        let child_ptr = serialize_view(*child, table);
                        let d = Box::new(CFlexibleData {
                            child: child_ptr,
                            grow,
                            shrink,
                        });
                        alloc_node(CViewTag::Flexible, Box::into_raw(d) as *mut c_void)
                    }

                    View::Component(c) => {
                        // Expand inline — components are transparent at the ABI boundary.
                        // We need a theme to expand; use a dummy light theme since we
                        // can't reach the real one here. Components should not be
                        // returned from top-level render in guest code.
                        let theme = Theme::light();
                        let rendered = c.render(&theme);
                        serialize_view(rendered, table)
                    }
                }
            }

            // ----------------------------------------------------------------
            // Free helpers exported to host
            // ----------------------------------------------------------------

            #[no_mangle]
            pub unsafe extern "C" fn hot_glyph_free_node(node: *mut CViewDesc) {
                if node.is_null() {
                    return;
                }
                // data pointer was allocated as a Box<C*Data> — reconstruct and drop.
                // We don't recurse here; the host is responsible for freeing children
                // before freeing the parent, which `cdesc_to_view` ensures by consuming
                // children during conversion.
                let desc = &*node;
                if !desc.data.is_null() {
                    match desc.tag {
                        CViewTag::Text => drop(Box::from_raw(desc.data as *mut CTextData)),
                        CViewTag::Button => drop(Box::from_raw(desc.data as *mut CButtonData)),
                        CViewTag::Column | CViewTag::Row => {
                            let d = Box::from_raw(desc.data as *mut CContainerData);
                            if d.children.len > 0 {
                                ::std::alloc::dealloc(
                                    d.children.ptr as *mut u8,
                                    ::std::alloc::Layout::array::<CViewDesc>(d.children.len)
                                        .unwrap(),
                                );
                            }
                        }
                        CViewTag::ZStack => {
                            let d = Box::from_raw(desc.data as *mut CZStackData);
                            if d.children.len > 0 {
                                ::std::alloc::dealloc(
                                    d.children.ptr as *mut u8,
                                    ::std::alloc::Layout::array::<CViewDesc>(d.children.len)
                                        .unwrap(),
                                );
                            }
                        }
                        CViewTag::Scroll => drop(Box::from_raw(desc.data as *mut CScrollData)),
                        CViewTag::Image => drop(Box::from_raw(desc.data as *mut CImageData)),
                        CViewTag::TextInput => {
                            drop(Box::from_raw(desc.data as *mut CTextInputData))
                        }
                        CViewTag::Rect => drop(Box::from_raw(desc.data as *mut CRectData)),
                        CViewTag::Flexible => drop(Box::from_raw(desc.data as *mut CFlexibleData)),
                        CViewTag::Spacer => {}
                    }
                }
                drop(Box::from_raw(node));
            }

            #[no_mangle]
            pub unsafe extern "C" fn hot_glyph_free_str(s: *mut c_char) {
                if s.is_null() {
                    return;
                }
                let len = ::std::ffi::CStr::from_ptr(s).to_bytes_with_nul().len();
                ::std::alloc::dealloc(
                    s as *mut u8,
                    ::std::alloc::Layout::array::<u8>(len).unwrap(),
                );
            }

            // ----------------------------------------------------------------
            // The three required guest exports
            // ----------------------------------------------------------------

            static mut SIGNAL_TABLE_PTR: *mut GlyphSignalTable = ::std::ptr::null_mut();

            #[no_mangle]
            pub unsafe extern "C" fn glyph_create_state(
                signals: *mut GlyphSignalTable,
            ) -> *mut c_void {
                SIGNAL_TABLE_PTR = signals;
                let app = <$App>::new(&*signals);
                Box::into_raw(Box::new(app)) as *mut c_void
            }

            #[no_mangle]
            pub unsafe extern "C" fn glyph_build_view(
                state: *mut c_void,
                theme: *const CTheme,
            ) -> *mut CViewDesc {
                let app = &*(state as *const $App);
                let t = ctheme_to_theme(&*theme);
                let view = app.render(&t);
                let table = &*SIGNAL_TABLE_PTR;
                serialize_view(view, table)
            }

            #[no_mangle]
            pub unsafe extern "C" fn glyph_destroy_state(state: *mut c_void) {
                drop(Box::from_raw(state as *mut $App));
            }
        }
    };
}

pub use glyph_guest;
