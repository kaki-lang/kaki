//! A lexer which produces tokens for source code. The lexer only produces the minimal information
//! about the token and its location, and does not try to make any decisions on context.

use crate::edition::Edition;
use super::span::Span;
use super::token::{Token, TokenKind};
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

/// The types of errors that can occur during lexing.
#[derive(Clone, Debug, PartialEq)]
pub enum LexerErrorKind {
    /// The token was incomplete. For example, this could be missing closing delimiters on a
    /// string.
    Incomplete,

    /// A token was able to be lexed, but its contents are invalid.
    Invalid,

    /// The sequence of graphemes do not match the definitions of any tokens. This is the
    /// fallthrough case where no tokens are able to matched.
    NoMatch,
}

/// A token of source code.
#[derive(Clone, Debug, PartialEq)]
pub struct LexerError<'a> {
    /// The text of the token.
    pub text: &'a str,

    /// The kind of the error.
    pub kind: LexerErrorKind,

    /// The location of the error.
    pub span: Span,

    /// The kind that the token would be (if able to be determined).
    pub token_kind: Option<TokenKind>,
}

/// The result from a partial lexer.
#[derive(Clone, Debug, PartialEq)]
pub enum LexerResult<'a> {
    /// A token was successfully lexed.
    Token(Token<'a>),

    /// A token was matched, but it contained an error.
    Error(LexerError<'a>),
}

/// This base lexer implements many common operations that derived lexers will share.
#[derive(Clone)]
pub struct LexerBase<'a> {
    /// The previous grapheme consumed by the lexer.
    prev: Option<&'a str>,

    /// The index in the source, measured in graphemes.
    index: usize,

    /// The index in the source, measured in bytes.
    index_bytes: usize,

    /// The marked index in the source, measured in graphemes.
    mark: usize,

    /// The marked index in the source, measured in bytes.
    mark_bytes: usize,

    /// The source code.
    source: &'a str,

    /// An iterator over the graphemes in the source.
    graphemes: Graphemes<'a>,
}

impl<'a> LexerBase<'a> {
    /// Create a new lexer for some source.
    ///
    /// # Arguments
    ///
    /// * `source` - The source to lex.
    ///
    /// # Returns
    ///
    /// A new [`Lexer`].
    pub fn new(source: &'a str) -> LexerBase<'a> {
        LexerBase {
            prev: None,
            index: 0,
            index_bytes: 0,
            mark: 0,
            mark_bytes: 0,
            source: source,
            graphemes: source.graphemes(true),
        }
    }

    /// Get the source string that the lexer is working on.
    pub fn source(&self) -> &'a str {
        self.source
    }

    /// Mark the lexer with the current position.
    pub fn mark(&mut self) {
        self.mark = self.index;
        self.mark_bytes = self.index_bytes;
    }

    /// Get the span of the marked to current position.
    pub fn span(&self) -> Span {
        Span::new(self.mark, self.index)
    }

    /// Get the next grapheme and advance the lexer.
    ///
    /// # Returns
    ///
    /// The next grapheme if not at the end of the lexer.
    pub fn next(&mut self) -> Option<&'a str> {
        if let Some(g) = self.graphemes.next() {
            self.index += 1;
            self.index_bytes += g.len();
            Some(g)
        } else {
            None
        }
    }

    /// Get the previous grapheme.
    ///
    /// # Returns
    ///
    /// The next grapheme if not at the start of the lexer.
    pub fn prev(&self) -> Option<&'a str> {
        self.prev
    }

    /// Peek the next grapheme without advancing the lexer.
    ///
    /// # Returns
    ///
    /// The next grapheme if not at the end of the lexer.
    pub fn peek(&self) -> Option<&'a str> {
        self.clone().graphemes.next()
    }

    /// Extract the text between the marked and current position.
    ///
    /// # Return
    ///
    /// The text between the marked boundaries.
    pub fn text(&self) -> &'a str {
        &self.source[self.mark_bytes..self.index_bytes]
    }

    /// Extract a token between the marked and current position.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of the token to extract.
    ///
    /// # Returns
    ///
    /// This always return `Some(Ok(...))` with the [`Token`].
    pub fn extract(&self, kind: TokenKind) -> Option<Result<Token<'a>, LexerError<'a>>> {
        Some(Ok(Token {
            text: self.text(),
            kind: kind,
            span: self.span(),
        }))
    }

    /// Extract an error between the marked and current position.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of the error.
    /// * `token_kind` - The kind of the token that the error occurred in, if known.
    ///
    /// # Returns
    ///
    /// This always returns `Some(Err(...))` with a [`LexerError`].
    pub fn error(
        &self,
        kind: LexerErrorKind,
        token_kind: Option<TokenKind>,
    ) -> Option<Result<Token<'a>, LexerError<'a>>> {
        Some(Err(LexerError {
            text: self.text(),
            kind: kind,
            span: self.span(),
            token_kind: token_kind,
        }))
    }

    /// Consume graphemes while a predicate is satisfied.
    ///
    /// # Arguments
    ///
    /// * `pred` - The predicate used to test.
    ///
    /// # Returns
    ///
    /// The number of graphemes consumed.
    fn take_while(&mut self, pred: &dyn Fn(&str) -> bool) -> usize {
        let mut count = 0;
        while let Some(g) = self.peek() {
            if !pred(g) {
                break;
            }
            count += 1;
        }
        count
    }

    /// Advance the lexer to consume a pattern if it exists at the current lexer position.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match.
    ///
    /// # Returns
    ///
    /// `true` when the pattern is matched and the lexer is advanced, `false` otherwise.
    fn expect(&mut self, pattern: &str) -> bool {
        let mut lexer = self.clone();
        let mut count: usize = 0;
        // Test that the pattern matches
        for g1 in pattern.graphemes(true) {
            count += 1;
            if let Some(g2) = lexer.next() {
                if g1 != g2 {
                    return false;
                }
            } else {
                return false;
            }
        }
        // Advance the lexer
        for _ in 0..count {
            self.next();
        }
        true
    }

    /// The pattern is treated as a set of graphemes and attempts to match any one of them at the
    /// current position. If a match is made the lexer is advanced.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match, which is treated as a set of graphemes.
    ///
    /// # Returns
    ///
    /// `true` when the pattern is matched and the lexer is advanced, `false` otherwise.
    fn expect_one(&mut self, pattern: &str) -> bool {
        for g in pattern.graphemes(true) {
            if self.expect(g) {
                return true;
            }
        }
        false
    }

    /// Lex an exact pattern if it matches.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match.
    /// * `kind` - The kind iof the resulting token.
    ///
    /// # Returns
    ///
    /// `Some(Ok(...))` containing a [`Token`] if the pattern was matched, otherwise `None`.
    fn lex_exact(
        &mut self,
        pattern: &str,
        kind: TokenKind,
    ) -> Option<Result<Token<'a>, LexerError<'a>>> {
        if self.expect(pattern) {
            self.extract(kind)
        } else {
            None
        }
    }
}

/// The operations that any lexer should be able to perform.
pub trait Lexer<'a> {
    /// Get the source code of the lexer.
    fn source(&self) -> &'a str;

    /// Get the edition of the lexer.
    fn edition(&self) -> Edition;

    /// If there are more tokens avaiable, consume the next one and return it. If the token cannot
    /// be be determined, and error is returned. When there are no more tokens left this should
    /// return `None`.
    fn next_token(&mut self) -> Option<LexerResult<'a>>;
}

impl<'a> Iterator for dyn Lexer<'a> {
    type Item = LexerResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
