//! A module containing all production rules for the parser.

use either::Either;
use fallible_iterator::FallibleIterator;

use crate::ast::error::SemanticError;
use crate::ast::{
    ASTNode, BlockASTNode, BlockReturnExpr, CrateASTNode, ExprASTNode, ExprStmtASTNode,
    ExternASTNode, ExternItem, FunCallASTNode, FuncASTNode, FuncProtoASTNode, GroupedExprASTNode,
    ItemASTNode, LetASTNode, LiteralASTNode, LiteralBox, ParamASTNode, PathASTNode, ReturnASTNode,
    Statements, StaticASTNode, Type, TypeASTMetaNode, UnderscoreASTNode,
};
use crate::parser::error::{ParserError, RecoverableParserError};
use crate::parser::{Parser, Result};
use crate::token::{Position, Span, Token, TokenType::*};

use self::expr_kind::*;
use self::macros::*;

mod expr_kind;
mod macros;
mod ops;

//TODO Get rid of this
#[allow(clippy::missing_docs_in_private_items)]
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

    /// Pushes a recoverable error into the parser's error list.
    fn push_rcv_error(&mut self, error: RecoverableParserError) {
        self.errors.push(error);
    }

    //TODO Improve documentation
    /// Parses the input file into a [`CrateASTNode`], consuming the `Parser`.
    pub(super) fn parse_crate(mut self) -> Result<CrateASTNode> {
        let items = self.parse_items()?;

        if !self.errors.is_empty() {
            return Err(ParserError::Aggregated(self.errors));
        }

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
            Static => ItemASTNode::Static(Box::new(self.parse_static(false)?)),
            Extern => ItemASTNode::Extern(Box::new(self.parse_extern()?)),
            _ => return unknown_token!(self),
        })
    }

    fn parse_func(&mut self) -> Result<FuncASTNode> {
        let proto = self.parse_func_proto()?;
        let body = self.parse_block_expr()?;
        let span = Span::new(proto.span().start(), body.span().end());
        Ok(FuncASTNode::new(proto, body, span))
    }

    fn parse_func_proto(&mut self) -> Result<FuncProtoASTNode> {
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

        let span = Span::new(start_pos, end_pos);
        Ok(FuncProtoASTNode::new(ident, params, ret_ty, span))
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
        let mutability = self.parse_mut()?;
        let ident = assert_ident_or_underscore!(self);

        // FunctionParameter'' rule
        assert_token!(self, Colon);
        let ty = self.parse_type()?;

        let assignee: Box<dyn ExprASTNode> = match ident {
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

    fn parse_mut(&mut self) -> Result<bool> {
        let token = self.peek()?;
        if let Mut = token.ty() {
            self.consume().expect("Mut token should be present.");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn parse_item_assignment(&mut self) -> Result<Option<Box<dyn ParserExpr>>> {
        let next = self.peek()?;
        match next.ty() {
            Assign => {
                self.consume().expect("Assign token should be present.");
                let expr = self.parse_expr()?;
                Ok(Some(expr))
            }
            Semi => Ok(None),
            _ => unknown_token!(self),
        }
    }

    fn parse_static(&mut self, is_extern: bool) -> Result<StaticASTNode> {
        let start_pos = assert_token!(self, Static).start();

        let mutability = self.parse_mut()?;
        let ident = assert_ident!(self);

        assert_token!(self, Colon);
        let ty = self.parse_type()?;
        let value = self.parse_item_assignment()?.map(ParserExpr::into_expr);

        let end_pos = assert_token!(self, Semi).end();
        let span = Span::new(start_pos, end_pos);

        let item = match value {
            Some(value) => {
                if is_extern {
                    self.push_rcv_error(
                        SemanticError::ExternStaticWithInitializer { span: value.span() }.into(),
                    );
                }
                StaticASTNode::new_with_assignment(ident, value, ty, mutability, span)
            }
            None => {
                if !is_extern {
                    self.push_rcv_error(SemanticError::StaticWithoutInitializer { span }.into());
                }
                StaticASTNode::new(ident, ty, mutability, span)
            }
        };
        Ok(item)
    }

    fn parse_extern(&mut self) -> Result<ExternASTNode> {
        let start_pos = assert_token!(self, Extern).start();
        let next = self.consume()?;
        let abi = match next.ty() {
            //TODO Add support for other ABIs
            Abi(abi) => match abi.as_ref() {
                "C" => abi.clone(),
                _ => {
                    self.push_rcv_error(RecoverableParserError::UnsupportedAbi(abi.clone()));
                    abi.clone()
                }
            },
            _ => return unknown_token!(self, next),
        };

        assert_token!(self, LBra);
        let items = self.parse_extern_items()?;
        let end_pos = assert_token!(self, RBra).end();

        let span = Span::new(start_pos, end_pos);
        Ok(ExternASTNode::new(abi, items, span))
    }

    fn parse_extern_items(&mut self) -> Result<Vec<ExternItem>> {
        let mut result = Vec::new();
        loop {
            let next = self.peek()?;
            match next.ty() {
                Fn => {
                    let item = self.parse_extern_func()?;
                    result.push(ExternItem::Func(Box::new(item)));
                }
                Static => {
                    let item = self.parse_static(true)?;
                    result.push(ExternItem::Static(Box::new(item)));
                }
                RBra => return Ok(result),
                _ => return unknown_token!(self),
            }
        }
    }

    fn parse_extern_func(&mut self) -> Result<FuncProtoASTNode> {
        let proto = self.parse_func_proto()?;

        let next = self.peek()?;
        match next.ty() {
            Semi => {
                assert_token!(self, Semi);
            }
            LBra => {
                let body_span = self.parse_block_expr()?.span();
                self.push_rcv_error(
                    SemanticError::ExternFunctionWithBody { span: body_span }.into(),
                )
            }
            _ => return unknown_token!(self),
        }

        Ok(proto)
    }

    fn parse_let_stmt(&mut self) -> Result<LetASTNode> {
        let start_pos = assert_token!(self, Let).start();

        //TODO Add support for destructuring
        let mutability = self.parse_mut()?;
        let ident = assert_ident!(self);

        //TODO Add support for type inference
        assert_token!(self, Colon);
        let ty = self.parse_type()?;
        let val = self.parse_item_assignment()?.map(ParserExpr::into_expr);

        let end_pos = match &val {
            Some(val) => val.span().end(),
            None => ty.span().end(),
        };
        let end_pos = match expect_token!(self, Semi) {
            Some(span) => span.end(),
            None => {
                self.push_rcv_error(RecoverableParserError::MissingToken(Semi, end_pos));
                end_pos
            }
        };
        let span = Span::new(start_pos, end_pos);

        let assignee = Box::new(PathASTNode::new(ident, span));
        let let_stmt = match val {
            Some(val) => LetASTNode::new_with_assignment(assignee, ty, val, mutability, span),
            None => LetASTNode::new(assignee, ty, mutability, span),
        };

        Ok(let_stmt)
    }

    // This production rule is not present in the transformed grammar
    // because (I believe) it cannot be represented as production rule in a context-free grammar.
    /// Returns (expr_stmt, had_trailing_semi)
    fn parse_expr_stmt(&mut self) -> Result<(ExprStmtASTNode, bool)> {
        let expr = self.parse_expr_wo_block()?;

        let semi = expect_token!(self, Semi);
        let end_pos = match semi {
            Some(span) => span.end(),
            None => expr.span().end(),
        };
        let span = Span::new(expr.span().start(), end_pos);

        let expr_stmt = ExprStmtASTNode::new(expr.into_expr(), span);
        Ok((expr_stmt, semi.is_none()))
    }

    fn parse_expr(&mut self) -> Result<Box<dyn ParserExpr>> {
        self.parse_expr_wo_block()
    }

    fn parse_expr_wo_block(&mut self) -> Result<Box<dyn ParserExpr>> {
        unimplemented!();
    }

    fn parse_expr_wo_block_(&mut self) -> Result<Box<dyn ParserExpr>> {
        unimplemented!()
    }

    fn parse_expr_w_block(&mut self) -> Result<Box<dyn ParserExpr>> {
        unimplemented!();
    }

    fn parse_literal_expr(&mut self) -> Result<LiteralBox> {
        /// A macro to create a boxed literal node.
        macro_rules! box_literal {
            ($box_ty:ident, $ty:ty, $val:expr, $span:expr) => {{
                let literal = LiteralASTNode::<$ty>::new($val, $span);
                Ok(LiteralBox::$box_ty(Box::new(literal)))
            }};
        }

        let token = self.consume()?;
        match token.ty() {
            //TODO Add support for different sizes of ints and floats
            IntLit(val) => box_literal!(I32, i32, *val, token.span()),
            FloatLit(val) => box_literal!(F64, f64, *val, token.span()),
            BoolLit(val) => box_literal!(Bool, bool, *val, token.span()),
            _ => unknown_token!(self, token),
        }
    }

    fn parse_path_or_call_expr(&mut self) -> Result<Box<dyn ParserExpr>> {
        let path = Box::new(self.parse_path_expr()?);
        let next = self.peek()?;
        Ok(match next.ty() {
            LPar => {
                // CallExpression' rule
                assert_token!(self, LPar);

                let params = self.parse_call_params()?;

                let end_pos = assert_token!(self, RPar).end();
                let span = Span::new(path.span().start(), end_pos);

                Box::new(FunCallASTNode::new(path, params, span))
            }
            _ => path,
        })
    }

    fn parse_path_expr(&mut self) -> Result<PathASTNode> {
        //TODO Add support for more complex paths
        let token = self.consume()?;
        match token.ty() {
            Ident(ident) => Ok(PathASTNode::new(ident.clone(), token.span())),
            _ => unknown_token!(self, token),
        }
    }

    fn parse_block_expr(&mut self) -> Result<BlockASTNode> {
        let start_pos = assert_token!(self, LBra).start();

        let (stmts, return_expr) = self.parse_stmts()?;

        let end_pos = assert_token!(self, RBra).end();
        let span = Span::new(start_pos, end_pos);

        Ok(match return_expr {
            Some(return_expr) => BlockASTNode::new_with_return(stmts, return_expr, span),
            None => BlockASTNode::new(stmts, span),
        })
    }

    fn parse_stmts(&mut self) -> Result<(Statements, BlockReturnExpr)> {
        let mut statements: Statements = Vec::new();
        loop {
            let next = self.peek()?;
            match next.ty() {
                Let => {
                    let stmt = self.parse_let_stmt()?;
                    statements.push(Box::new(stmt));
                }
                Semi => {
                    assert_token!(self, Semi);
                    continue;
                }
                RBra => return Ok((statements, None)),
                _ => {
                    let (expr_stmt, is_last) = self.parse_expr_stmt()?;
                    if is_last {
                        let return_expr = Some(expr_stmt.into_expr());
                        return Ok((statements, return_expr));
                    } else {
                        statements.push(Box::new(expr_stmt));
                    }
                }
            }
        }
    }

    fn parse_operator_expr(&mut self) -> Result<Box<dyn ParserExpr>> {
        ops::parse_ops(self)
    }

    fn parse_grouped_expr_or_unit_lit(
        &mut self,
    ) -> Result<Either<GroupedExprASTNode, LiteralASTNode<()>>> {
        let start_pos = assert_token!(self, LPar).start();

        // GroupedOrUnit rule
        let next = self.peek()?;
        Ok(match next.ty() {
            RPar => {
                let end_pos = assert_token!(self, RPar).end();
                let span = Span::new(start_pos, end_pos);
                Either::Right(LiteralASTNode::<()>::new(span))
            }
            _ => {
                let expr = self.parse_expr()?;
                let end_pos = assert_token!(self, RPar).end();
                let span = Span::new(start_pos, end_pos);

                let expr = GroupedExprASTNode::new(expr.into_expr(), span);
                Either::Left(expr)
            }
        })
    }

    fn parse_call_params(&mut self) -> Result<Vec<Box<dyn ExprASTNode>>> {
        unimplemented!()
    }

    //TODO Implement production rules

    fn parse_return(&mut self) -> Result<ReturnASTNode> {
        let span = assert_token!(self, Return);

        // ReturnExpressionTail' rule
        let next = self.peek()?;
        Ok(match next.ty() {
            Return | Minus | Not | IntLit(_) | FloatLit(_) | BoolLit(_) | LPar | Underscore
            | LBra | If | Unsafe | Ident(_) | Loop | While => {
                let expr = self.parse_expr()?.into_expr();
                let span = Span::new(span.start(), expr.span().end());

                ReturnASTNode::new(expr, span)
            }
            RPar | Comma | RBra | Semi => ReturnASTNode::empty(span),
            _ => return unknown_token!(self),
        })
    }

    fn parse_underscore_expr(&mut self) -> Result<UnderscoreASTNode> {
        let span = assert_token!(self, Underscore);
        Ok(UnderscoreASTNode::new(span))
    }

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
