#![allow(clippy::type_complexity)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{text, Theme};

    fn make_router() -> Router {
        Router::new(vec![
            Box::new(|_| text("page 0", 16.0).into()),
            Box::new(|_| text("page 1", 16.0).into()),
            Box::new(|_| text("page 2", 16.0).into()),
        ])
    }

    #[test]
    fn starts_at_page_zero() {
        assert_eq!(make_router().current(), 0);
    }

    #[test]
    fn push_navigates_forward() {
        let r = make_router();
        r.push(2);
        assert_eq!(r.current(), 2);
    }

    #[test]
    fn pop_goes_back() {
        let r = make_router();
        r.push(1);
        r.push(2);
        r.pop();
        assert_eq!(r.current(), 1);
    }

    #[test]
    fn pop_does_not_go_below_root() {
        let r = make_router();
        r.pop();
        r.pop();
        assert_eq!(r.current(), 0);
        assert_eq!(r.stack.get().len(), 1);
    }

    #[test]
    fn replace_swaps_current_page() {
        let r = make_router();
        r.push(1);
        r.replace(2);
        assert_eq!(r.current(), 2);
        assert_eq!(r.stack.get().len(), 2);
    }

    #[test]
    fn render_returns_correct_page() {
        let r = make_router();
        r.push(1);
        let theme = Theme::light();
        // Just ensure render doesn't panic and produces a view.
        let _ = r.render(&theme);
    }
}
