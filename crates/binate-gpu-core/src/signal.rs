use std::cell::{Cell, RefCell};
use std::rc::Rc;

thread_local! {
    static NEEDS_REDRAW: Cell<bool> = Cell::new(false);
}

/// Returns true if any signal was set since the last call to `clear_redraw`.
pub fn needs_redraw() -> bool {
    NEEDS_REDRAW.with(|f| f.get())
}

pub fn clear_redraw() {
    NEEDS_REDRAW.with(|f| f.set(false));
}

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

    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        self.dirty.set(true);
        NEEDS_REDRAW.with(|f| f.set(true));
    }

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
