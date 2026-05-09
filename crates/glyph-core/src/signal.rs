use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

static NEEDS_REDRAW: AtomicBool = AtomicBool::new(false);

pub fn needs_redraw() -> bool {
    NEEDS_REDRAW.load(Ordering::Relaxed)
}

pub fn clear_redraw() {
    NEEDS_REDRAW.store(false, Ordering::Relaxed);
}

/// Shared, cloneable reactive value. Clones share the same underlying cell,
/// so writes from any clone are visible to all others and trigger a redraw.
/// `Signal` is `Send + Sync` so it can be written from background threads.
pub struct Signal<T: Clone> {
    value: Arc<Mutex<T>>,
}

impl<T: Clone> Signal<T> {
    pub fn new(value: T) -> Self {
        Self { value: Arc::new(Mutex::new(value)) }
    }

    pub fn get(&self) -> T {
        self.value.lock().unwrap().clone()
    }

    pub fn set(&self, value: T) {
        *self.value.lock().unwrap() = value;
        NEEDS_REDRAW.store(true, Ordering::Relaxed);
    }
}

impl<T: Clone> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self { value: Arc::clone(&self.value) }
    }
}
