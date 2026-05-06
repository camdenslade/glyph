//! Platform event loop and windowing via winit + wgpu.
//!
//! `App::run` is the entry point. It opens an 800x600 window, initializes the
//! GPU renderer, and drives the render loop. On each left-click it rebuilds
//! the view tree to hit-test buttons; on `RedrawRequested` it re-renders.

mod app;

pub use app::App;
