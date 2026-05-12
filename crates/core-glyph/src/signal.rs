use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub(crate) static NEEDS_REDRAW: AtomicBool = AtomicBool::new(false);

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

impl<T: Clone> Signal<T> {
    /// Return a raw pointer to the underlying `Arc<Mutex<T>>` with the
    /// reference count incremented. The caller is responsible for eventually
    /// reconstructing the Arc (via `Arc::from_raw`) to avoid a leak.
    pub fn as_raw_arc(&self) -> *const std::sync::Mutex<T> {
        Arc::into_raw(Arc::clone(&self.value))
    }
}
