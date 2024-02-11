//! A module containing Assignment AST node implementation.

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
}

impl ASTNode for AssignASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let assignee = self.assignee.as_ast();
        let value = self.value.as_ast();
        Some(Box::new(vec![assignee, value].into_iter()))
    }
}

impl ExprASTNode for AssignASTNode {}

impl std::fmt::Display for AssignASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assign {}", self.span)
    }
}
