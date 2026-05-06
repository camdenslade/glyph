use binate_gpu_core::{Color, FontWeight, Signal, button, column, text};
use binate_gpu_platform::App;

fn main() {
    let count = Signal::new(0i32);
    App::run(move || {
        column(vec![
            text(format!("Count: {}", count.get()), 48.0)
                .weight(FontWeight::Bold)
                .into(),
            text("Click the button to increment.", 16.0)
                .color(Color::rgb(0.5, 0.5, 0.5))
                .into(),
            button("Click me", {
                let count = count.clone();
                move || count.set(count.get() + 1)
            })
            .bg(Color::rgb(0.1, 0.1, 0.9))
            .text_color(Color::WHITE)
            .radius(10.0)
            .font_size(16.0)
            .into(),
        ])
    });
}
