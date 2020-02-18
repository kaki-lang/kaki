//! The Kaki programming language.

extern crate num_bigint;
extern crate unicode_segmentation;

/// The crate version.
pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod compiler;
