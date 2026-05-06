<p align="center">
  <img src="Binate-GPU.png" alt="Binate GPU" width="320" />
</p>

<h1 align="center">Binate GPU</h1>

<p align="center">
  A GPU-accelerated reactive UI framework for Rust.
</p>

---

Binate GPU renders UI via custom wgpu pipelines with signal-based reactivity and flexbox layout. Describe your interface as a `View` tree, bind state with `Signal<T>`, and the platform loop redraws automatically on every write. A macOS AppKit bridge is also available for native rendering from the same tree.

## Quick Start

```sh
cargo run -p binate-gpu-demo
```

The demo opens an 800x600 window with a reactive counter backed by `Signal<i32>`.

## Example

```rust
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
```

## How it works

Each frame runs two GPU passes:

1. **Rect pass**: filled rectangles and button backgrounds drawn with a WGSL SDF shader, anti-aliased rounded corners at any radius
2. **Text pass**: glyphs shaped by cosmic-text, packed into a 1024x1024 R8 atlas, rendered by sampling atlas alpha and applying vertex color

Rects are submitted before text so text always composites on top. Layout is computed by [Taffy](https://github.com/DioxusLabs/taffy) (flexbox) each frame using real shaped text metrics for intrinsic sizing.

## Signals

`Signal<T>` is a cloneable reactive cell. Any write sets a thread-local dirty flag; the platform loop checks it after every event and calls `request_redraw` when set.

```rust
let value = Signal::new(0i32);
value.set(value.get() + 1); // triggers a redraw on the next event
```

## Crates

| Crate | Role |
|---|---|
| `binate-gpu-core` | `View` tree, `Signal<T>`, Taffy layout, flat quad output |
| `binate-gpu-text` | cosmic-text shaping, glyph atlas, text measurement |
| `binate-gpu-render` | wgpu pipelines, two-pass renderer |
| `binate-gpu-platform` | winit event loop, hit-test, click dispatch |
| `binate-gpu-native` | macOS AppKit bridge (objc2) |
| `binate-gpu-demo` | Counter app example |

## Status

| Feature | Status |
|---|---|
| GPU rect + text rendering | Working |
| Signal-driven redraws | Working |
| Flexbox layout (Taffy) | Working |
| Mouse hit-test + click | Working |
| macOS native bridge | Compiles, no demo yet |
| Scrolling | Not implemented |
| Text input | Not implemented |
| Linux / Windows | wgpu handles backends; platform layer untested |
| Component model | Not implemented |
| Hot-reload | Not implemented |

## License

MIT. See [LICENSE](LICENSE).
