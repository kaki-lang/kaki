//! Inners working of the compiler. This includes lexing, parsing, and code generation.
//!
//! A special feature of the compiler is that all parsing is grapheme based, which means that
//! unless otherwise noted, all indexing is done at the grapheme level.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod token;
