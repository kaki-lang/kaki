//! Utility functions for working with text.

/// Test that a string contains an ASCII lowercase character.
pub fn is_lower(s: &str) -> bool {
    match s {
        "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o"
        | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII uppercase character.
pub fn is_upper(s: &str) -> bool {
    match s {
        "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O"
        | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII alphabetic character or underscore.
pub fn is_alpha(s: &str) -> bool {
    is_lower(s) || is_upper(s) || s == "_"
}

/// Test that a string contains an ASCII alphabetic character, underscore, or decimal digit.
pub fn is_alphanum(s: &str) -> bool {
    is_alpha(s) || is_digit(s)
}

/// Test that a string contains an ASCII decimal digit.
pub fn is_digit(s: &str) -> bool {
    match s {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII binary digit.
pub fn is_bin_digit(s: &str) -> bool {
    match s {
        "0" | "1" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII octal digit.
pub fn is_oct_digit(s: &str) -> bool {
    match s {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" => true,
        _ => false,
    }
}

/// Test that a string contains an ASCII hexadecimal digit.
pub fn is_hex_digit(s: &str) -> bool {
    is_digit(s)
        || match s {
            "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F" => true,
            _ => false,
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
}
