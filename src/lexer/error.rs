//! Error types for the lexer.

use std::error::Error;
use std::fmt;

use crate::token::Span;

/// The type of error that can occur during lexing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerErrorKind {
    /// The integer literal has an invalid format.
    InvalidIntLiteral(Box<str>),
    /// The float literal has an invalid format.
    InvalidFloatLiteral(Box<str>),
    /// The string literal is not terminated (i.e. it is missing a closing quote).
    UnterminatedStringLiteral,
    /// An unknown token was encountered.
    UnknownToken(char),
}

/// An error that can occur during lexing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    kind: LexerErrorKind,
    span: Span,
}

impl LexerError {
    /// Creates a new lexer error of a specific kind at the given span.
    pub fn new(kind: LexerErrorKind, span: Span) -> LexerError {
        LexerError { kind, span }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LexerErrorKind::InvalidIntLiteral(s) => {
                write!(f, r#"Invalid integer literal "{}" at {}"#, s, self.span)
            }
            LexerErrorKind::InvalidFloatLiteral(s) => {
                write!(f, r#"Invalid float literal "{}" at {}"#, s, self.span)
            }
            LexerErrorKind::UnterminatedStringLiteral => {
                write!(
                    f,
                    "Unterminated string literal starting at {}",
                    self.span.start()
                )
            }
            LexerErrorKind::UnknownToken(c) => {
                write!(f, r#"Unknown token "{}" at {}"#, *c as u32, self.span)
            }
        }
    }
}

impl Error for LexerError {}
