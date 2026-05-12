/// Guest-side hot-reload entry point for glyph-demo.
///
/// This module is only compiled when the `hot-reload` feature is enabled.
/// It exposes `DemoApp` with a `new(signals)` constructor that allocates
/// signals via the host's `GlyphSignalTable` instead of `Signal::new`.
///
/// The regular `main.rs` binary is unaffected.
#[cfg(feature = "hot-reload")]
mod hot {
    use core_glyph::{Component, FontWeight, Theme, View, button, column, image, row, text, text_input};
    use hot_glyph::abi::GlyphSignalTable;
    use std::os::raw::{c_char, c_void};
    use std::ffi::CString;

    #[derive(Clone, Copy)]
    struct HotSignalI32 {
        handle: *mut c_void,
        table: *const GlyphSignalTable,
    }

    unsafe impl Send for HotSignalI32 {}
    unsafe impl Sync for HotSignalI32 {}

    impl HotSignalI32 {
        fn get(&self) -> i32 {
            unsafe { ((*self.table).get_i32)(self.handle) }
        }
        fn set(&self, v: i32) {
            unsafe { ((*self.table).set_i32)(self.handle, v) }
        }
    }

    #[derive(Clone, Copy)]
    struct HotSignalF32 {
        handle: *mut c_void,
        table: *const GlyphSignalTable,
    }

    unsafe impl Send for HotSignalF32 {}
    unsafe impl Sync for HotSignalF32 {}

    impl HotSignalF32 {
        fn get(&self) -> f32 {
            unsafe { ((*self.table).get_f32)(self.handle) }
        }
        #[allow(dead_code)]
        fn set(&self, v: f32) {
            unsafe { ((*self.table).set_f32)(self.handle, v) }
        }
    }

    #[derive(Clone, Copy)]
    struct HotSignalBool {
        handle: *mut c_void,
        table: *const GlyphSignalTable,
    }

    unsafe impl Send for HotSignalBool {}
    unsafe impl Sync for HotSignalBool {}

    impl HotSignalBool {
        fn get(&self) -> bool {
            unsafe { ((*self.table).get_bool)(self.handle) != 0 }
        }
        fn set(&self, v: bool) {
            unsafe { ((*self.table).set_bool)(self.handle, v as u8) }
        }
    }

    #[derive(Clone, Copy)]
    struct HotSignalStr {
        handle: *mut c_void,
        table: *const GlyphSignalTable,
    }

    unsafe impl Send for HotSignalStr {}
    unsafe impl Sync for HotSignalStr {}

    impl HotSignalStr {
        fn get(&self) -> String {
            let mut buf = vec![0u8; 4096];
            let n = unsafe {
                ((*self.table).get_str)(self.handle, buf.as_mut_ptr() as *mut c_char, buf.len())
            };
            String::from_utf8_lossy(&buf[..n]).into_owned()
        }
        fn set(&self, v: &str) {
            let cs = CString::new(v).unwrap_or_default();
            unsafe { ((*self.table).set_str)(self.handle, cs.as_ptr()) }
        }
    }

    pub struct HotDemoApp {
        count: HotSignalI32,
        search_value: HotSignalStr,
        search_focused: HotSignalBool,
        dark_mode: HotSignalBool,
        table: *const GlyphSignalTable,
    }

    unsafe impl Send for HotDemoApp {}
    unsafe impl Sync for HotDemoApp {}

    impl HotDemoApp {
        pub fn new(signals: &GlyphSignalTable) -> Self {
            let table = signals as *const GlyphSignalTable;
            let empty = CString::new("").unwrap();
            Self {
                count:          HotSignalI32  { handle: (signals.new_i32)(0),             table },
                search_value:   HotSignalStr  { handle: (signals.new_str)(empty.as_ptr()), table },
                search_focused: HotSignalBool { handle: (signals.new_bool)(0),            table },
                dark_mode:      HotSignalBool { handle: (signals.new_bool)(0),            table },
                table,
            }
        }
    }

    impl Component for HotDemoApp {
        fn render(&self, theme: &Theme) -> View {
            use core_glyph::Signal;

            let count_val = self.count.get();
            let count = self.count;
            let dark_mode = self.dark_mode;
            let label = if dark_mode.get() { "Switch to Light" } else { "Switch to Dark" };

            let search_val = self.search_value.get();
            let search_display = if search_val.is_empty() {
                "Nothing typed yet.".to_string()
            } else {
                format!("You typed: {search_val}")
            };

            // For text_input we need real Signal<String>/Signal<bool>.
            // The host owns these; we create lightweight Signal wrappers that
            // share the same underlying Arc as the host's SignalSlot.
            // Since HotSignalStr wraps an opaque pointer we can't do that here,
            // so we use a local Signal bridged via on_submit for the demo.
            // A full implementation would add Signal handle cloning to the table.
            let sv = Signal::new(self.search_value.get());
            let sf = Signal::new(self.search_focused.get());

            column(vec![
                image("Glyph.png").size(120.0, 120.0).radius(12.0).into(),

                row(vec![
                    text(format!("Count: {count_val}"), 32.0)
                        .weight(FontWeight::Bold)
                        .color(theme.text)
                        .into(),
                    button("Increment", move || count.set(count.get() + 1))
                        .bg(theme.primary)
                        .text_color(theme.on_primary)
                        .radius(theme.radius)
                        .into(),
                ]).into(),

                column(vec![
                    text_input(sv, sf, Signal::new(self.search_value.get().len()))
                        .placeholder("Type something...")
                        .bg(theme.surface)
                        .text_color(theme.text)
                        .border_color(theme.border)
                        .radius(theme.radius)
                        .font_size(theme.font_size)
                        .width(320.0)
                        .into(),
                    text(search_display, theme.font_size)
                        .color(theme.text_muted)
                        .into(),
                ]).into(),

                button(label, move || dark_mode.set(!dark_mode.get()))
                    .bg(theme.surface)
                    .text_color(theme.text)
                    .radius(theme.radius)
                    .into(),
            ]).into()
        }
    }

    hot_glyph::glyph_guest!(HotDemoApp);
}
