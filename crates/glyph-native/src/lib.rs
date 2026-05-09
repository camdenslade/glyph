//! Native platform bridge (macOS only).
//!
//! `NativeApp::run` converts a `View` tree into a live AppKit view hierarchy
//! using objc2 bindings instead of the GPU renderer. Button callbacks are
//! stored as Rust closures behind a custom Objective-C `ActionTarget` class.

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::NativeApp;
