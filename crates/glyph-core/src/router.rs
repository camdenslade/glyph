use crate::component::Component;
use crate::signal::Signal;
use crate::theme::Theme;
use crate::view::View;

pub struct Router {
    pub stack: Signal<Vec<usize>>,
    pages: Vec<Box<dyn Fn(&Theme) -> View + Send>>,
}

impl Router {
    pub fn new(pages: Vec<Box<dyn Fn(&Theme) -> View + Send>>) -> Self {
        Self {
            stack: Signal::new(vec![0]),
            pages,
        }
    }

    pub fn push(&self, idx: usize) {
        let mut s = self.stack.get();
        s.push(idx);
        self.stack.set(s);
    }

    pub fn pop(&self) {
        let mut s = self.stack.get();
        if s.len() > 1 {
            s.pop();
        }
        self.stack.set(s);
    }

    pub fn replace(&self, idx: usize) {
        let mut s = self.stack.get();
        if let Some(last) = s.last_mut() {
            *last = idx;
        }
        self.stack.set(s);
    }

    pub fn current(&self) -> usize {
        *self.stack.get().last().unwrap_or(&0)
    }
}

impl Component for Router {
    fn render(&self, theme: &Theme) -> View {
        let idx = self.current();
        (self.pages[idx])(theme)
    }
}
