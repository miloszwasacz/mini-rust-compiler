//! A module containing all production rules for the parser.

use fallible_iterator::FallibleIterator;

use crate::ast::{
    CrateASTNode, ExternASTNode, FuncASTNode, ItemASTNode, ParamASTNode, StaticASTNode,
};
use crate::parser::error::ParserError;
use crate::parser::{Parser, Result};
use crate::token::{Position, Span, Token, TokenType::*};

macro_rules! unknown_token {
    ($self:expr) => {{
        let token = $self
            .consume()
            .expect("Unsuccessful consuming after successful peeking should be impossible.");
        unknown_token!($self, token)
    }};
    ($self:expr, $token:expr) => {
        Err(ParserError::UnexpectedToken($token))
    };
}

macro_rules! assert_token {
    ($self:expr, $expected:pat) => {{
        let token = $self.consume()?;
        assert_token!($self, token, $expected)
    }};
    ($self:expr, $token:expr, $expected:pat) => {
        match $token.ty() {
            $expected => {}
            _ => unknown_token!($self, $token)?,
        }
    };
}

macro_rules! assert_ident {
    ($self:expr) => {{
        let token = $self.consume()?;
        assert_ident!($self, token)
    }};
    ($self:expr, $token:expr) => {
        match $token.ty() {
            Ident(ident) => ident.clone(),
            _ => unknown_token!($self, $token)?,
        }
    };
}

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
        let items = self.parse_items(Vec::new())?;

        let end_pos = match self.consume() {
            Ok(t) if t.is_eof() => t.span().end(),
            Ok(t) => panic!("Unexpected token: {:?} - expected EOF.", t.ty()),
            Err(e) => panic!("No finishing EOF token found.\nError: {}", e),
        };
        let span = Span::new(Position::new(), end_pos);
        let name = self.filename.clone();

        Ok(CrateASTNode::new(name, items, span))
    }

    fn parse_items(&mut self, mut current_items: Vec<ItemASTNode>) -> Result<Vec<ItemASTNode>> {
        let next = self.peek()?;
        match next.ty() {
            Fn | Static | Extern => {
                let item = self.parse_item()?;
                current_items.push(item);
                self.parse_items(current_items)
            }
            EOF => Ok(current_items),
            _ => unknown_token!(self),
        }
    }

    fn parse_item(&mut self) -> Result<ItemASTNode> {
        let next = self.peek()?;
        Ok(match next.ty() {
            Fn => ItemASTNode::Func(Box::new(self.parse_func()?)),
            Static => ItemASTNode::Static(Box::new(self.parse_static()?)),
            Extern => ItemASTNode::Extern(Box::new(self.parse_extern()?)),
            _ => return unknown_token!(self),
        })
    }

    fn parse_func(&mut self) -> Result<FuncASTNode> {
        unimplemented!()
    }

    fn parse_static(&mut self) -> Result<StaticASTNode> {
        unimplemented!()
    }

    fn parse_extern(&mut self) -> Result<ExternASTNode> {
        unimplemented!()
    }

    fn parse_func_params(
        &mut self,
        current_params: Vec<ParamASTNode>,
    ) -> Result<Vec<ParamASTNode>> {
        unimplemented!()
    }

    //TODO Implement production rules
}
