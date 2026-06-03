# Contributing to Glyph

## Setup

```sh
git clone https://github.com/camdenslade/glyph
cd glyph
cargo build --workspace --exclude github-glyph
```

Requires Rust 1.76+ and macOS or Windows.

## Running tests

```sh
cargo test --workspace --exclude github-glyph
```

## Lint

CI runs clippy with `-D warnings` — make sure this passes before opening a PR:

```sh
cargo clippy --workspace --exclude github-glyph -- -D warnings
```

## Running the demo

```sh
cargo run -p demo-glyph
```

## Crate structure

| Crate | Role |
|---|---|
| `core-glyph` | View tree, signals, layout, animations |
| `text-glyph` | Font loading, shaping, glyph atlas |
| `render-glyph` | wgpu GPU pipelines |
| `platform-glyph` | winit event loop, input, menus, OS APIs |
| `ui-glyph` | Design system: components, icons, themes |
| `glyph` | Public API — re-exports all of the above |
| `glyph-kit` | CLI: `glyph new` / `run` / `build` |
| `hot-glyph` | Hot-reload support |
| `native-glyph` | macOS AppKit bridge |

## Code style

- No decorated comment headers (`// ──────`) or banner blocks (`// ------`)
- No comments explaining *what* code does — only *why* when the reason is non-obvious
- `snake_case` for functions/variables, `PascalCase` for types, `SCREAMING_SNAKE` for constants

## Opening a PR

Run tests and clippy, then open a PR against `main`. The PR template will guide you through the checklist.
