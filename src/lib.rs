//! The Kaki programming language.

extern crate num_bigint;
extern crate unicode_segmentation;

/// The crate version.
pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");

// The util module must come first so that its macros are available to everything else.
#[macro_use]
pub mod util;
pub mod compiler;
pub mod edition;
