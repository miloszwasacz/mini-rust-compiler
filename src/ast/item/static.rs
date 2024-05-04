//! A module containing the Static Item AST node implementation.

use std::fmt;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, AssignASTNode, VarType};
use crate::token::Span;

/// An AST node representing a static item.
#[derive(Debug)]
pub struct StaticASTNode {
    item: Box<AssignASTNode>,
    ty: VarType,
    mutable: bool,
    span: Span,
}

impl StaticASTNode {
    /// Creates a new `StaticASTNode` with the given item, type, mutability and span.
    pub fn new(item: Box<AssignASTNode>, ty: VarType, mutable: bool, span: Span) -> StaticASTNode {
        StaticASTNode {
            item,
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

impl ASTNode for StaticASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        self.item.children()
    }
}

impl fmt::Display for StaticASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mutability = if self.mutable { "Mut" } else { "" };
        write!(f, "Static{} {}", mutability, self.span)
    }
}
