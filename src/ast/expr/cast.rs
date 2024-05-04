//! A module containing Type Cast AST node implementation.

use std::{fmt, iter};

use crate::ast::{
    as_ast, ast_defaults, ASTChildIterator, ASTNode, ExprASTNode, ValueExprASTNode, VarType,
};
use crate::token::Span;

/// An AST node representing a type cast.
#[derive(Debug)]
pub struct TypeCastASTNode {
    value: Box<dyn ExprASTNode>,
    ty: VarType,
    span: Span,
}

impl TypeCastASTNode {
    /// Creates a new `TypeCastASTNode` with the given value, type and span.
    pub fn new(value: Box<dyn ExprASTNode>, ty: VarType, span: Span) -> TypeCastASTNode {
        TypeCastASTNode { value, ty, span }
    }
}

impl ASTNode for TypeCastASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = iter::once(self.value.as_ast());
        Some(Box::new(iter))
    }
}

impl ExprASTNode for TypeCastASTNode {}

impl ValueExprASTNode for TypeCastASTNode {}

impl fmt::Display for TypeCastASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type Cast {} {}", self.span, self.ty)
    }
}
