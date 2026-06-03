//! Platform event loop and windowing via winit + wgpu.
//!
//! `App::run` opens an 800x600 window with the default light theme.
//! Use `App::run_with_theme` to supply a custom `Theme`.

mod app;
pub mod menu;
pub mod platform;

pub use app::{App, AppBuilder, WindowCloser, WindowOpener};
#[cfg(feature = "hot-reload")]
pub use app::HotApp;
pub use core_glyph::Theme;

// Convenience re-exports so callers can do `platform_glyph::clipboard_read()` etc.
pub use platform::{
    clipboard_read, clipboard_write,
    open_url, reveal_in_explorer,
    pick_file, pick_files, pick_folder, pick_file_filtered, save_file,
    notify,
};

pub use menu::MenuBar;
