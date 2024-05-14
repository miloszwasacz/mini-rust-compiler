//! A module containing Assignment AST node implementation.

use std::{fmt, iter};

use crate::ast::error::SemanticError;
use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, AssigneeExprASTNode, ExprASTNode,
    ExpressionBox, ExpressionBoxKind, ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing an assignment.
#[derive(Debug)]
pub struct AssignASTNode {
    /// The assignee has to be an [assignee expression](ExpressionBox::Assignee).
    assignee: Box<ExpressionBox>,
    /// The value has to be a [value expression](ExpressionBox::Value).
    value: Box<ExpressionBox>,
    span: Span,
}

impl AssignASTNode {
    /// Creates a new `AssignASTNode` with the given assignee, value and span.
    pub fn new(
        assignee: Box<ExpressionBox>,
        value: Box<ExpressionBox>,
        span: Span,
    ) -> AssignASTNode {
        AssignASTNode {
            assignee,
            value,
            span,
        }
    }

    /// Returns the assignee.
    ///
    /// # Errors
    /// Returns a [`SemanticError::WrongExpressionKind`] if the assignee
    /// is not an [assignee expression](ExpressionBox::Assignee).
    pub fn assignee(&self) -> Result<&dyn AssigneeExprASTNode, SemanticError> {
        match self.assignee.as_ref() {
            ExpressionBox::Assignee(expr) => Ok(expr.as_ref()),
            _ => Err(SemanticError::WrongExpressionKind {
                expected: ExpressionBoxKind::Assignee,
                actual: self.assignee.kind(),
                span: self.assignee.span(),
            }),
        }
    }

    /// Returns the value.
    ///
    /// # Errors
    /// Returns a [`SemanticError::WrongExpressionKind`] if the value
    /// is not a [value expression](ExpressionBox::Value).
    pub fn value(&self) -> Result<&dyn ValueExprASTNode, SemanticError> {
        match self.value.as_ref() {
            ExpressionBox::Value(expr) => Ok(expr.as_ref()),
            _ => Err(SemanticError::WrongExpressionKind {
                expected: ExpressionBoxKind::Value,
                actual: self.value.kind(),
                span: self.value.span(),
            }),
        }
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
