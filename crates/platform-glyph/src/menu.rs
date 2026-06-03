//! Native OS menu bar via `muda`.
//!
//! Build a `MenuBar` with the declarative API, then pass it to `AppBuilder::menu()`.
//! On macOS it becomes the global app menu. On Windows it attaches to each window.
//!
//! See the `glyph` crate for a full usage example.

use muda::{
    accelerator::{Accelerator, Code, Modifiers},
    Menu, MenuItem, PredefinedMenuItem, Submenu,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


/// A declarative native menu bar definition. Pass to `AppBuilder::menu()`.
pub struct MenuBar {
    pub(crate) inner: Menu,
    pub(crate) handlers: Arc<Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>>>,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            inner: Menu::new(),
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a top-level submenu. The closure receives a `SubMenuBuilder` and must
    /// return it (call `.build()` or just return it — `Into<SubMenuBuilt>` is implemented).
    pub fn submenu(self, label: &str, build: impl FnOnce(SubMenuBuilder) -> SubMenuBuilder) -> Self {
        let handlers = Arc::clone(&self.handlers);
        let builder = SubMenuBuilder::new(label, handlers);
        let built = build(builder).finish();
        let _ = self.inner.append(&built.submenu);
        Self { inner: self.inner, handlers: built.handlers }
    }
}

impl Default for MenuBar {
    fn default() -> Self { Self::new() }
}


pub struct SubMenuBuilder {
    label: String,
    items: Vec<muda::MenuItemKind>,
    handlers: Arc<Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>>>,
}

pub(crate) struct SubMenuBuilt {
    pub submenu: Submenu,
    pub handlers: Arc<Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>>>,
}

impl SubMenuBuilder {
    fn new(label: &str, handlers: Arc<Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>>>) -> Self {
        Self { label: label.to_string(), items: vec![], handlers }
    }

    /// Add a regular menu item with an optional keyboard shortcut.
    /// Shortcut format: `"CmdOrCtrl+S"`, `"Alt+F4"`, `""` for none.
    pub fn item(mut self, label: &str, shortcut: &str, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        let accel = parse_shortcut(shortcut);
        let item = MenuItem::new(label, true, accel);
        let id = item.id().0.to_string();
        self.handlers.lock().unwrap().insert(id, Box::new(on_click));
        self.items.push(muda::MenuItemKind::MenuItem(item));
        self
    }

    /// Add a visual separator line.
    pub fn separator(mut self) -> Self {
        self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::separator()));
        self
    }

    // ── Predefined (OS-native) items ───────────────────────────────────────
    pub fn cut(mut self) -> Self        { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::cut(None))); self }
    pub fn copy(mut self) -> Self       { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::copy(None))); self }
    pub fn paste(mut self) -> Self      { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::paste(None))); self }
    pub fn select_all(mut self) -> Self { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::select_all(None))); self }
    pub fn undo(mut self) -> Self       { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::undo(None))); self }
    pub fn redo(mut self) -> Self       { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::redo(None))); self }
    pub fn minimize(mut self) -> Self   { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::minimize(None))); self }
    pub fn maximize(mut self) -> Self   { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::maximize(None))); self }
    pub fn fullscreen(mut self) -> Self { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::fullscreen(None))); self }
    pub fn close_window(mut self) -> Self { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::close_window(None))); self }
    pub fn quit(mut self) -> Self       { self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::quit(None))); self }

    pub fn about(mut self, app_name: &str) -> Self {
        self.items.push(muda::MenuItemKind::Predefined(PredefinedMenuItem::about(
            None,
            Some(muda::AboutMetadata { name: Some(app_name.to_string()), ..Default::default() }),
        )));
        self
    }

    /// Nest another submenu.
    pub fn submenu(mut self, label: &str, build: impl FnOnce(SubMenuBuilder) -> SubMenuBuilder) -> Self {
        let handlers = Arc::clone(&self.handlers);
        let builder = SubMenuBuilder::new(label, handlers);
        let built = build(builder).finish();
        self.items.push(muda::MenuItemKind::Submenu(built.submenu));
        self.handlers = built.handlers;
        self
    }

    pub(crate) fn finish(self) -> SubMenuBuilt {
        let refs: Vec<&dyn muda::IsMenuItem> = self.items.iter().map(|k| -> &dyn muda::IsMenuItem {
            match k {
                muda::MenuItemKind::MenuItem(i) => i,
                muda::MenuItemKind::Submenu(s) => s,
                muda::MenuItemKind::Predefined(p) => p,
                muda::MenuItemKind::Check(c) => c,
                muda::MenuItemKind::Icon(i) => i,
            }
        }).collect();
        let submenu = Submenu::with_items(&self.label, true, &refs).expect("submenu");
        SubMenuBuilt { submenu, handlers: self.handlers }
    }
}


fn parse_shortcut(s: &str) -> Option<Accelerator> {
    if s.is_empty() { return None; }
    let mut mods = Modifiers::empty();
    let mut key_part = "";
    for part in s.split('+') {
        match part.trim() {
            "CmdOrCtrl" | "Cmd" => {
                #[cfg(target_os = "macos")]
                { mods |= Modifiers::SUPER; }
                #[cfg(not(target_os = "macos"))]
                { mods |= Modifiers::CONTROL; }
            }
            "Ctrl"             => { mods |= Modifiers::CONTROL; }
            "Alt" | "Option"   => { mods |= Modifiers::ALT; }
            "Shift"            => { mods |= Modifiers::SHIFT; }
            "Meta" | "Super"   => { mods |= Modifiers::SUPER; }
            other              => { key_part = other; }
        }
    }
    let code = parse_code(key_part)?;
    Some(Accelerator::new(Some(mods), code))
}

fn parse_code(s: &str) -> Option<Code> {
    if s.len() == 1 {
        return match s.chars().next().unwrap().to_ascii_uppercase() {
            'A' => Some(Code::KeyA), 'B' => Some(Code::KeyB), 'C' => Some(Code::KeyC),
            'D' => Some(Code::KeyD), 'E' => Some(Code::KeyE), 'F' => Some(Code::KeyF),
            'G' => Some(Code::KeyG), 'H' => Some(Code::KeyH), 'I' => Some(Code::KeyI),
            'J' => Some(Code::KeyJ), 'K' => Some(Code::KeyK), 'L' => Some(Code::KeyL),
            'M' => Some(Code::KeyM), 'N' => Some(Code::KeyN), 'O' => Some(Code::KeyO),
            'P' => Some(Code::KeyP), 'Q' => Some(Code::KeyQ), 'R' => Some(Code::KeyR),
            'S' => Some(Code::KeyS), 'T' => Some(Code::KeyT), 'U' => Some(Code::KeyU),
            'V' => Some(Code::KeyV), 'W' => Some(Code::KeyW), 'X' => Some(Code::KeyX),
            'Y' => Some(Code::KeyY), 'Z' => Some(Code::KeyZ),
            '0' => Some(Code::Digit0), '1' => Some(Code::Digit1), '2' => Some(Code::Digit2),
            '3' => Some(Code::Digit3), '4' => Some(Code::Digit4), '5' => Some(Code::Digit5),
            '6' => Some(Code::Digit6), '7' => Some(Code::Digit7), '8' => Some(Code::Digit8),
            '9' => Some(Code::Digit9),
            _ => None,
        };
    }
    match s {
        "F1"  => Some(Code::F1),  "F2"  => Some(Code::F2),  "F3"  => Some(Code::F3),
        "F4"  => Some(Code::F4),  "F5"  => Some(Code::F5),  "F6"  => Some(Code::F6),
        "F7"  => Some(Code::F7),  "F8"  => Some(Code::F8),  "F9"  => Some(Code::F9),
        "F10" => Some(Code::F10), "F11" => Some(Code::F11), "F12" => Some(Code::F12),
        "Enter"|"Return"  => Some(Code::Enter),
        "Escape"|"Esc"    => Some(Code::Escape),
        "Backspace"       => Some(Code::Backspace),
        "Delete"          => Some(Code::Delete),
        "Tab"             => Some(Code::Tab),
        "Space"           => Some(Code::Space),
        "ArrowUp"|"Up"    => Some(Code::ArrowUp),
        "ArrowDown"|"Down"=> Some(Code::ArrowDown),
        "ArrowLeft"|"Left"=> Some(Code::ArrowLeft),
        "ArrowRight"|"Right"=>Some(Code::ArrowRight),
        "Home"      => Some(Code::Home),
        "End"       => Some(Code::End),
        "PageUp"    => Some(Code::PageUp),
        "PageDown"  => Some(Code::PageDown),
        _ => None,
    }
}


#[cfg(target_os = "macos")]
pub fn install_menu_macos(menu: &MenuBar) {
    menu.inner.init_for_nsapp();
}
#[cfg(not(target_os = "macos"))]
pub fn install_menu_macos(_menu: &MenuBar) {}

#[cfg(target_os = "windows")]
pub fn install_menu_windows(menu: &MenuBar, hwnd: isize) {
    unsafe { let _ = menu.inner.init_for_hwnd(hwnd); }
}
#[cfg(not(target_os = "windows"))]
pub fn install_menu_windows(_menu: &MenuBar, _hwnd: isize) {}

/// Drain pending menu events and fire registered callbacks.
pub fn poll_menu_events(handlers: &Arc<Mutex<HashMap<String, Box<dyn Fn() + Send + Sync>>>>) {
    while let Ok(event) = muda::MenuEvent::receiver().try_recv() {
        let id = event.id().0.to_string();
        if let Some(f) = handlers.lock().unwrap().get(&id) {
            f();
        }
    }
}
