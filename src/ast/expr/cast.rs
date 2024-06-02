//! A module containing Type Cast AST node implementation.

use std::{fmt, iter};

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode,
    TypeASTMetaNode, ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing a type cast.
#[derive(Debug)]
pub struct TypeCastASTNode {
    /// The value can be [any kind of expression](ExprASTNode).
    value: Box<dyn ExprASTNode>,
    ty: TypeASTMetaNode,
    span: Span,
}

impl TypeCastASTNode {
    /// Creates a new `TypeCastASTNode` with the given value, type and span.
    pub fn new(value: Box<dyn ExprASTNode>, ty: TypeASTMetaNode, span: Span) -> TypeCastASTNode {
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

impl ExprASTNode for TypeCastASTNode {
    fn try_as_place(&self) -> Option<&dyn PlaceExprASTNode> {
        None
    }

    fn try_as_value(&self) -> Option<&dyn ValueExprASTNode> {
        Some(self)
    }

    fn try_as_assignee(&self) -> Option<&dyn AssigneeExprASTNode> {
        None
    }
}

impl ValueExprASTNode for TypeCastASTNode {}

impl fmt::Display for TypeCastASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type Cast {} `{}`", self.span, self.ty)
    }
}
