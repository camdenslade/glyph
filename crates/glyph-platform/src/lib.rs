//! Platform event loop and windowing via winit + wgpu.
//!
//! `App::run` opens an 800x600 window with the default light theme.
//! Use `App::run_with_theme` to supply a custom `Theme`.

mod app;

pub use app::{App, WindowOpener};
#[cfg(feature = "hot-reload")]
pub use app::HotApp;
pub use glyph_core::Theme;
