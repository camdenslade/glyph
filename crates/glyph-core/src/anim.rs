/// Time-based value interpolation driven by the platform event loop.
///
/// # Usage
///
/// ```rust
/// use glyph_core::{Signal, Tween, Easing};
///
/// let opacity = Signal::new(0.0f32);
/// let tween   = Tween::new(opacity.clone(), Easing::EaseOut, 0.3);
/// tween.start(1.0); // animate opacity → 1.0 over 300ms
///
/// // The platform's event loop advances all live tweens each frame.
/// // When the tween completes the signal holds exactly the target value.
/// ```
use std::sync::{Arc, Mutex, Weak};

use crate::signal::{Signal, NEEDS_REDRAW};
use crate::view::Lerp;

// ---------------------------------------------------------------------------
// Easing functions
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    /// Cubic bezier with two control points P1 and P2 (P0=(0,0), P3=(1,1)).
    CubicBezier(f32, f32, f32, f32),
}

impl Easing {
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t * t,
            Easing::EaseOut => {
                let u = 1.0 - t;
                1.0 - u * u * u
            }
            Easing::EaseInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let u = -2.0 * t + 2.0;
                    1.0 - u * u * u / 2.0
                }
            }
            Easing::CubicBezier(x1, y1, x2, y2) => {
                // Newton's method to solve for t given x, then evaluate y.
                cubic_bezier(t, x1, y1, x2, y2)
            }
        }
    }
}

fn cubic_bezier(x: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let mut t = x;
    for _ in 0..8 {
        let cx = 3.0 * x1;
        let bx = 3.0 * (x2 - x1) - cx;
        let ax = 1.0 - cx - bx;
        let sample = ax * t * t * t + bx * t * t + cx * t;
        let dx = 3.0 * ax * t * t + 2.0 * bx * t + cx;
        if dx.abs() < 1e-6 { break; }
        t -= (sample - x) / dx;
    }
    let cy = 3.0 * y1;
    let by_ = 3.0 * (y2 - y1) - cy;
    let ay = 1.0 - cy - by_;
    ay * t * t * t + by_ * t * t + cy * t
}

// ---------------------------------------------------------------------------
// Global tween registry — the platform polls this each frame
// ---------------------------------------------------------------------------

static REGISTRY: std::sync::OnceLock<Arc<Mutex<Vec<Weak<dyn AnyTween>>>>> =
    std::sync::OnceLock::new();

fn registry() -> &'static Arc<Mutex<Vec<Weak<dyn AnyTween>>>> {
    REGISTRY.get_or_init(|| Arc::new(Mutex::new(Vec::new())))
}

/// Advance all live tweens by `dt` seconds. Returns true if any tween is
/// still running (the caller should request a redraw).
/// Dead weak refs are pruned on each call.
pub fn tick_tweens(dt: f32) -> bool {
    let mut reg = registry().lock().unwrap();
    let mut still_running = false;
    reg.retain(|weak| {
        if let Some(t) = weak.upgrade() {
            if t.tick(dt) { still_running = true; }
            true
        } else {
            false
        }
    });
    still_running
}

trait AnyTween: Send + Sync {
    /// Advance the tween by `dt` seconds. Returns true if still animating.
    fn tick(&self, dt: f32) -> bool;
}

// ---------------------------------------------------------------------------
// Tween<T>
// ---------------------------------------------------------------------------

struct TweenInner<T: Lerp> {
    signal:   Signal<T>,
    easing:   Easing,
    duration: f32,
    from:     Option<T>,
    to:       Option<T>,
    elapsed:  f32,
    running:  bool,
}

impl<T: Lerp> TweenInner<T> {
    fn tick(&mut self, dt: f32) -> bool {
        if !self.running { return false; }
        let (from, to) = match (&self.from, &self.to) {
            (Some(f), Some(t)) => (f.clone(), t.clone()),
            _ => { self.running = false; return false; }
        };
        self.elapsed = (self.elapsed + dt).min(self.duration);
        let raw_t = if self.duration > 0.0 { self.elapsed / self.duration } else { 1.0 };
        let t = self.easing.apply(raw_t);
        self.signal.set(T::lerp(&from, &to, t));
        if raw_t >= 1.0 {
            self.signal.set(to);
            self.running = false;
            return false;
        }
        true
    }
}

/// A reusable, cloneable animation driver. Clones share the same underlying
/// state — calling `start` on any clone starts/restarts the animation.
pub struct Tween<T: Lerp>(Arc<Mutex<TweenInner<T>>>);

impl<T: Lerp + 'static> Tween<T> {
    /// Create a tween that drives `signal` over `duration` seconds using `easing`.
    pub fn new(signal: Signal<T>, easing: Easing, duration: f32) -> Self {
        let inner = Arc::new(Mutex::new(TweenInner {
            signal,
            easing,
            duration,
            from: None,
            to: None,
            elapsed: 0.0,
            running: false,
        }));
        let weak: Weak<dyn AnyTween> = Arc::downgrade(&(Arc::clone(&inner) as Arc<dyn AnyTween>));
        registry().lock().unwrap().push(weak);
        Self(inner)
    }

    /// Start animating from `from` to `to`, restarting if already running.
    pub fn animate(&self, from: T, to: T) {
        let mut g = self.0.lock().unwrap();
        g.from    = Some(from);
        g.to      = Some(to);
        g.elapsed = 0.0;
        g.running = true;
        std::sync::atomic::AtomicBool::store(
            &NEEDS_REDRAW,
            true,
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    /// Animate from the signal's current value to `to`.
    pub fn start(&self, to: T) {
        let current = self.0.lock().unwrap().signal.get();
        self.animate(current, to);
    }

    /// Stop the animation at the current value.
    pub fn stop(&self) {
        self.0.lock().unwrap().running = false;
    }

    pub fn is_running(&self) -> bool {
        self.0.lock().unwrap().running
    }
}

impl<T: Lerp> Clone for Tween<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: Lerp + 'static> AnyTween for Mutex<TweenInner<T>> {
    fn tick(&self, dt: f32) -> bool {
        self.lock().unwrap().tick(dt)
    }
}
