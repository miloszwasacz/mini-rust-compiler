//! Error types for the parser.

use std::error::Error;
use std::fmt;
use std::rc::Rc;

use crate::ast::error::SemanticError;
use crate::lexer::error::LexerError;
use crate::token::{Position, Token, TokenType};

/// The type of error that can occur during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    /// A list of errors that occurred during parsing,
    /// but did not prevent the parser from continuing.
    Aggregated(Vec<RecoverableParserError>),
    /// An error that occurred during lexing.
    LexicalError(LexerError),
    /// Unexpected end of file.
    UnexpectedEOF,
    /// An unexpected token was encountered.
    UnexpectedToken {
        /// The unexpected token.
        actual: Token,
        /// The expected token type.
        expected: &'static str,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Aggregated(errs) => {
                writeln!(f, "Multiple errors occurred during parsing:")?;
                for err in errs {
                    writeln!(f, "{}", err)?;
                }
                Ok(())
            }
            ParserError::LexicalError(err) => fmt::Display::fmt(err, f),
            ParserError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            ParserError::UnexpectedToken { actual, expected } => {
                write!(f, "Expected {}, got {}", expected, actual)
            }
        }
    }
}

impl Error for ParserError {}

impl From<LexerError> for ParserError {
    fn from(error: LexerError) -> Self {
        ParserError::LexicalError(error)
    }
}

/// The type of error that can occur during parsing
/// but does not prevent the parser from continuing.
#[derive(Debug, Clone, PartialEq)]
pub enum RecoverableParserError {
    /// A semantic error.
    SemanticError(SemanticError),
    /// Missing a required token.
    MissingToken(TokenType, Position),
    /// An unsupported ABI was encountered.
    UnsupportedAbi(Rc<str>),
}

impl fmt::Display for RecoverableParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoverableParserError::SemanticError(err) => fmt::Display::fmt(err, f),
            RecoverableParserError::MissingToken(expected, pos) => {
                write!(f, "Expected token {:?} at position {}", expected, pos)
            }
            RecoverableParserError::UnsupportedAbi(abi) => {
                write!(f, "Unsupported ABI: {}", abi)
            }
        }
    }
}

impl Error for RecoverableParserError {}

impl From<SemanticError> for RecoverableParserError {
    fn from(error: SemanticError) -> Self {
        RecoverableParserError::SemanticError(error)
    }
}
