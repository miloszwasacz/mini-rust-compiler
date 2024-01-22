//! Helper functions for the lexer.

/// Checks if the character is a whitespace, i.e. one of the following:
/// - Horizontal tab (U+0009, '\t')
/// - Line feed (U+000A, '\n')
/// - Vertical tab (U+000B)
/// - Form feed (U+000C)
/// - Carriage return (U+000D, '\r')
/// - Space (U+0020, ' ')
/// - Next line (U+0085)
/// - Left-to-right mark (U+200E)
/// - Right-to-left mark (U+200F)
/// - Line separator (U+2028)
/// - Paragraph separator (U+2029)
pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n'
            | '\u{000B}'
            | '\u{000C}'
            | '\r'
            | ' '
            | '\u{0085}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{2028}'
            | '\u{2029}'
    )
}

/// Checks if the character is a new line, i.e. '\n'.
pub fn is_new_line(c: char) -> bool {
    c == '\n'
}

/// Checks if the character is an ASCII digit, i.e. matches the regex `[0-9]`.
/// This is equivalent to [char::is_ascii_digit()].
pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_whitespace() {
        assert!(is_whitespace('\t'));
        assert!(is_whitespace('\n'));
        assert!(is_whitespace('\u{000B}'));
        assert!(is_whitespace('\u{000C}'));
        assert!(is_whitespace('\r'));
        assert!(is_whitespace(' '));
        assert!(is_whitespace('\u{0085}'));
        assert!(is_whitespace('\u{200E}'));
        assert!(is_whitespace('\u{200F}'));
        assert!(is_whitespace('\u{2028}'));
        assert!(is_whitespace('\u{2029}'));
        assert!(!is_whitespace('a'));
        assert!(!is_whitespace('.'));
    }

    #[test]
    fn test_is_new_line() {
        assert!(is_new_line('\n'));
        assert!(!is_new_line('\t'));
        assert!(!is_new_line('\r'));
        assert!(!is_new_line('n'));
    }

    #[test]
    fn test_is_digit() {
        assert!(is_digit('0'));
        assert!(is_digit('1'));
        assert!(is_digit('2'));
        assert!(is_digit('3'));
        assert!(is_digit('4'));
        assert!(is_digit('5'));
        assert!(is_digit('6'));
        assert!(is_digit('7'));
        assert!(is_digit('8'));
        assert!(is_digit('9'));
        assert!(!is_digit('a'));
        assert!(!is_digit('.'));
    }
}
