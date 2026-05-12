/// `HotLoader` — watches a source directory, recompiles the guest crate on
/// change, and hot-swaps the dylib without restarting the host process.
///
/// State (Signals) lives in the host's `SignalRegistry` and survives reloads.
/// The guest state pointer (`glyph_create_state` return value) is also kept
/// across reloads because the guest's state layout is not expected to change in
/// a way that invalidates existing allocations during development. If the guest
/// needs to reset state it can expose a separate reset function.
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::os::raw::c_void;

use libloading::Library;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use core_glyph::{Theme, View};

use crate::abi::{
    CTheme, CColor, FnBuildView, FnCreateState, FnDestroyState, FnFreeNode,
    FnFreeStr, GlyphSignalTable,
    SYM_BUILD_VIEW, SYM_CREATE_STATE, SYM_DESTROY_STATE, SYM_FREE_NODE, SYM_FREE_STR,
};
use crate::convert::{SignalRegistry, build_signal_table, cdesc_to_view, set_active_registry};

fn theme_to_c(t: &Theme) -> CTheme {
    fn cc(c: core_glyph::Color) -> CColor {
        CColor { r: c.r, g: c.g, b: c.b, a: c.a }
    }
    CTheme {
        background:    cc(t.background),
        surface:       cc(t.surface),
        primary:       cc(t.primary),
        on_primary:    cc(t.on_primary),
        text:          cc(t.text),
        text_muted:    cc(t.text_muted),
        border:        cc(t.border),
        border_focused: cc(t.border_focused),
        radius:        t.radius,
        font_size:     t.font_size,
    }
}

struct GuestSymbols {
    create_state:  FnCreateState,
    build_view:    FnBuildView,
    destroy_state: FnDestroyState,
    free_node:     FnFreeNode,
    free_str:      FnFreeStr,
}

pub struct HotLoader {
    lib:            Option<Library>,
    syms:           Option<GuestSymbols>,
    lib_path:       PathBuf,
    package_name:   String,
    guest_state:    *mut c_void,
    registry:       Box<SignalRegistry>,
    signal_table:   Box<GlyphSignalTable>,
    _watcher:       RecommendedWatcher,
    dirty:          Arc<AtomicBool>,
    /// Set to true when a background rebuild completes successfully.
    rebuild_done:   Arc<AtomicBool>,
    /// Used to abort an in-flight rebuild if another change arrives.
    build_lock:     Arc<Mutex<()>>,
}

unsafe impl Send for HotLoader {}
unsafe impl Sync for HotLoader {}

impl HotLoader {
    /// Create a loader that watches `src_dir` for `.rs` changes and recompiles
    /// `package_name` (the `--package` argument to cargo).
    pub fn new(src_dir: &Path, lib_path: &Path, package_name: &str) -> Self {
        let dirty = Arc::new(AtomicBool::new(false));
        let rebuild_done = Arc::new(AtomicBool::new(false));

        let dirty2 = Arc::clone(&dirty);
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
                        let is_rs = event.paths.iter().any(|p| {
                            p.extension().is_some_and(|e| e == "rs")
                        });
                        if is_rs {
                            dirty2.store(true, Ordering::Relaxed);
                        }
                    }
                    _ => {}
                }
            }
        })
        .expect("failed to create file watcher");

        watcher
            .watch(src_dir, RecursiveMode::Recursive)
            .expect("failed to watch src dir");

        let mut registry = Box::new(SignalRegistry::new());
        set_active_registry(&mut *registry as *mut SignalRegistry);

        let signal_table = Box::new(build_signal_table());

        let mut loader = Self {
            lib: None,
            syms: None,
            lib_path: lib_path.to_owned(),
            package_name: package_name.to_owned(),
            guest_state: std::ptr::null_mut(),
            registry,
            signal_table,
            _watcher: watcher,
            dirty,
            rebuild_done,
            build_lock: Arc::new(Mutex::new(())),
        };

        // Do an initial load — the dylib must already be built before calling run_hot.
        loader.load_lib();
        loader
    }

    /// Call once per event loop tick (e.g. in `AboutToWait`).
    /// Returns true if a reload just completed and the window should redraw.
    pub fn poll_reload(&mut self) -> bool {
        if self.rebuild_done.swap(false, Ordering::Relaxed) {
            self.load_lib();
            return true;
        }

        if self.dirty.swap(false, Ordering::Relaxed) {
            self.kick_rebuild();
        }

        false
    }

    /// Build the view tree for the current frame.
    pub fn build_view(&mut self, theme: &Theme) -> Option<View> {
        let syms = self.syms.as_ref()?;
        if self.guest_state.is_null() { return None; }

        let ctheme = theme_to_c(theme);
        set_active_registry(&mut *self.registry as *mut SignalRegistry);

        let desc_ptr = unsafe { (syms.build_view)(self.guest_state, &ctheme) };
        if desc_ptr.is_null() { return None; }

        let view = unsafe {
            cdesc_to_view(desc_ptr, &self.registry, syms.free_node, syms.free_str)
        };

        // Free the top-level CViewDesc node (children were consumed during conversion).
        unsafe { (syms.free_node)(desc_ptr) };

        Some(view)
    }

    fn load_lib(&mut self) {
        // Drop the old library *before* opening the new one so the OS can
        // unmap the old pages. On macOS dlclose is a no-op for dylibs that
        // still have open symbol references, so we set syms to None first.
        self.syms = None;
        if let Some(old) = self.lib.take() {
            drop(old);
        }

        let lib = match unsafe { Library::new(&self.lib_path) } {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[glyph-hot] failed to load {:?}: {e}", self.lib_path);
                return;
            }
        };

        let syms = unsafe {
            macro_rules! sym {
                ($name:expr, $ty:ty) => {
                    match lib.get::<$ty>($name) {
                        Ok(s) => *s,
                        Err(e) => {
                            eprintln!("[glyph-hot] missing symbol: {e}");
                            return;
                        }
                    }
                };
            }
            GuestSymbols {
                create_state:  sym!(SYM_CREATE_STATE,  FnCreateState),
                build_view:    sym!(SYM_BUILD_VIEW,    FnBuildView),
                destroy_state: sym!(SYM_DESTROY_STATE, FnDestroyState),
                free_node:     sym!(SYM_FREE_NODE,     FnFreeNode),
                free_str:      sym!(SYM_FREE_STR,      FnFreeStr),
            }
        };

        // Only call create_state on the first load. On reloads we reuse the
        // existing guest_state pointer — the memory it points to is still valid
        // because the host's allocator owns it (via Box::into_raw in the guest).
        if self.guest_state.is_null() {
            set_active_registry(&mut *self.registry as *mut SignalRegistry);
            self.guest_state = unsafe {
                (syms.create_state)(self.signal_table.as_mut() as *mut GlyphSignalTable)
            };
        }

        self.syms = Some(syms);
        self.lib = Some(lib);
        eprintln!("[glyph-hot] loaded {:?}", self.lib_path);
    }

    fn kick_rebuild(&self) {
        let pkg = self.package_name.clone();
        let done = Arc::clone(&self.rebuild_done);
        let lock = Arc::clone(&self.build_lock);

        std::thread::spawn(move || {
            let _guard = lock.lock().unwrap();
            eprintln!("[glyph-hot] rebuilding {pkg}...");
            let status = std::process::Command::new("cargo")
                .args(["build", "--package", &pkg, "--lib"])
                .status();
            match status {
                Ok(s) if s.success() => {
                    eprintln!("[glyph-hot] rebuild OK");
                    done.store(true, Ordering::Relaxed);
                }
                Ok(s) => eprintln!("[glyph-hot] cargo exited with {s}"),
                Err(e) => eprintln!("[glyph-hot] failed to run cargo: {e}"),
            }
        });
    }
}

impl Drop for HotLoader {
    fn drop(&mut self) {
        if let (Some(syms), s) = (self.syms.take(), self.guest_state) {
            if !s.is_null() {
                unsafe { (syms.destroy_state)(s) };
            }
        }
    }
}
