//! The Kaki programming language.

extern crate num_bigint;

/// The crate version.
pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod compiler;
pub mod edition;
