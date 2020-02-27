//! A lexer which produces tokens for source code. The lexer only produces the minimal information
//! about the token and its location, and does not try to make any decisions on context.

use super::span::Span;
use super::token::{Token, TokenKind};
use unicode_segmentation::{Graphemes, UnicodeSegmentation};
use LexerErrorKind::*;
use TokenKind::*;

/// Returns its argument if it is the `Some` case of an [`Option`].
macro_rules! return_some {
    ($e:expr) => {
        if $e.is_some() {
            return $e;
        }
    };
}

/// Test that a string contains an ASCII lowercase character.
fn is_lower(s: &str) -> bool {
    match s {
        "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o"
        | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII uppercase character.
fn is_upper(s: &str) -> bool {
    match s {
        "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O"
        | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII alphabetic character or underscore.
fn is_alpha(s: &str) -> bool {
    is_lower(s) || is_upper(s) || s == "_"
}

/// Test that a string contains an ASCII alphabetic character, underscore, or decimal digit.
fn is_alphanum(s: &str) -> bool {
    is_alpha(s) || is_digit(s)
}

/// Test that a string contains an ASCII decimal digit.
fn is_digit(s: &str) -> bool {
    match s {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII binary digit.
fn is_bin_digit(s: &str) -> bool {
    match s {
        "0" | "1" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII octal digit.
fn is_oct_digit(s: &str) -> bool {
    match s {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII hexadecimal digit.
fn is_hex_digit(s: &str) -> bool {
    is_digit(s)
        || match s {
            "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F" => true,
            _ => false,
        }
}

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
    UnknownSequence,
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

/// An iterator for producing tokens from a source.
#[derive(Clone)]
pub struct Lexer<'a> {
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

impl<'a> Lexer<'a> {
    /// Create a new lexer for some source.
    ///
    /// # Arguments
    ///
    /// * `source` - The source to lex.
    ///
    /// # Returns
    ///
    /// A new [`Lexer`].
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            prev: None,
            index: 0,
            index_bytes: 0,
            mark: 0,
            mark_bytes: 0,
            source: source,
            graphemes: source.graphemes(true),
        }
    }

    /// Mark the lexer with the current position.
    fn mark(&mut self) {
        self.mark = self.index;
        self.mark_bytes = self.index_bytes;
    }

    /// Get the next grapheme and advance the lexer.
    ///
    /// # Returns
    ///
    /// The next grapheme if not at the end of the lexer.
    fn next(&mut self) -> Option<&'a str> {
        if let Some(g) = self.graphemes.next() {
            self.index += 1;
            self.index_bytes += g.len();
            Some(g)
        } else {
            None
        }
    }

    /// Peek the next grapheme without advancing the lexer.
    ///
    /// # Returns
    ///
    /// The next grapheme if not at the end of the lexer.
    fn peek(&self) -> Option<&'a str> {
        self.clone().graphemes.next()
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
        return count;
    }

    /// Extract the text between the marked and current position.
    fn text(&self) -> &'a str {
        &self.source[self.mark_bytes..self.index_bytes]
    }

    /// Get the span of the marked to current position.
    fn span(&self) -> Span {
        Span::new(self.mark, self.index)
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
    fn extract(&self, kind: TokenKind) -> Option<Result<Token<'a>, LexerError<'a>>> {
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
    fn error(
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
            if let Some(g2) = lexer.graphemes.next() {
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

    /// Advance the lexer to consume exactly one grapheme present in the pattern if it exists at
    /// the current lexer position.
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
        return false;
    }

    /// Lex an exact expression if it matches.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match.
    /// * `kind` - The kind iof the resulting token.
    ///
    /// # Returns
    ///
    /// `Some(Ok(...))` containing a [`Token`] if the pattern was matched, otherwise `None`
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

    /// Lex a space if there is one.
    ///
    /// # Returns
    ///
    /// `Some(Ok(...))` containing a [`Token`] if a space was found, otherwise `None`
    fn lex_space(&mut self) -> Option<Result<Token<'a>, LexerError<'a>>> {
        let mut seen = false;
        while let Some(g) = self.peek() {
            if g == " " || g == "\t" {
                seen = true;
                self.next();
            } else {
                break;
            }
        }
        if seen {
            self.extract(Space)
        } else {
            None
        }
    }

    /// Lex a newline if there is one.
    ///
    /// # Returns
    ///
    /// `Some(Ok(...))` containing a [`Token`] if a newline was found, otherwise `None`
    fn lex_newline(&mut self) -> Option<Result<Token<'a>, LexerError<'a>>> {
        return_some!(self.lex_exact("\n", NewLine));
        return_some!(self.lex_exact("\r\n", NewLine));
        None
    }

    /// Lex a comment if there is one.
    ///
    /// # Returns
    ///
    /// `Some(Ok(...))` containing a [`Token`] if a comment was found, otherwise `None`
    fn lex_comment(&mut self) -> Option<Result<Token<'a>, LexerError<'a>>> {
        // Try to lex block comments
        if self.expect("#[[") {
            let mut depth: usize = 1;
            while let Some(_) = self.peek() {
                if self.expect("]]") {
                    depth -= 1;
                } else {
                    self.next();
                }
                if depth == 0 {
                    break;
                }
            }
            if depth == 0 {
                self.extract(CommentBlock)
            } else {
                self.error(Incomplete, Some(CommentBlock))
            }
        // Try to lex line comments
        } else if self.expect("#") {
            while let Some(_) = self.peek() {
                if let Some(Ok(_)) = self.clone().lex_newline() {
                    break;
                }
            }
            self.extract(CommentLine)
        // There was no comment
        } else {
            None
        }
    }

    /// Lex a name if there is one.
    ///
    /// # Returns
    ///
    /// `Some(Ok(...))` containing a [`Token`] if a name was found, otherwise `None`
    fn lex_name(&mut self) -> Option<Result<Token<'a>, LexerError<'a>>> {
        if let Some(g) = self.peek() {
            if !is_alpha(g) {
                return None;
            }
        } else {
            return None;
        }
        // Eat up underscores
        let count = self.take_while(&|g| g == "_");
        // Test if the name is a non-underscore name
        if let Some(g) = self.peek() {
            // Lext a lowercase name if the next grapheme is a lowercase letter. This implements
            // the regular expression `[a-z][a-zA-Z_0-9]*(!|\?)?`, but marks the result as invalid
            // if an uppercase character is seen.
            if is_lower(g) {
                let mut valid = true;
                while let Some(g2) = self.peek() {
                    if !is_alphanum(g2) {
                        break;
                    }
                    self.next();
                    if is_upper(g2) {
                        valid = false;
                    }
                }
                self.expect_one("!?");
                return if valid {
                    self.extract(NameLower)
                } else {
                    self.error(Invalid, Some(NameLower))
                };
            // Lext an uppercase name if the next grapheme is a uppercase letter. This implements
            // the regular expression `[A-Z][a-zA-Z_0-9]*(!|\?)?`, but marks the result as invalid
            // if the trailing `!` or `?` is seen.
            } else if is_upper(g) {
                self.take_while(&is_alphanum);
                return if !self.expect_one("!?") {
                    self.extract(NameUpper)
                } else {
                    self.error(Invalid, Some(NameUpper))
                };
            // Lex an anonymous name if the next grapheme is a digit. This implements the regular
            // expression `[0-9][a-zA-Z_0-9]*(!|\?)?`, but marks the result as invalid if any
            // non-digit graphemes are seen.
            } else if is_digit(g) {
                let mut valid = true;
                while let Some(g2) = self.peek() {
                    if !is_alphanum(g2) {
                        break;
                    }
                    self.next();
                    if is_alpha(g2) {
                        valid = false;
                    }
                }
                if self.expect_one("!?") {
                    valid = false;
                }
                return if valid {
                    self.extract(NameAnon)
                } else {
                    self.error(Invalid, Some(NameAnon))
                };
            }
        }
        // At this point, the name is known to be all underscores, but the name is only allowed to
        // contain 1 underscore.
        return if count == 1 {
            self.extract(NameUnderscore)
        } else {
            self.error(Invalid, Some(NameUnderscore))
        };
    }

    /// Lex an integer of float literal.
    ///
    /// # Returns
    ///
    /// `Some(Ok(...))` containing a [`Token`] if a number was found, otherwise `None`
    fn lex_number(&mut self) -> Option<Result<Token<'a>, LexerError<'a>>> {
        // Take a digit
        let first_is_zero = if let Some(g) = self.peek() {
            if !is_digit(g) {
                return None;
            }
            self.next();
            g == "0"
        } else {
            return None;
        };
        // Take binary, octal, and hexadecimal literals
        if first_is_zero {
            if let Some(g) = self.peek() {
                // Binary
                if g == "b" {
                    self.next();
                    if self.consume_number(true, &is_bin_digit) > 0 {
                        return self.extract(IntBin);
                    } else {
                        return self.error(Incomplete, Some(IntBin));
                    }
                // Octal
                } else if g == "o" {
                    self.next();
                    if self.consume_number(true, &is_oct_digit) > 0 {
                        return self.extract(IntOct);
                    } else {
                        return self.error(Incomplete, Some(IntOct));
                    }
                // Hex
                } else if g == "x" {
                    self.next();
                    if self.consume_number(true, &is_hex_digit) > 0 {
                        return self.extract(IntHex);
                    } else {
                        return self.error(Incomplete, Some(IntHex));
                    }
                }
            }
        }
        // This is either a decimal integer or a float at this point
        self.consume_number(true, &is_digit);
        let mut kind = IntDec;
        // Eat the fraction if this is is a float
        if let Some(".") = self.peek() {
            if let Some(g) = self.peek() {
                if is_digit(g) {
                    self.consume_number(true, &is_digit);
                    kind = Float;
                }
            }
        }
        // Eat the exponent if present
        if let Some("e") = self.peek() {
            kind = Float;
            self.next();
            self.expect("-");
            if self.consume_number(false, &is_digit) == 0 {
                return self.error(Incomplete, Some(Float));
            }
        }
        return self.extract(kind);
    }

    /// Take graphemes that represent a number.
    ///
    /// # Arguments
    ///
    /// * `underscore` - Indicates that underscores are allowed. An underscore will never be the
    ///     first or last grapheme that is taken.
    /// * `pred` - The predicate that must be matched for a grapheme to be taken. This should not
    ///     include underscores as a match.
    ///
    /// # Returns
    ///
    /// The number of graphemes taken.
    fn consume_number(&mut self, underscore: bool, pred: &dyn Fn(&str) -> bool) -> usize {
        let mut count = 0;
        if let Some(g2) = self.peek() {
            if !pred(g2) {
                return count;
            }
            self.next();
            count += 1;
        }
        count += self.take_while(&pred);
        if underscore {
            while let Some("_") = self.peek() {
                self.next();
                count += 1;
                count += self.take_while(&pred);
            }
        }
        return count;
    }

    /// Lex a string literal.
    ///
    /// # Returns
    ///
    /// `Some(Ok(...))` containing a [`Token`] if a string was found, otherwise `None`
    fn lex_string(&mut self) -> Option<Result<Token<'a>, LexerError<'a>>> {
        // Determine the kind of the string and eat up the delimiters
        let kind = if self.expect("@\"\"\"") {
            StringSmart
        } else if self.expect("\"\"\"") {
            StringMulti
        } else if self.expect("\"") {
            StringSingle
        } else {
            return None;
        };
        // TODO Invalid for single line string to have a line break
        // TODO Invalid escapes
        // TODO Invalid characters
        // Eat up the string contents
        let mut seen_closing = false;
        while let Some(g) = self.peek() {
            // Skip escaped quotes
            if g == "\"" && self.expect("\\\"") {
                continue;
            // Skip take the closing quotes
            } else if let StringSmart | StringMulti = kind {
                if self.expect("\"\"\"") {
                    seen_closing = true;
                    break;
                }
            } else if self.expect("\"") {
                seen_closing = true;
                break;
            }
            // Otherwise take the token in the string
            self.next();
        }
        if !seen_closing {
            self.error(Incomplete, Some(kind))
        } else {
            self.extract(kind)
        }
    }

    /// Lex the next token.
    ///
    /// # Arguments
    ///
    /// * `recursive` - Indicates that this is arecursive call to the function.
    ///
    /// # Returns
    ///
    /// `Some(Ok(...))` containing a [`Token`] if one was found, `Some(Err(...))` if an error
    /// occurred, and `None` if there are no more tokens to left to take.
    fn lex_next(&mut self, recursive: bool) -> Option<Result<Token<'a>, LexerError<'a>>> {
        // There is nothing left to lex
        if let None = self.peek() {
            return None;
        }
        // Mark the starting point of the lexer
        self.mark();
        // Lex the next token
        return_some!(self.lex_space());
        return_some!(self.lex_newline());
        return_some!(self.lex_comment());
        return_some!(self.lex_name());
        return_some!(self.lex_number());
        return_some!(self.lex_string());
        return_some!(self.lex_exact("&&", AmpAmp));
        return_some!(self.lex_exact("&", Amp));
        return_some!(self.lex_exact("@@", AtAt));
        return_some!(self.lex_exact("@{", AtBraceL));
        return_some!(self.lex_exact("@", At));
        return_some!(self.lex_exact("!=", BangEq));
        return_some!(self.lex_exact("!", Bang));
        return_some!(self.lex_exact("[", BracketL));
        return_some!(self.lex_exact("]", BracketR));
        return_some!(self.lex_exact("{", BraceL));
        return_some!(self.lex_exact("}", BraceR));
        return_some!(self.lex_exact("\\", BackSlash));
        return_some!(self.lex_exact("^", Caret));
        return_some!(self.lex_exact("::", ColonColon));
        return_some!(self.lex_exact(":", Colon));
        return_some!(self.lex_exact(",", Comma));
        return_some!(self.lex_exact(".", Dot));
        return_some!(self.lex_exact("==", EqEq));
        return_some!(self.lex_exact("=", Eq));
        return_some!(self.lex_exact(">=", GtEq));
        return_some!(self.lex_exact(">>", GtGt));
        return_some!(self.lex_exact(">", Gt));
        return_some!(self.lex_exact("<=>", LtEqGt));
        return_some!(self.lex_exact("<=", LtEq));
        return_some!(self.lex_exact("<<", LtLt));
        return_some!(self.lex_exact("<", Lt));
        return_some!(self.lex_exact("-", Minus));
        return_some!(self.lex_exact("(", ParenL));
        return_some!(self.lex_exact(")", ParenR));
        return_some!(self.lex_exact("%", Percent));
        return_some!(self.lex_exact("||", PipePipe));
        return_some!(self.lex_exact("|", Pipe));
        return_some!(self.lex_exact("+", Plus));
        return_some!(self.lex_exact("?=", QuestionEq));
        return_some!(self.lex_exact("?", Question));
        return_some!(self.lex_exact(";", Semicolon));
        return_some!(self.lex_exact("//", SlashSlash));
        return_some!(self.lex_exact("/", Slash));
        return_some!(self.lex_exact("**", StarStar));
        return_some!(self.lex_exact("*", Star));
        return_some!(self.lex_exact("~", Tilde));
        // No tokens were matched so an error must be issued
        if recursive {
            // On a recursive call, simply return the error type
            self.next();
            self.error(UnknownSequence, None)
        } else {
            // Coalesce all consecutive failures to match a token into a single error.
            let mut lexer = self.clone();
            while let Some(Err(LexerError {
                kind: UnknownSequence,
                ..
            })) = lexer.lex_next(true)
            {
                self.next();
            }
            self.error(UnknownSequence, None)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex_next(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
    static UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    static UNDERSCORE: &'static str = "_";
    static DIGITS: &'static str = "0123456789";
    static BIN_DIGITS: &'static str = "01";
    static OCT_DIGITS: &'static str = "01234567";
    static HEX_DIGITS: &'static str = "0123456789abcdefABCDEF";
    // This is okay to split across multiple lines because space and linebreaks are printable
    static PRINTABLE: &'static str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[
        \\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

    macro_rules! concat {
        ($( $s:expr ),*) => {
            {
                let mut s = String::new();
                $(
                    s.push_str($s);
                )*
                s
            }
        };
    }

    macro_rules! remove {
        ($s:expr, $( $pattern:expr ),*) => {
            {
                let mut s: String = $s.into();
                $(
                    for g in $pattern.graphemes(true) {
                        s = s.replace(g, "");
                    }
                )*
                s
            }
        };
    }

    #[test]
    fn test_is_lower() {
        for g in LOWER.graphemes(true) {
            assert!(is_lower(g));
        }
        for g in remove!(PRINTABLE, LOWER).graphemes(true) {
            assert!(!is_lower(g));
        }
    }

    #[test]
    fn test_is_upper() {
        for g in UPPER.graphemes(true) {
            assert!(is_upper(g));
        }
        for g in remove!(PRINTABLE, UPPER).graphemes(true) {
            assert!(!is_upper(g));
        }
    }

    #[test]
    fn test_is_alpha() {
        let pattern = concat!(LOWER, UPPER, UNDERSCORE);
        for g in pattern.graphemes(true) {
            assert!(is_alpha(g));
        }
        for g in remove!(PRINTABLE, pattern).graphemes(true) {
            assert!(!is_alpha(g));
        }
    }

    #[test]
    fn test_is_alphanum() {
        let pattern = concat!(LOWER, UPPER, UNDERSCORE, DIGITS);
        for g in pattern.graphemes(true) {
            assert!(is_alphanum(g));
        }
        for g in remove!(PRINTABLE, pattern).graphemes(true) {
            assert!(!is_alphanum(g));
        }
    }

    #[test]
    fn test_is_digit() {
        for g in DIGITS.graphemes(true) {
            assert!(is_digit(g));
        }
        for g in remove!(PRINTABLE, DIGITS).graphemes(true) {
            assert!(!is_digit(g));
        }
    }

    #[test]
    fn test_is_bin_digit() {
        for g in BIN_DIGITS.graphemes(true) {
            assert!(is_bin_digit(g));
        }
        for g in remove!(PRINTABLE, BIN_DIGITS).graphemes(true) {
            assert!(!is_bin_digit(g));
        }
    }

    #[test]
    fn test_is_oct_digit() {
        for g in OCT_DIGITS.graphemes(true) {
            assert!(is_oct_digit(g));
        }
        for g in remove!(PRINTABLE, OCT_DIGITS).graphemes(true) {
            assert!(!is_oct_digit(g));
        }
    }

    #[test]
    fn test_is_hex_digit() {
        for g in HEX_DIGITS.graphemes(true) {
            assert!(is_hex_digit(g));
        }
        for g in remove!(PRINTABLE, HEX_DIGITS).graphemes(true) {
            assert!(!is_hex_digit(g));
        }
    }

    // TODO Test the lexer
}
