//! A module containing Assignment AST node implementation.

use std::{fmt, iter};

use crate::ast::{
    as_ast, ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode,
    ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing an assignment.
#[derive(Debug)]
pub struct AssignASTNode {
    assignee: Box<dyn AssigneeExprASTNode>,
    value: Box<dyn ValueExprASTNode>,
    span: Span,
}

impl AssignASTNode {
    /// Creates a new `AssignASTNode` with the given assignee, value and span.
    pub fn new(
        assignee: Box<dyn AssigneeExprASTNode>,
        value: Box<dyn ValueExprASTNode>,
        span: Span,
    ) -> AssignASTNode {
        AssignASTNode {
            assignee,
            value,
            span,
        }
    }

    /// Returns the assignee.
    pub fn assignee(&self) -> &dyn AssigneeExprASTNode {
        self.assignee.as_ref()
    }

    /// Returns the value.
    pub fn value(&self) -> &dyn ValueExprASTNode {
        self.value.as_ref()
    }
}

impl ASTNode for AssignASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let assignee = iter::once(self.assignee.as_ast());
        let value = iter::once(self.value.as_ast());
        let iter = assignee.chain(value);
        Some(Box::new(iter))
    }
}

impl ExprASTNode for AssignASTNode {}

impl ValueExprASTNode for AssignASTNode {}

impl fmt::Display for AssignASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assignment {}", self.span)
    }
}
