//! A module containing all production rules for the parser.

use either::Either;
use fallible_iterator::FallibleIterator;

use crate::ast::error::SemanticError;
use crate::ast::*;
use crate::parser::error::{ParserError, RecoverableParserError};
use crate::parser::{Parser, Result};
use crate::token::{Position, Span, Token, TokenType::*};

use self::macros::*;

mod macros;
mod ops;

//TODO Refactor usages of unknown_token! to specify the expected token type better

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
                _ => return unknown_token!(self, "<item>"),
            }
        }
    }

    fn parse_item(&mut self) -> Result<ItemASTNode> {
        let next = self.peek()?;
        Ok(match next.ty() {
            Fn => ItemASTNode::Func(Box::new(self.parse_func()?)),
            Static => ItemASTNode::Static(Box::new(self.parse_static(false)?)),
            Extern => ItemASTNode::Extern(Box::new(self.parse_extern()?)),
            _ => return unknown_token!(self, "<item>"),
        })
    }

    fn parse_func(&mut self) -> Result<FuncASTNode> {
        let proto = self.parse_func_proto()?;
        let body = self.parse_block_expr()?;
        let span = Span::new(proto.span().start(), body.span().end());
        Ok(FuncASTNode::new(proto, body, span))
    }

    fn parse_func_proto(&mut self) -> Result<FuncProtoASTNode> {
        let start_pos = assert_token!(self, Fn, "'fn'").start();

        let ident = assert_ident!(self, "<ident>");

        assert_token!(self, LPar, "'('");

        let params = self.parse_func_params()?;

        // If there is no return type, the prototype ends with the closing parenthesis.
        let mut end_pos = assert_token!(self, RPar, "')'").end();

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
                _ => return unknown_token!(self, "<fn parameter>"),
            }

            // FunctionParameters' rule
            let next = self.peek()?;
            match next.ty() {
                Comma => assert_token!(self, Comma, "','"),
                RPar => return Ok(result),
                _ => return unknown_token!(self, "',', ')'"),
            };
        }
    }

    fn parse_param(&mut self) -> Result<ParamASTNode> {
        // FunctionParam + FunctionParam' rules
        let mutability = self.parse_mut()?;
        let token = self.consume()?;
        let ident_span = token.span();
        let ident = assert_ident_or_underscore!(self, token);

        // FunctionParam'' rule
        assert_token!(self, Colon, "':'");
        let ty = self.parse_type()?;

        let assignee: Box<dyn ExprASTNode> = match ident {
            None => Box::new(UnderscoreASTNode::new(ident_span)),
            Some(ident) => Box::new(PathASTNode::new(ident, ident_span)),
        };
        Ok(ParamASTNode::new(assignee, ty, mutability, ident_span))
    }

    fn parse_func_ret_ty(&mut self) -> Result<Option<TypeASTMetaNode>> {
        let next = self.peek()?;
        match next.ty() {
            Semi | LBra => Ok(None),
            Arrow => {
                self.consume().expect("Arrow token should be present.");
                self.parse_type().map(Some)
            }
            _ => unknown_token!(self, "'->', ';', '{'"),
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

    fn parse_item_assignment(&mut self) -> Result<Option<Box<dyn ExprASTNode>>> {
        let next = self.peek()?;
        match next.ty() {
            Assign => {
                self.consume().expect("Assign token should be present.");
                let expr = self.parse_expr()?;
                Ok(Some(expr))
            }
            Semi => Ok(None),
            _ => unknown_token!(self, "'=', ';'"),
        }
    }

    fn parse_static(&mut self, is_extern: bool) -> Result<StaticASTNode> {
        let start_pos = assert_token!(self, Static, "'static'").start();

        let mutability = self.parse_mut()?;
        let ident = assert_ident!(self, "'_', 'mut', <ident>");

        assert_token!(self, Colon, "':'");
        let ty = self.parse_type()?;
        let value = self.parse_item_assignment()?;

        let end_pos = assert_token!(self, Semi, "';'").end();
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
        let start_pos = assert_token!(self, Extern, "'extern'").start();
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
            _ => return unknown_token!(self, next, "<ABI>"),
        };

        assert_token!(self, LBra, "'{'");
        let items = self.parse_extern_items()?;
        let end_pos = assert_token!(self, RBra, "'}'").end();

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
                _ => return unknown_token!(self, "<item>"),
            }
        }
    }

    fn parse_extern_func(&mut self) -> Result<FuncProtoASTNode> {
        let proto = self.parse_func_proto()?;

        let next = self.peek()?;
        match next.ty() {
            Semi => {
                assert_token!(self, Semi, "';'");
            }
            LBra => {
                let body_span = self.parse_block_expr()?.span();
                self.push_rcv_error(
                    SemanticError::ExternFunctionWithBody { span: body_span }.into(),
                );
            }
            _ => return unknown_token!(self, "';', '{'"),
        }

        Ok(proto)
    }

    fn parse_let_stmt(&mut self) -> Result<LetASTNode> {
        let start_pos = assert_token!(self, Let, "'let'").start();

        //TODO Add support for destructuring
        let mutability = self.parse_mut()?;
        let ident = assert_ident!(self, "<pattern>");

        //TODO Add support for type inference
        assert_token!(self, Colon, "':'");
        let ty = self.parse_type()?;
        let val = self.parse_item_assignment()?;

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

        let expr_stmt = ExprStmtASTNode::new(expr, span);
        Ok((expr_stmt, semi.is_none()))
    }

    fn parse_expr(&mut self) -> Result<Box<dyn ExprASTNode>> {
        self.parse_expr_wo_block()
    }

    fn parse_expr_wo_block(&mut self) -> Result<Box<dyn ExprASTNode>> {
        let next = self.peek()?;
        match next.ty() {
            Minus | Not | IntLit(_) | FloatLit(_) | BoolLit(_) | LPar | Underscore | LBra | If
            | Unsafe | Ident(_) | Loop | While => self.parse_operator_expr(),
            Return => {
                let return_expr = self.parse_return()?;
                Ok(Box::new(return_expr))
            }
            _ => unknown_token!(self, "<expr>"),
        }
    }

    // ExpressionWithoutBlock' rule
    fn parse_expr_wo_block_(&mut self) -> Result<Box<dyn ExprASTNode>> {
        let next = self.peek()?;
        match next.ty() {
            IntLit(_) | FloatLit(_) | BoolLit(_) => self.parse_literal_expr(),
            Ident(_) => self.parse_path_or_call_expr(),
            LPar => {
                let expr = self.parse_grouped_expr_or_unit_lit()?;
                Ok(match expr {
                    Either::Left(expr) => Box::new(expr),
                    Either::Right(lit) => Box::new(lit),
                })
            }
            Underscore => {
                let expr = self.parse_underscore_expr()?;
                Ok(Box::new(expr))
            }
            _ => unknown_token!(self, "<expr>"),
        }
    }

    fn parse_expr_w_block(&mut self) -> Result<Box<dyn ExprASTNode>> {
        let next = self.peek()?;
        Ok(match next.ty() {
            LBra => Box::new(self.parse_block_expr()?),
            Loop | While => self.parse_loop_expr()?,
            If => Box::new(self.parse_if_expr()?),
            Unsafe => Box::new(self.parse_unsafe_expr()?),
            _ => return unknown_token!(self, "<expr>"),
        })
    }

    fn parse_literal_expr(&mut self) -> Result<Box<dyn ExprASTNode>> {
        /// A macro to create a boxed literal node.
        macro_rules! box_literal {
            ($ty:ty, $val:expr, $span:expr) => {{
                let literal = LiteralASTNode::<$ty>::new($val, $span);
                Ok(Box::new(literal))
            }};
        }

        let token = self.consume()?;
        match token.ty() {
            //TODO Add support for different sizes of ints and floats
            IntLit(val) => box_literal!(i32, *val, token.span()),
            FloatLit(val) => box_literal!(f64, *val, token.span()),
            BoolLit(val) => box_literal!(bool, *val, token.span()),
            _ => unknown_token!(self, token, "<literal>"),
        }
    }

    // Ident' rule
    fn parse_path_or_call_expr(&mut self) -> Result<Box<dyn ExprASTNode>> {
        let path = Box::new(self.parse_path_expr()?);
        let next = self.peek()?;
        Ok(match next.ty() {
            LPar => {
                // CallExpression' rule
                assert_token!(self, LPar, "'('");

                let params = self.parse_call_params()?;

                let end_pos = assert_token!(self, RPar, "')'").end();
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
            _ => unknown_token!(self, token, "<path>"),
        }
    }

    fn parse_block_expr(&mut self) -> Result<BlockASTNode> {
        let start_pos = assert_token!(self, LBra, "'{'").start();

        let (stmts, return_expr) = self.parse_stmts()?;

        let end_pos = assert_token!(self, RBra, "'}'").end();
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
                    assert_token!(self, Semi, "';'");
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

    fn parse_operator_expr(&mut self) -> Result<Box<dyn ExprASTNode>> {
        ops::parse_ops(self)
    }

    fn parse_grouped_expr_or_unit_lit(
        &mut self,
    ) -> Result<Either<GroupedExprASTNode, LiteralASTNode<()>>> {
        let start_pos = assert_token!(self, LPar, "'('").start();

        // GroupedOrUnit rule
        let next = self.peek()?;
        Ok(match next.ty() {
            RPar => {
                let end_pos = assert_token!(self, RPar, "')'").end();
                let span = Span::new(start_pos, end_pos);
                Either::Right(LiteralASTNode::<()>::new(span))
            }
            _ => {
                let expr = self.parse_expr()?;
                let end_pos = assert_token!(self, RPar, "')'").end();
                let span = Span::new(start_pos, end_pos);

                let expr = GroupedExprASTNode::new(expr, span);
                Either::Left(expr)
            }
        })
    }

    fn parse_call_params(&mut self) -> Result<Vec<Box<dyn ExprASTNode>>> {
        let mut result = Vec::new();
        loop {
            // CallParams rule
            let next = self.peek()?;
            match next.ty() {
                Return | Minus | Not | IntLit(_) | FloatLit(_) | BoolLit(_) | LPar | Underscore
                | LBra | If | Unsafe | Ident(_) | Loop | While => {
                    let expr = self.parse_expr()?;
                    result.push(expr);
                }
                RPar => return Ok(result),
                _ => return unknown_token!(self, "<expr>"),
            }

            // CallParams' rule
            let next = self.peek()?;
            match next.ty() {
                Comma => assert_token!(self, Comma, "','"),
                RPar => return Ok(result),
                _ => return unknown_token!(self, "',', ')'"),
            };
        }
    }

    fn parse_loop_expr(&mut self) -> Result<Box<dyn ExprASTNode>> {
        let next = self.peek()?;
        Ok(match next.ty() {
            Loop => Box::new(self.parse_inf_loop_expr()?),
            While => Box::new(self.parse_pred_loop_expr()?),
            _ => return unknown_token!(self, "<loop expr>"),
        })
    }

    fn parse_inf_loop_expr(&mut self) -> Result<InfLoopASTNode> {
        let start_pos = assert_token!(self, Loop, "'loop'").start();
        let body = self.parse_block_expr()?;
        let end_pos = body.span().end();
        let span = Span::new(start_pos, end_pos);

        let loop_expr = InfLoopASTNode::new(Box::new(body), span);
        Ok(loop_expr)
    }

    fn parse_pred_loop_expr(&mut self) -> Result<WhileASTNode> {
        let start_pos = assert_token!(self, While, "'while'").start();

        let condition = self.parse_expr()?;
        let body = self.parse_block_expr()?;

        let end_pos = body.span().end();
        let span = Span::new(start_pos, end_pos);

        let while_expr = WhileASTNode::new(condition, Box::new(body), span);
        Ok(while_expr)
    }

    fn parse_if_expr(&mut self) -> Result<IfASTNode> {
        let start_pos = assert_token!(self, If, "'if'").start();

        let condition = self.parse_expr()?;
        let then_block = self.parse_block_expr()?;
        let else_expr = self.parse_else_expr()?;

        let end_pos = match &else_expr {
            ElseExpr::None => then_block.span().end(),
            ElseExpr::Else(block) => block.span().end(),
            ElseExpr::ElseIf(if_node) => if_node.span().end(),
        };
        let span = Span::new(start_pos, end_pos);

        Ok(IfASTNode::new(
            condition,
            Box::new(then_block),
            else_expr,
            span,
        ))
    }

    // IfExpressionTail' & ElseExpression' rules
    fn parse_else_expr(&mut self) -> Result<ElseExpr> {
        // IfExpressionTail' rule
        let next = self.peek()?;
        Ok(match next.ty() {
            Else => {
                assert_token!(self, Else, "'else'");

                // ElseExpression' rule
                let next = self.peek()?;
                match next.ty() {
                    If => {
                        let if_node = self.parse_if_expr()?;
                        ElseExpr::ElseIf(Box::new(if_node))
                    }
                    LBra => {
                        let block = self.parse_block_expr()?;
                        ElseExpr::Else(Box::new(block))
                    }
                    _ => return unknown_token!(self, "'if', '{'"),
                }
            }
            RPar | Comma | LBra | As | Asterisk | Div | Mod | Plus | Minus | BitAnd | BitXor
            | BitOr | Eq | Ne | Lt | Gt | Le | Ge | And | Or | Assign | RBra | Semi => {
                ElseExpr::None
            }
            _ => {
                return unknown_token!(
                    self,
                    "'else', ')', ',', '{', 'as', <operator>, '=', '}', ';'"
                )
            }
        })
    }

    fn parse_unsafe_expr(&mut self) -> Result<UnsafeBlockASTNode> {
        let start_pos = assert_token!(self, Unsafe, "'unsafe'").start();
        let block = self.parse_block_expr()?;
        let end_pos = block.span().end();
        let span = Span::new(start_pos, end_pos);

        let unsafe_block = UnsafeBlockASTNode::new(Box::new(block), span);
        Ok(unsafe_block)
    }

    fn parse_return(&mut self) -> Result<ReturnASTNode> {
        let span = assert_token!(self, Return, "'return'");

        // ReturnExpressionTail' rule
        let next = self.peek()?;
        Ok(match next.ty() {
            Return | Minus | Not | IntLit(_) | FloatLit(_) | BoolLit(_) | LPar | Underscore
            | LBra | If | Unsafe | Ident(_) | Loop | While => {
                let expr = self.parse_expr()?;
                let span = Span::new(span.start(), expr.span().end());

                ReturnASTNode::new(expr, span)
            }
            RPar | Comma | RBra | Semi => ReturnASTNode::empty(span),
            _ => return unknown_token!(self, "<expr>, ';'"),
        })
    }

    fn parse_underscore_expr(&mut self) -> Result<UnderscoreASTNode> {
        let span = assert_token!(self, Underscore, "'_'");
        Ok(UnderscoreASTNode::new(span))
    }

    fn parse_type(&mut self) -> Result<TypeASTMetaNode> {
        let token = self.consume()?;
        match token.ty() {
            Ident(ident) => match ident.parse::<Type>() {
                Ok(ty) => Ok(TypeASTMetaNode::new(ty, token.span())),
                Err(_) => unknown_token!(self, token, "<type>"),
            },
            LPar => {
                //TODO Add support for tuples
                let end_pos = assert_token!(self, RPar, "')'").end();
                let span = Span::new(token.span().start(), end_pos);
                Ok(TypeASTMetaNode::new(Type::Unit, span))
            }
            _ => {
                //TODO Add support for other symbol-based types (e.g. references, slices, etc.)
                unknown_token!(self, token, "<type>")
            }
        }
    }
}
