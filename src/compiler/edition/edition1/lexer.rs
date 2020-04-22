//! The lexer for edition 1.

use crate::compiler::lexer::{Lexer, LexerBase, LexerResult};
use crate::compiler::token::Token;
use crate::edition::Edition;

/// The lexer for the edition 1 of the specification.
#[derive(Clone)]
pub struct LexerEdition1<'a> {
    /// The base lexer.
    base: LexerBase<'a>,
}

impl<'a> LexerEdition1<'a> {
    /// Create a new lexer for some source.
    ///
    /// # Arguments
    ///
    /// * `source` - The source to lex.
    ///
    /// # Returns
    ///
    /// A new [`LexerEdition1`].
    pub fn new(source: &'a str) -> LexerEdition1<'a> {
        LexerEdition1 {
            base: LexerBase::new(source),
        }
    }
}

impl<'a> Lexer<'a> for LexerEdition1<'a> {
    fn source(&self) -> &'a str {
        self.base.source()
    }

    fn edition(&self) -> Edition {
        Edition::Edition1
    }

    fn next_token(&mut self) -> Option<LexerResult<'a>> {
        unimplemented!()
    }
}

// TODO Make the lexer
