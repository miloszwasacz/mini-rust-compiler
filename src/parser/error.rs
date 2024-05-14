//! Error types for the parser.

use std::error::Error;
use std::fmt;
use std::rc::Rc;

use crate::lexer::error::LexerError;
use crate::token::Token;

/// The type of error that can occur during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    /// An error that occurred during lexing.
    LexicalError(LexerError),
    /// Unexpected end of file.
    UnexpectedEOF,
    /// An unexpected token was encountered.
    UnexpectedToken(Token),
    /// An unsupported ABI was encountered.
    UnsupportedAbi(Rc<str>),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //TODO Implement fmt::Display for ParserError
        unimplemented!()
    }
}

impl Error for ParserError {}

impl From<LexerError> for ParserError {
    fn from(error: LexerError) -> Self {
        ParserError::LexicalError(error)
    }
}
