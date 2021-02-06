//! TextBlock is a tool for calculating positions for all elements
//! in text block (paragraph) with specific settings like font face,
//! tracking / leading etc.
//!
//! ## Example
//!
//! ```rust,no_run
//! use text_block::{CharacterStyle, ParagraphStyle, TextBlock};
//!
//! let character = CharacterStyle::build().finish();
//! let paragraph = ParagraphStyle::build().style(character).finish();
//! let text_block = TextBlock::build()
//!    .style(paragraph_style)
//!    .text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
//!    .finish();
//!
//! ```
mod block;
mod color;
mod face;
mod size;
mod styles;
mod transform;

pub use color::Color;
pub use face::Face;
pub use size::Size;
pub use styles::{CharacterStyle, ParagraphStyle};
pub use transform::TextTransformation;
