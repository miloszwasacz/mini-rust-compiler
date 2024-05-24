//! A module containing the Function Parameter AST node implementation.

use std::{fmt, iter};

use crate::ast::{ast_defaults, ASTChildIterator, ASTNode, ExprASTNode, TypeASTMetaNode};
use crate::token::Span;

/// An AST node representing a function parameter.
#[derive(Debug)]
pub struct ParamASTNode {
    /// The assignee has to be an [assignee expression](crate::ast::AssigneeExprASTNode).
    assignee: Box<dyn ExprASTNode>,
    ty: TypeASTMetaNode,
    mutable: bool,
    span: Span,
}

impl ParamASTNode {
    /// Creates a new `ParamASTNode` with the given assignee, type, mutability and span.
    pub fn new(
        assignee: Box<dyn ExprASTNode>,
        ty: TypeASTMetaNode,
        mutable: bool,
        span: Span,
    ) -> ParamASTNode {
        ParamASTNode {
            assignee,
            ty,
            mutable,
            span,
        }
    }

    /// Returns the type.
    pub fn ty(&self) -> TypeASTMetaNode {
        self.ty
    }

    /// Returns whether the parameter is mutable.
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
}

impl ASTNode for ParamASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = iter::once(self.assignee.as_ast());
        Some(Box::new(iter))
    }
}

impl fmt::Display for ParamASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mutability = if self.is_mutable() { "Mut" } else { "" };
        write!(f, "Param{} {}", mutability, self.span)
    }
}