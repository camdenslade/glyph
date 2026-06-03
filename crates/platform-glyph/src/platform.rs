//! Native OS integration: clipboard, file dialogs, notifications.
//! These are thin wrappers so the glyph crate can expose a clean API
//! without developers importing rfd or arboard directly.


/// Read plain text from the system clipboard. Returns `None` if the clipboard
/// is empty or contains non-text content.
pub fn clipboard_read() -> Option<String> {
    arboard::Clipboard::new()
        .ok()
        .and_then(|mut cb| cb.get_text().ok())
}

/// Write plain text to the system clipboard.
pub fn clipboard_write(text: impl Into<String>) {
    if let Ok(mut cb) = arboard::Clipboard::new() {
        let _ = cb.set_text(text.into());
    }
}


use std::path::PathBuf;

/// Open a native "pick file" dialog. Blocks until the user picks or cancels.
/// Returns `None` if cancelled.
pub fn pick_file() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_file()
}

/// Open a native "pick files" dialog (multi-select).
pub fn pick_files() -> Vec<PathBuf> {
    rfd::FileDialog::new().pick_files().unwrap_or_default()
}

/// Open a native "pick folder" dialog.
pub fn pick_folder() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_folder()
}

/// Open a native "save file" dialog. Returns the chosen path or `None`.
pub fn save_file(default_name: impl Into<String>) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_file_name(default_name.into())
        .save_file()
}

/// Like `pick_file` but pre-filters to the given extensions (e.g. `&["png", "jpg"]`).
pub fn pick_file_filtered(title: &str, extensions: &[&str]) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title(title)
        .add_filter("Files", extensions)
        .pick_file()
}


/// Show a native OS notification. On macOS this posts to Notification Center;
/// on Windows it uses the system tray notification area.
/// Requires `notify-rust` — currently a no-op stub until the dep is added.
pub fn notify(title: impl Into<String>, body: impl Into<String>) {
    // Stubbed — add `notify-rust = "4"` to Cargo.toml to activate.
    let _ = (title.into(), body.into());
}


/// Open a URL in the default system browser.
pub fn open_url(url: impl Into<String>) {
    let url = url.into();
    #[cfg(target_os = "macos")]
    { let _ = std::process::Command::new("open").arg(&url).spawn(); }
    #[cfg(target_os = "windows")]
    { let _ = std::process::Command::new("cmd").args(["/C", "start", &url]).spawn(); }
    #[cfg(target_os = "linux")]
    { let _ = std::process::Command::new("xdg-open").arg(&url).spawn(); }
}

/// Reveal a file path in Finder / Explorer.
pub fn reveal_in_explorer(path: impl Into<std::path::PathBuf>) {
    let path = path.into();
    #[cfg(target_os = "macos")]
    { let _ = std::process::Command::new("open").arg("-R").arg(&path).spawn(); }
    #[cfg(target_os = "windows")]
    { let _ = std::process::Command::new("explorer").arg(format!("/select,{}", path.display())).spawn(); }
    #[cfg(target_os = "linux")]
    { let _ = std::process::Command::new("xdg-open").arg(path.parent().unwrap_or(&path)).spawn(); }
}
