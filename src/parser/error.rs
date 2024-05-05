//! Error types for the parser.

use crate::lexer::error::LexerError;

/// The type of error that can occur during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    /// An error that occurred during lexing.
    LexicalError(LexerError),
    /// Unexpected end of file.
    UnexpectedEOF,
}

impl From<LexerError> for ParserError {
    fn from(error: LexerError) -> Self {
        ParserError::LexicalError(error)
    }
}
