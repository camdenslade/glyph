//! Pre-built widgets for glyph.
//!
//! Each widget implements both `Component` (GPU rendering) and `Widget`
//! (platform hints for native fallback). Use them directly or as reference
//! implementations when building your own.
//!
//! # Available widgets
//!
//! - [`Checkbox`] — boolean toggle with a label
//! - [`Toggle`] — on/off switch
//! - [`Slider`] — numeric range input
//! - [`RadioGroup`] — mutually exclusive option selection
//! - [`Select`] — dropdown option picker

mod checkbox;
mod radio;
mod select;
mod slider;
mod toggle;

pub use checkbox::Checkbox;
pub use radio::RadioGroup;
pub use select::Select;
pub use slider::Slider;
pub use toggle::Toggle;
