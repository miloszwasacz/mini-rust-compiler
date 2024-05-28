//! A module containing implementations for distinguishing between
//! expressions with and without blocks.

use crate::ast::{
    ArithExprASTNode, AssignASTNode, BlockASTNode, CompExprASTNode, ExprASTNode, FunCallASTNode,
    GroupedExprASTNode, IfASTNode, InfLoopASTNode, LazyBoolExprASTNode, LiteralASTNode, LiteralBox,
    NegExprASTNode, PathASTNode, ReturnASTNode, TypeCastASTNode, UnderscoreASTNode,
    UnsafeBlockASTNode, WhileASTNode,
};

use self::ExpressionKind::*;

/// Implements the [`ParserExpr`] trait for the given type such that
/// it returns the given [`ExpressionKind`].
macro_rules! parser_expr_impl {
    ($ast_ty:ty, $kind:ident) => {
        impl ParserExpr for $ast_ty {
            fn into_expr(self: Box<Self>) -> Box<dyn ExprASTNode> {
                self
            }

            fn kind(&self) -> ExpressionKind {
                $kind
            }
        }
    };
}

/// A classification of expressions based on whether they have a block as a part of them or not.
///
/// It is only used during parsing to determine whether to expect a semicolon
/// after an expression or not.
pub(super) enum ExpressionKind {
    /// An expression with a block as a part of it.
    WithBlock,
    /// An expression without a block.
    WithoutBlock,
}

/// A trait that provides a way to determine the [`ExpressionKind`].
pub(super) trait ParserExpr: ExprASTNode {
    /// Casts the parser expression into its super-trait.
    fn into_expr(self: Box<Self>) -> Box<dyn ExprASTNode>;

    /// Returns the [kind](ExpressionKind) of the expression.
    fn kind(&self) -> ExpressionKind;
}

//#region WithBlock

parser_expr_impl!(BlockASTNode, WithBlock);
parser_expr_impl!(InfLoopASTNode, WithBlock);
parser_expr_impl!(WhileASTNode, WithBlock);
parser_expr_impl!(IfASTNode, WithBlock);
parser_expr_impl!(UnsafeBlockASTNode, WithBlock);

//#endregion

//#region WithoutBlock

parser_expr_impl!(LiteralASTNode<i32>, WithoutBlock);
parser_expr_impl!(LiteralASTNode<f64>, WithoutBlock);
parser_expr_impl!(LiteralASTNode<bool>, WithoutBlock);
parser_expr_impl!(LiteralASTNode<()>, WithoutBlock);
impl LiteralBox {
    /// Converts the literal box into a boxed parser expression.
    pub(super) fn into_parser_expr(self) -> Box<dyn ParserExpr> {
        match self {
            LiteralBox::I32(lit) => lit,
            LiteralBox::F64(lit) => lit,
            LiteralBox::Bool(lit) => lit,
            LiteralBox::Unit(lit) => lit,
        }
    }
}
parser_expr_impl!(PathASTNode, WithoutBlock);
parser_expr_impl!(NegExprASTNode, WithoutBlock);
parser_expr_impl!(ArithExprASTNode, WithoutBlock);
parser_expr_impl!(CompExprASTNode, WithoutBlock);
parser_expr_impl!(LazyBoolExprASTNode, WithoutBlock);
parser_expr_impl!(TypeCastASTNode, WithoutBlock);
parser_expr_impl!(AssignASTNode, WithoutBlock);
parser_expr_impl!(FunCallASTNode, WithoutBlock);
parser_expr_impl!(GroupedExprASTNode, WithoutBlock);
parser_expr_impl!(ReturnASTNode, WithoutBlock);
parser_expr_impl!(UnderscoreASTNode, WithoutBlock);

//#endregion
