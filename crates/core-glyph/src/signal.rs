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

impl Signal<f32> {
    /// Clamp `value` to `[0, max]` then set. No-op if `max < 0` (not yet measured).
    pub fn set_clamped(&self, value: f32, max: f32) {
        self.set(value.clamp(0.0, max.max(0.0)));
    }
}

/// Scroll a container to an absolute Y position, clamped to the measured content height.
/// `offset_y` and `max_scroll` are the signals passed to `scroll()`.
pub fn scroll_to_y(offset_y: &Signal<f32>, max_scroll: &Signal<(f32, f32)>, y: f32) {
    let max = max_scroll.get().1;
    offset_y.set(y.clamp(0.0, max.max(0.0)));
}

/// Scroll to the top of a scroll container.
pub fn scroll_to_top(offset_y: &Signal<f32>) {
    offset_y.set(0.0);
}

/// Scroll to the bottom of a scroll container.
pub fn scroll_to_bottom(offset_y: &Signal<f32>, max_scroll: &Signal<(f32, f32)>) {
    let max = max_scroll.get().1;
    offset_y.set(max.max(0.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_initial_value() {
        let s = Signal::new(42i32);
        assert_eq!(s.get(), 42);
    }

    #[test]
    fn set_updates_value() {
        let s = Signal::new(0i32);
        s.set(99);
        assert_eq!(s.get(), 99);
    }

    #[test]
    fn set_marks_redraw() {
        clear_redraw();
        let s = Signal::new(false);
        assert!(!needs_redraw());
        s.set(true);
        assert!(needs_redraw());
        clear_redraw();
    }

    #[test]
    fn clone_shares_value() {
        let a = Signal::new(1i32);
        let b = a.clone();
        a.set(7);
        assert_eq!(b.get(), 7);
    }

    #[test]
    fn independent_signals_do_not_share_value() {
        let a = Signal::new(1i32);
        let b = Signal::new(1i32);
        a.set(99);
        assert_eq!(b.get(), 1);
    }
}
