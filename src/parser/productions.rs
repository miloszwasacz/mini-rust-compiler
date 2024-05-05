//! A module containing all production rules for the parser.

use fallible_iterator::FallibleIterator;

use crate::ast::CrateASTNode;
use crate::parser::error::ParserError;
use crate::parser::{Parser, Result};
use crate::token::Token;

impl Parser {
    /// Consumes the next token from the lexer.
    fn consume(&mut self) -> Result<Token> {
        match self.lexer.next() {
            Ok(None) => Err(ParserError::UnexpectedEOF),
            Ok(Some(t)) => Ok(t),
            Err(e) => Err(e.into()),
        }
    }

    /// Peeks at the next token from the lexer without consuming it.
    fn peek(&mut self) -> Result<&Token> {
        match self.lexer.peek() {
            Ok(None) => Err(ParserError::UnexpectedEOF),
            Ok(Some(t)) => Ok(t),
            Err(e) => Err(e.into()),
        }
    }

    //TODO Improve documentation
    /// Parses the input file into a `CrateASTNode`, consuming the `Parser`.
    pub(super) fn into_crate_ast(self) -> Result<CrateASTNode> {
        unimplemented!()
    }

    //TODO Implement production rules
}
