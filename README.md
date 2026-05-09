<p align="center">
  <img src="Glyph.png" alt="Glyph" width="320" />
</p>

<h1 align="center">Glyph</h1>

<p align="center">
  A GPU-accelerated reactive UI framework for Rust.
</p>

---

Glyph renders UI via custom wgpu pipelines with signal-based reactivity and flexbox layout. Describe your interface as a `View` tree, bind state with `Signal<T>`, and the platform loop redraws automatically on every write. A macOS AppKit bridge is also available for native rendering from the same tree.

## Quick Start

```sh
cargo run -p glyph-demo
```

The demo opens an 800x600 window with a reactive counter backed by `Signal<i32>`.

## Example

```rust
use glyph_core::{Color, FontWeight, Signal, button, column, text};
use glyph_platform::App;

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
| `glyph-core` | `View` tree, `Signal<T>`, Taffy layout, flat quad output |
| `glyph-text` | cosmic-text shaping, glyph atlas, text measurement |
| `glyph-render` | wgpu pipelines, rect/text/image renderer |
| `glyph-platform` | winit event loop, hit-test, click/hover/scroll dispatch |
| `glyph-native` | macOS AppKit bridge (objc2) |
| `glyph-widgets` | pre-built widgets: Checkbox, Toggle, Slider, RadioGroup, Select |
| `glyph-demo` | interactive demo app |
| `glyph-github` | GitHub dashboard example |

## Status

| Feature | Status |
|---|---|
| GPU rect + text + image rendering | Working |
| Signal-driven redraws | Working |
| Flexbox layout (Taffy) | Working |
| Text wrapping | Working |
| Mouse hit-test, click, hover, scroll | Working |
| Text input with on_submit | Working |
| Container backgrounds, borders, shadows | Working |
| Clip regions | Working |
| ZStack / flex grow / spacer | Working |
| Widget system (Component + Widget traits) | Working |
| macOS AppKit native bridge | Working |
| Linux / Windows | wgpu handles backends; platform layer untested |
| Custom fonts | Not implemented |
| Multi-window | Not implemented |
| Hot-reload | Not implemented |

## License

MIT. See [LICENSE](LICENSE).
