//! Facilities for dealing with tokens directly.

use super::span::Span;

/// The kind of a token.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    /// Only tabs and spaces.
    Space,

    /// One `\n` or `\r\n` sequence.
    NewLine,

    /// One line comment, not including the newline.
    CommentLine,

    /// One block comment, including all nested comments.
    CommentBlock,

    /// A name that starts with a lowercase letter.
    NameLower,

    /// A name that starts with a uppercase letter.
    NameUpper,

    /// A name that only consists of one underscore.
    NameUnderscore,

    /// A name that is `_<non-negative-integer>`.
    NameAnon,

    /// A binary integral value.
    IntBin,

    /// An octal integral value.
    IntOct,

    /// A decimal integral value.
    IntDec,

    /// A hexadecimal integral value.
    IntHex,

    /// A floating point value.
    Float,

    /// A single line string, including quotes.
    StringSingle,

    /// A multiline string, including quotes.
    StringMulti,

    /// A smart string, including quotes.
    StringSmart,

    /// One `&`.
    Amp,

    /// One `&&`.
    AmpAmp,

    /// One `@`.
    At,

    /// One `@@`.
    AtAt,

    /// One `@{`.
    AtBraceL,

    /// One `!`.
    Bang,

    /// One `!=`.
    BangEq,

    /// One `[`.
    BracketL,

    /// One `]`.
    BracketR,

    /// One `{`.
    BraceL,

    /// One `}`.
    BraceR,

    /// One `\`.
    BackSlash,

    /// One `^`.
    Caret,

    /// One `:`.
    Colon,

    /// One `::`.
    ColonColon,

    /// One `,`.
    Comma,

    /// One `.`.
    Dot,

    /// One `=`.
    Eq,

    /// One `==`.
    EqEq,

    /// One `>`.
    Gt,

    /// One `>=`.
    GtEq,

    /// One `>>`.
    GtGt,

    /// One `<`.
    Lt,

    /// One `<=`.
    LtEq,

    /// One `<=>`.
    LtEqGt,

    /// One `<<`.
    LtLt,

    /// One `-`.
    Minus,

    /// One `(`.
    ParenL,

    /// One `)`.
    ParenR,

    /// One `%`.
    Percent,

    /// One `|`.
    Pipe,

    /// One `||`.
    PipePipe,

    /// One `+`.
    Plus,

    /// One `?`.
    Question,

    /// One `?=`.
    QuestionEq,

    /// One `/`.
    Slash,

    /// One `//`.
    SlashSlash,

    /// One `*`.
    Star,

    /// One `**`.
    StarStar,

    /// One `~`.
    Tilde,
}

/// A token of source code.
#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    /// The text of the token.
    pub text: &'a str,

    /// The kind of the token.
    pub kind: TokenKind,

    /// The location of the token.
    pub span: Span,
}

impl<'a> Token<'a> {
    /// Create a new [`Token`].
    ///
    /// # Arguments
    ///
    /// * `text` - The text of the token.
    /// * `kind` - The kind of the token.
    /// * `span` - The location of the token in the source.
    ///
    /// # Returns
    ///
    /// A new [`Token`].
    pub fn new(text: &str, kind: TokenKind, span: Span) -> Token {
        Token {
            text: text,
            kind: kind,
            span: span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind::*;

    #[test]
    fn test_token_new() {
        assert_eq!(
            Token::new("hello", NameLower, Span::new(5, 10)),
            Token {
                text: "hello",
                kind: NameLower,
                span: Span::new(5, 10)
            }
        );
    }
}
