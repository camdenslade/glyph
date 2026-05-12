use core_glyph::View;
use objc2::{
    declare_class, msg_send_id,
    mutability::MainThreadOnly,
    rc::Retained,
    runtime::AnyObject,
    ClassType, DeclaredClass,
};
use objc2_app_kit::{
    NSApplication, NSBackingStoreType, NSButton, NSLayoutAttribute,
    NSStackView, NSTextField, NSUserInterfaceLayoutOrientation,
    NSView, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{MainThreadMarker, NSArray, NSPoint, NSRect, NSSize, NSString};
use std::cell::Cell;

// A Rust fat pointer (`*mut dyn Fn()`) is two words: data pointer + vtable pointer.
// We split it into two `*mut ()` ivars so it can be stored inside an Objective-C
// object without a separate heap allocation. `Drop` reassembles and frees it.
struct ActionTargetIvars {
    callback_data:   Cell<*mut ()>,
    callback_vtable: Cell<*mut ()>,
}

declare_class!(
    struct ActionTarget;

    unsafe impl ClassType for ActionTarget {
        type Super = objc2::runtime::NSObject;
        type Mutability = MainThreadOnly;
        const NAME: &'static str = "GlyphActionTarget";
    }

    impl DeclaredClass for ActionTarget {
        type Ivars = ActionTargetIvars;
    }

    unsafe impl ActionTarget {
        #[method(performAction:)]
        fn perform_action(&self, _sender: *mut AnyObject) {
            let data    = self.ivars().callback_data.get();
            let vtable  = self.ivars().callback_vtable.get();
            if !data.is_null() {
                let fat: *mut dyn Fn() = unsafe { std::mem::transmute((data, vtable)) };
                unsafe { (*fat)() };
            }
        }
    }
);

impl ActionTarget {
    fn new(mtm: MainThreadMarker, callback: Box<dyn Fn()>) -> Retained<Self> {
        let raw: *mut dyn Fn() = Box::into_raw(callback);
        let (data, vtable): (*mut (), *mut ()) = unsafe { std::mem::transmute(raw) };
        let this = mtm.alloc::<Self>();
        let this = this.set_ivars(ActionTargetIvars {
            callback_data:   Cell::new(data),
            callback_vtable: Cell::new(vtable),
        });
        unsafe { msg_send_id![super(this), init] }
    }
}

impl Drop for ActionTarget {
    fn drop(&mut self) {
        let data   = self.ivars().callback_data.get();
        let vtable = self.ivars().callback_vtable.get();
        if !data.is_null() {
            let fat: *mut dyn Fn() = unsafe { std::mem::transmute((data, vtable)) };
            unsafe { drop(Box::from_raw(fat)) };
            self.ivars().callback_data.set(std::ptr::null_mut());
        }
    }
}

/// Entry point for the macOS native render path.
pub struct NativeApp;

impl NativeApp {
    /// Build a `View` tree once and render it as a native AppKit window.
    /// `targets` keeps `ActionTarget` objects alive for the duration of the run
    /// loop; dropping them before `app.run` would leave dangling ObjC selectors.
    pub fn run(build_view: impl Fn() -> View + 'static) {
        let mtm = MainThreadMarker::new().expect("must run on main thread");

        let app = NSApplication::sharedApplication(mtm);
        app.setActivationPolicy(objc2_app_kit::NSApplicationActivationPolicy::Regular);

        let window = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                mtm.alloc::<NSWindow>(),
                NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(800.0, 600.0)),
                NSWindowStyleMask::Titled
                    | NSWindowStyleMask::Closable
                    | NSWindowStyleMask::Resizable
                    | NSWindowStyleMask::Miniaturizable,
                NSBackingStoreType::NSBackingStoreBuffered,
                false,
            )
        };

        window.setTitle(&NSString::from_str("glyph"));

        let view = build_view();
        let mut targets: Vec<Retained<ActionTarget>> = Vec::new();
        let root_nsview = build_nsview(mtm, &view, &mut targets);

        window.setContentView(Some(&root_nsview));
        window.center();
        window.makeKeyAndOrderFront(None);
        unsafe { app.run() };

        drop(targets);
    }
}

fn build_nsview(
    mtm: MainThreadMarker,
    view: &View,
    targets: &mut Vec<Retained<ActionTarget>>,
) -> Retained<NSView> {
    match view {
        View::Column { children, .. } => {
            stack_view(mtm, children, targets, NSUserInterfaceLayoutOrientation::Vertical)
        }
        View::Row { children, .. } => {
            stack_view(mtm, children, targets, NSUserInterfaceLayoutOrientation::Horizontal)
        }
        View::Text { content, font_size, color, .. } => {
            label(mtm, content, *font_size, *color)
        }
        View::Button { label: text, on_click, font_size, .. } => {
            native_button(mtm, text, on_click.as_ref(), *font_size, targets)
        }
        View::Rect { .. } | View::Image { .. } | View::TextInput { .. }
        | View::TextArea { .. } | View::VirtualList { .. } | View::Spacer => {
            let view = unsafe { NSView::initWithFrame(mtm.alloc::<NSView>(), NSRect::ZERO) };
            unsafe { Retained::cast(view) }
        }
        View::Scroll { child, .. } | View::Flexible { child, .. } | View::Opacity { child, .. } => build_nsview(mtm, child, targets),
        View::ZStack { children, .. } => {
            stack_view(mtm, children, targets, NSUserInterfaceLayoutOrientation::Vertical)
        }
        View::Component(c) => {
            let rendered = c.render(&core_glyph::Theme::light());
            build_nsview(mtm, &rendered, targets)
        }
    }
}

fn stack_view(
    mtm: MainThreadMarker,
    children: &[View],
    targets: &mut Vec<Retained<ActionTarget>>,
    orientation: NSUserInterfaceLayoutOrientation,
) -> Retained<NSView> {
    let child_views: Vec<Retained<NSView>> = children
        .iter()
        .map(|c| build_nsview(mtm, c, targets))
        .collect();

    let refs: Vec<&NSView> = child_views
        .iter()
        .map(|v: &Retained<NSView>| v.as_ref())
        .collect();
    let arr = NSArray::from_slice(&refs);
    let stack = unsafe { NSStackView::stackViewWithViews(&arr, mtm) };
    unsafe { stack.setOrientation(orientation) };
    unsafe { stack.setAlignment(NSLayoutAttribute::CenterX) };
    unsafe { stack.setSpacing(16.0) };
    unsafe { Retained::cast(stack) }
}

fn label(
    mtm: MainThreadMarker,
    content: &str,
    font_size: f32,
    color: core_glyph::Color,
) -> Retained<NSView> {
    let tf = unsafe { NSTextField::labelWithString(&NSString::from_str(content), mtm) };
    let font = unsafe { objc2_app_kit::NSFont::systemFontOfSize(font_size as f64) };
    unsafe { tf.setFont(Some(&font)) };
    let ns_color = unsafe {
        objc2_app_kit::NSColor::colorWithRed_green_blue_alpha(
            color.r as f64, color.g as f64, color.b as f64, color.a as f64,
        )
    };
    unsafe { tf.setTextColor(Some(&ns_color)) };
    unsafe { Retained::cast(tf) }
}

fn native_button(
    mtm: MainThreadMarker,
    title: &str,
    on_click: &dyn Fn(),
    font_size: f32,
    targets: &mut Vec<Retained<ActionTarget>>,
) -> Retained<NSView> {
    let cb: Box<dyn Fn()> = unsafe {
        std::mem::transmute::<Box<dyn Fn() + '_>, Box<dyn Fn() + 'static>>(
            Box::new(on_click),
        )
    };
    let target = ActionTarget::new(mtm, cb);
    let sel = objc2::sel!(performAction:);

    let btn = unsafe {
        NSButton::buttonWithTitle_target_action(
            &NSString::from_str(title),
            Some(target.as_ref() as &AnyObject),
            Some(sel),
            mtm,
        )
    };

    let font = unsafe { objc2_app_kit::NSFont::systemFontOfSize(font_size as f64) };
    unsafe { btn.setFont(Some(&font)) };

    targets.push(target);
    unsafe { Retained::cast(btn) }
}
