//! A module containing all production rules for the parser.

use fallible_iterator::FallibleIterator;

use crate::ast::{
    ASTNode, AssigneeExprASTNode, BlockASTNode, CrateASTNode, ExternASTNode, FuncASTNode,
    FuncProtoASTNode, ItemASTNode, ParamASTNode, PathASTNode, StaticASTNode, Type, TypeASTMetaNode,
    UnderscoreASTNode,
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
            $expected => $token.span(),
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

macro_rules! assert_ident_or_underscore {
    ($self:expr) => {{
        let token = $self.consume()?;
        assert_ident_or_underscore!($self, token)
    }};
    ($self:expr, $token:expr) => {
        match $token.ty() {
            Ident(ident) => Some(ident.clone()),
            Underscore => None,
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
        let mut result = Vec::new();
        loop {
            let next = self.peek()?;
            match next.ty() {
                Fn | Static | Extern => {
                    let item = self.parse_item()?;
                    result.push(item);
                }
                EOF => return Ok(result),
                _ => return unknown_token!(self),
            }
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
        let start_pos = assert_token!(self, Fn).start();

        let ident = assert_ident!(self);

        assert_token!(self, LPar);

        let params = self.parse_func_params()?;

        // If there is no return type, the prototype ends with the closing parenthesis.
        let mut end_pos = assert_token!(self, RPar).end();

        // If there is a return type, the prototype ends with the return type.
        let ret_ty = self.parse_func_ret_ty()?;
        let ret_ty = match ret_ty {
            Some(node) => {
                end_pos = node.span().end();
                node
            }
            None => TypeASTMetaNode::new(Type::Unit, Span::new(end_pos, end_pos)),
        };

        let body = self.parse_block_expr()?;

        let proto_span = Span::new(start_pos, end_pos);
        let func_span = Span::new(start_pos, body.span().end());

        let proto = FuncProtoASTNode::new(ident, params, ret_ty, proto_span);
        Ok(FuncASTNode::new(proto, body, func_span))
    }

    fn parse_static(&mut self) -> Result<StaticASTNode> {
        unimplemented!()
    }

    fn parse_extern(&mut self) -> Result<ExternASTNode> {
        unimplemented!()
    }

    fn parse_func_params(&mut self) -> Result<Vec<ParamASTNode>> {
        let mut result = Vec::new();
        loop {
            // FunctionParameters rule
            let next = self.peek()?;
            match next.ty() {
                Mut | Underscore | Ident(_) => {
                    let param = self.parse_param()?;
                    result.push(param);
                }
                RPar => return Ok(result),
                _ => return unknown_token!(self),
            }

            // FunctionParameters' rule
            let next = self.peek()?;
            match next.ty() {
                Comma => assert_token!(self, Comma),
                RPar => return Ok(result),
                _ => return unknown_token!(self),
            };
        }
    }

    //noinspection GrazieInspection
    fn parse_param(&mut self) -> Result<ParamASTNode> {
        // FunctionParameter + FunctionParameter' rules
        let token = self.consume()?;
        let (mutability, ident) = match token.ty() {
            Mut => {
                let ident = assert_ident_or_underscore!(self);
                (true, ident)
            }
            Underscore => (false, None),
            Ident(ident) => (false, Some(ident.clone())),
            _ => return unknown_token!(self, token),
        };

        // FunctionParameter'' rule
        assert_token!(self, Colon);
        let ty = self.parse_type()?;

        let assignee: Box<dyn AssigneeExprASTNode> = match ident {
            None => Box::new(UnderscoreASTNode::new(token.span())),
            Some(ident) => Box::new(PathASTNode::new(ident, token.span())),
        };
        Ok(ParamASTNode::new(assignee, ty, mutability, token.span()))
    }

    fn parse_func_ret_ty(&mut self) -> Result<Option<TypeASTMetaNode>> {
        let next = self.peek()?;
        match next.ty() {
            Semi | LBra => Ok(None),
            Arrow => {
                self.consume().expect("Arrow token should be present.");
                self.parse_type().map(Some)
            }
            _ => unknown_token!(self),
        }
    }

    fn parse_block_expr(&mut self) -> Result<BlockASTNode> {
        unimplemented!()
    }

    //TODO Implement production rules

    fn parse_type(&mut self) -> Result<TypeASTMetaNode> {
        let token = self.consume()?;
        match token.ty() {
            Ident(ident) => match ident.parse::<Type>() {
                Ok(ty) => Ok(TypeASTMetaNode::new(ty, token.span())),
                Err(_) => unknown_token!(self, token),
            },
            LPar => {
                //TODO Add support for tuples
                let end_pos = assert_token!(self, RPar).end();
                let span = Span::new(token.span().start(), end_pos);
                Ok(TypeASTMetaNode::new(Type::Unit, span))
            }
            _ => {
                //TODO Add support for other symbol-based types (e.g. references, slices, etc.)
                unknown_token!(self, token)
            }
        }
    }
}
