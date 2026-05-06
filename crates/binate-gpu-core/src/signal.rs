use std::cell::{Cell, RefCell};
use std::rc::Rc;

// Thread-local dirty flag; set by any Signal::set on this thread.
thread_local! {
    static NEEDS_REDRAW: Cell<bool> = Cell::new(false);
}

/// Returns true if any `Signal` was written since the last `clear_redraw`.
pub fn needs_redraw() -> bool {
    NEEDS_REDRAW.with(|f| f.get())
}

/// Clears the per-thread redraw flag. Call this after requesting a redraw.
pub fn clear_redraw() {
    NEEDS_REDRAW.with(|f| f.set(false));
}

/// Shared, cloneable reactive value. Clones share the same underlying cell,
/// so writes from any clone are visible to all others and trigger a redraw.
pub struct Signal<T: Copy> {
    value: Rc<RefCell<T>>,
    dirty: Rc<Cell<bool>>,
}

impl<T: Copy> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            dirty: Rc::new(Cell::new(false)),
        }
    }

    pub fn get(&self) -> T {
        *self.value.borrow()
    }

    /// Write a new value and mark both the per-signal and per-thread dirty flags.
    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        self.dirty.set(true);
        NEEDS_REDRAW.with(|f| f.set(true));
    }

    /// True if this signal was written since the last `clear_dirty`.
    pub fn is_dirty(&self) -> bool {
        self.dirty.get()
    }

    pub fn clear_dirty(&self) {
        self.dirty.set(false);
    }
}

impl<T: Copy> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            dirty: Rc::clone(&self.dirty),
        }
    }
}
