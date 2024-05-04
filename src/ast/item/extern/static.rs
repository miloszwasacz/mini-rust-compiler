//! A module containing the Static Item AST node implementation specific to the extern block.

use std::fmt;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, VarType};
use crate::token::Span;

/// An AST node representing a static item.
#[derive(Debug)]
pub struct ExternStaticASTNode {
    assignee: Box<dyn AssigneeExprASTNode>,
    ty: VarType,
    mutable: bool,
    span: Span,
}

impl ExternStaticASTNode {
    /// Creates a new `StaticASTNode` with the given assignee, type, mutability and span.
    pub fn new(
        assignee: Box<dyn AssigneeExprASTNode>,
        ty: VarType,
        mutable: bool,
        span: Span,
    ) -> ExternStaticASTNode {
        ExternStaticASTNode {
            assignee,
            ty,
            mutable,
            span,
        }
    }

    /// Returns the type.
    pub fn ty(&self) -> VarType {
        self.ty
    }

    /// Returns whether the item is mutable.
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
}

impl ASTNode for ExternStaticASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        self.assignee.children()
    }
}

impl fmt::Display for ExternStaticASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mutability = if self.mutable { "Mut" } else { "" };
        write!(f, "Static{} {}", mutability, self.span)
    }
}
