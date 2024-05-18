//! Error types for the lexer.

use std::error::Error;
use std::fmt;

use crate::token::{Position, Span};

/// The type of error that can occur during lexing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerErrorKind {
    /// The integer literal has an invalid format.
    InvalidIntLiteral(Box<str>),
    /// The float literal has an invalid format.
    InvalidFloatLiteral(Box<str>),
    /// The identifier contains invalid characters.
    InvalidIdentifier {
        /// The identifier that caused the error.
        ident: Box<str>,
        /// The positions of the invalid characters.
        invalid_char_pos: Vec<Position>,
    },
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
            LexerErrorKind::InvalidIdentifier {
                ident,
                invalid_char_pos,
            } => {
                write!(
                    f,
                    r#"Invalid identifier "{}" at {} with invalid characters at {}"#,
                    ident,
                    self.span,
                    invalid_char_pos
                        .iter()
                        .map(|pos| pos.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
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
