//! Facilities for dealing with tokens directly.

use self::TokenKind::*;
use super::span::Span;

/// The kind of a token.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    /// Only tabs and spaces.
    Space,

    /// One `\n` or `\r\n` sequence.
    NewLine,

    /// A word that starts with a lowercase letter.
    Lower,

    /// A word that starts with a uppercase letter.
    Upper,

    /// A word that only consists of underscores.
    Under,

    /// A word that is `$` or `$<non-negative-integer>`.
    Anon,

    /// An integral value.
    Integer,

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

impl TokenKind {
    /// Test if a token is whitespace.
    pub fn is_whitespace(&self) -> bool {
        match self {
            Space | NewLine => true,
            _ => false,
        }
    }

    /// Test if a token is a word.
    pub fn is_word(&self) -> bool {
        match self {
            Lower | Upper | Under | Anon => true,
            _ => false,
        }
    }

    /// Test if a token is an operator or punctuation.
    pub fn is_op_punc(&self) -> bool {
        match self {
            Space | NewLine | Amp | AmpAmp | At | AtAt | AtBraceL | Bang | BangEq | BracketL
            | BracketR | BraceL | BraceR | BackSlash | Caret | Colon | ColonColon | Comma | Dot
            | Eq | EqEq | Gt | GtEq | GtGt | Lt | LtEq | LtLt | Minus | ParenL | ParenR
            | Percent | Pipe | PipePipe | Plus | Question | QuestionEq | Slash | SlashSlash
            | Star | StarStar | Tilde => true,
            _ => false,
        }
    }

    /// Test if a token can be used as a boundary.
    pub fn is_boundary(&self) -> bool {
        self.is_whitespace() || self.is_op_punc()
    }
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
