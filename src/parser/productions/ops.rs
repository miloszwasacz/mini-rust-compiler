//! A module containing production rules for operators.
//!
//! The implementation takes into account the precedence of operators.
//! For the detailed operator precedence, see the transformed grammar file
//! or the [Rust reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html).

use super::*;

/// A module with functions to construct binary operators.
mod ctr {
    use crate::ast::{
        ArithExprASTNode, ArithOperator, AssignASTNode, CompExprASTNode, CompOperator, ExprASTNode,
        LazyBoolExprASTNode, LazyBoolOperator,
    };
    use crate::token::Span;

    /// A macro to generate constructors for binary operators.
    macro_rules! ctr {
        (
            $name:ident,
            $ty:ident
        ) => {
            pub fn $name(lhs: Box<dyn ExprASTNode>, rhs: Box<dyn ExprASTNode>, span: Span) -> $ty {
                $ty::new(lhs, rhs, span)
            }
        };
        (
            $name:ident,
            $ty:ident,
            $param:expr
        ) => {
            pub fn $name(lhs: Box<dyn ExprASTNode>, rhs: Box<dyn ExprASTNode>, span: Span) -> $ty {
                $ty::new($param, lhs, rhs, span)
            }
        };
    }

    ctr!(assign, AssignASTNode);
    ctr!(lazy_or, LazyBoolExprASTNode, LazyBoolOperator::Or);
    ctr!(lazy_and, LazyBoolExprASTNode, LazyBoolOperator::And);
    ctr!(eq, CompExprASTNode, CompOperator::Eq);
    ctr!(ne, CompExprASTNode, CompOperator::Ne);
    ctr!(gt, CompExprASTNode, CompOperator::Gt);
    ctr!(lt, CompExprASTNode, CompOperator::Lt);
    ctr!(ge, CompExprASTNode, CompOperator::Ge);
    ctr!(le, CompExprASTNode, CompOperator::Le);
    ctr!(bit_or, ArithExprASTNode, ArithOperator::BitOr);
    ctr!(bit_xor, ArithExprASTNode, ArithOperator::BitXor);
    ctr!(bit_and, ArithExprASTNode, ArithOperator::BitAnd);
    ctr!(add, ArithExprASTNode, ArithOperator::Add);
    ctr!(sub, ArithExprASTNode, ArithOperator::Sub);
    ctr!(mul, ArithExprASTNode, ArithOperator::Mul);
    ctr!(div, ArithExprASTNode, ArithOperator::Div);
    ctr!(rem, ArithExprASTNode, ArithOperator::Rem);
}

/// A macro to generate functions to parse binary operators.
macro_rules! parse_op {
    (
        $name:ident,
        $next:ident,
        [ $( ( $op:pat, $ctr:path), )+ ],
        $empty:pat,
    ) => {
        mod $name {
            use super::*;

            pub(super) fn parse(parser: &mut Parser) -> Result<Box<dyn ParserExpr>> {
                let lhs = $next::parse(parser)?;
                parse_tail(parser, lhs)
            }

            fn parse_tail(parser: &mut Parser, lhs: Box<dyn ParserExpr>) -> Result<Box<dyn ParserExpr>> {
                let next = parser.peek()?;
                match next.ty() {
                    $(
                        $op => {
                            let lhs = lhs.into_expr();
                            assert_token!(parser, $op);
                            let rhs = $next::parse(parser)?.into_expr();
                            let span = Span::new(lhs.span().start(), rhs.span().end());

                            let lhs = Box::new($ctr(lhs, rhs, span));
                            parse_tail(parser, lhs)
                        },
                    )+
                    $empty => Ok(lhs),
                    _ => unknown_token!(parser),
                }
            }
        }
    };

    // Helper macro pattern for prettier formatting.
    (
        $name:ident,
        $next:ident,
        [
            ( $first_op:pat, $first_ctr:path )
            $( ,( $op:pat, $ctr:path) )*
        ],
        $empty:pat,
    ) => {
        parse_op!(
            $name,
            $next,
            [
                ( $first_op, $first_ctr )
                $(, ( $op, $ctr) )*,
            ],
            $empty,
        );
    };
}

/// The main function to parse operators.
pub fn parse_ops(parser: &mut Parser) -> Result<Box<dyn ParserExpr>> {
    op1::parse(parser)
}

// `Expr1` and `Expr1'`
parse_op!(
    op1,
    op2,
    [(Assign, ctr::assign)],
    LBra | LPar | Comma | RBra | Semi,
);

// `Expr2` and `Expr2'`
parse_op!(
    op2,
    op3,
    [(Or, ctr::lazy_or)],
    LBra | LPar | Comma | RBra | Semi | Assign,
);

// `Expr3` and `Expr3'`
parse_op!(
    op3,
    op4,
    [(And, ctr::lazy_and)],
    LBra | LPar | Comma | RBra | Semi | Assign | Or,
);

// `Expr4` and `Expr4'`
parse_op!(
    op4,
    op5,
    [
        (Eq, ctr::eq),
        (Ne, ctr::ne),
        (Gt, ctr::gt),
        (Lt, ctr::lt),
        (Ge, ctr::ge),
        (Le, ctr::le)
    ],
    LBra | LPar | Comma | RBra | Semi | Assign | Or | And,
);

// `Expr5` and `Expr5'`
parse_op!(
    op5,
    op6,
    [(BitOr, ctr::bit_or)],
    LBra | LPar | Comma | RBra | Semi | Assign | Or | And | Eq | Ne | Gt | Lt | Ge | Le,
);

// `Expr6` and `Expr6'`
parse_op!(
    op6,
    op7,
    [(BitXor, ctr::bit_xor)],
    LBra | LPar | Comma | RBra | Semi | Assign | Or | And | Eq | Ne | Gt | Lt | Ge | Le | BitOr,
);

// `Expr7` and `Expr7'`
parse_op!(
    op7,
    op8,
    [(BitAnd, ctr::bit_and)],
    LBra | LPar
        | Comma
        | RBra
        | Semi
        | Assign
        | Or
        | And
        | Eq
        | Ne
        | Gt
        | Lt
        | Ge
        | Le
        | BitOr
        | BitXor,
);

// `Expr8` and `Expr8'`
parse_op!(
    op8,
    op9,
    [(Plus, ctr::add), (Minus, ctr::sub)],
    LBra | LPar
        | Comma
        | RBra
        | Semi
        | Assign
        | Or
        | And
        | Eq
        | Ne
        | Gt
        | Lt
        | Ge
        | Le
        | BitOr
        | BitXor
        | BitAnd,
);

// `Expr9` and `Expr9'`
parse_op!(
    op9,
    op10,
    [(Asterisk, ctr::mul), (Div, ctr::div), (Mod, ctr::rem)],
    LBra | LPar
        | Comma
        | RBra
        | Semi
        | Assign
        | Or
        | And
        | Eq
        | Ne
        | Gt
        | Lt
        | Ge
        | Le
        | BitOr
        | BitXor
        | BitAnd
        | Plus
        | Minus,
);

// `Expr10` and `Expr10'`
mod op10 {
    use crate::ast::TypeCastASTNode;

    use super::*;

    // `Expr10`
    pub fn parse(parser: &mut Parser) -> Result<Box<dyn ParserExpr>> {
        let lhs = op11::parse(parser)?;
        parse_tail(parser, lhs)
    }

    // `Expr10'`
    fn parse_tail(parser: &mut Parser, lhs: Box<dyn ParserExpr>) -> Result<Box<dyn ParserExpr>> {
        let next = parser.peek()?;
        match next.ty() {
            As => {
                let lhs = lhs.into_expr();
                assert_token!(parser, As);
                let ty = Parser::parse_type(parser)?;
                let span = Span::new(lhs.span().start(), ty.span().end());

                let lhs = Box::new(TypeCastASTNode::new(lhs, ty, span));
                parse_tail(parser, lhs)
            }
            LBra | LPar | Comma | RBra | Semi | Assign | Or | And | Eq | Ne | Gt | Lt | Ge | Le
            | BitOr | BitXor | BitAnd | Plus | Minus | Asterisk | Div | Mod => Ok(lhs),
            _ => unknown_token!(parser),
        }
    }
}

// `Expr11` and `NegationExpression`
mod op11 {
    use crate::ast::{NegExprASTNode, NegOperator};

    use super::*;

    // `Expr11`
    pub fn parse(parser: &mut Parser) -> Result<Box<dyn ParserExpr>> {
        let next = parser.peek()?;
        match next.ty() {
            Minus => {
                let start_pos = assert_token!(parser, Minus).start();
                parse_negation(parser, NegOperator::Neg, start_pos)
            }
            Not => {
                let start_pos = assert_token!(parser, Not).start();
                parse_negation(parser, NegOperator::Not, start_pos)
            }
            IntLit(_) | FloatLit(_) | BoolLit(_) | LPar | Underscore | Ident(_) => {
                Parser::parse_expr_wo_block_(parser)
            }
            LBra | If | Unsafe | Loop | While => Parser::parse_expr_w_block(parser),
            _ => unknown_token!(parser),
        }
    }

    // `NegationExpression`
    fn parse_negation(
        parser: &mut Parser,
        op: NegOperator,
        start_pos: Position,
    ) -> Result<Box<dyn ParserExpr>> {
        let expr = parse(parser)?;
        let span = Span::new(start_pos, expr.span().end());

        let expr = NegExprASTNode::new(op, expr.into_expr(), span);
        Ok(Box::new(expr))
    }
}
