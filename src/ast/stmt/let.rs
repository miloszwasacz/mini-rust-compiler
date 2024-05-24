//! A module containing the Let Statement AST node implementation.

use std::{fmt, iter};

use crate::ast::{ast_defaults, ASTChildIterator, ASTNode, ExprASTNode, Type, TypeASTMetaNode};
use crate::token::Span;

/// An AST node representing a let statement.
#[derive(Debug)]
pub struct LetASTNode {
    /// The declaration has to be an [assignee expression](crate::ast::AssigneeExprASTNode).
    decl: Box<dyn ExprASTNode>,
    ty: TypeASTMetaNode,
    /// The value has to be a [value expression](crate::ast::ValueExprASTNode).
    value: Option<Box<dyn ExprASTNode>>,
    mutable: bool,
    span: Span,
}

impl LetASTNode {
    /// Creates a new `LetASTNode` with the given declaration, type, mutability and span.
    pub fn new(
        decl: Box<dyn ExprASTNode>,
        ty: TypeASTMetaNode,
        mutable: bool,
        span: Span,
    ) -> LetASTNode {
        LetASTNode {
            decl,
            ty,
            value: None,
            mutable,
            span,
        }
    }

    /// Creates a new `LetASTNode` with the given declaration, type, assigned value, mutability and span.
    pub fn new_with_assignment(
        decl: Box<dyn ExprASTNode>,
        ty: TypeASTMetaNode,
        value: Box<dyn ExprASTNode>,
        mutable: bool,
        span: Span,
    ) -> LetASTNode {
        LetASTNode {
            decl,
            ty,
            value: Some(value),
            mutable,
            span,
        }
    }

    /// Returns the type of the declaration
    pub fn ty(&self) -> Type {
        self.ty.ty()
    }

    /// Returns whether the variable is mutable.
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
}

impl ASTNode for LetASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let decl = iter::once(self.decl.as_ast());
        let value = self.value.iter().map(|v| v.as_ast());
        let iter = decl.chain(value);
        Some(Box::new(iter))
    }
}

impl fmt::Display for LetASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mutability = if self.mutable { "Mut" } else { "" };
        write!(f, "Let{} {}", mutability, self.span)
    }
}
