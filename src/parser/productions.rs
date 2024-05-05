//! A module containing all production rules for the parser.

use fallible_iterator::FallibleIterator;

use crate::ast::{CrateASTNode, ItemASTNode};
use crate::parser::{Parser, Result};
use crate::parser::error::ParserError;
use crate::token::{Position, Span, Token};

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
    pub(super) fn parse_crate(mut self) -> Result<CrateASTNode> {
        let items = self.parse_items()?;

        let end_pos = match self.consume() {
            Ok(t) if t.is_eof() => t.span().end(),
            Ok(t) => panic!("Unexpected token: {:?} - expected EOF.", t.ty()),
            Err(e) => panic!("No finishing EOF token found.\nError: {}", e),
        };
        let span = Span::new(Position::new(), end_pos);
        let name = self.filename.clone();

        Ok(CrateASTNode::new(name, items, span))
    }

    fn parse_items(&mut self) -> Result<Vec<ItemASTNode>> {
        unimplemented!()
    }

    //TODO Implement production rules
}
