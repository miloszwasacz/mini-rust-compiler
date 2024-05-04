//! A module containing the Let Statement AST node implementation.

use std::fmt;

use crate::ast::{
    as_ast, ast_defaults, ASTChildIterator, ASTNode, AssignASTNode, AssigneeExprASTNode,
    ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing a let statement.
#[derive(Debug)]
pub struct LetASTNode {
    assignment: Box<AssignASTNode>,
    mutable: bool,
    span: Span,
}

impl LetASTNode {
    /// Creates a new `LetASTNode` with the given assignment, mutability and span.
    pub fn new(assignment: Box<AssignASTNode>, mutable: bool, span: Span) -> LetASTNode {
        LetASTNode {
            assignment,
            mutable,
            span,
        }
    }

    /// Returns the assignee.
    pub fn assignee(&self) -> &dyn AssigneeExprASTNode {
        self.assignment.assignee()
    }

    /// Returns the value.
    pub fn value(&self) -> &dyn ValueExprASTNode {
        self.assignment.value()
    }

    /// Returns whether the variable is mutable.
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
}

impl ASTNode for LetASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        self.assignment.children()
    }
}

impl fmt::Display for LetASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mutability = if self.mutable { "Mut" } else { "" };
        write!(f, "Let{} {}", mutability, self.span)
    }
}
