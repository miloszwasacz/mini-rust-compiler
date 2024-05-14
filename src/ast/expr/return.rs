//! A module containing Return AST node implementation.

use std::{fmt, iter};

use crate::ast::{ast_defaults, ASTNode, AsASTNode, ExprASTNode, ExpressionBox, ValueExprASTNode};
use crate::token::Span;

/// An AST node representing a return expression.
#[derive(Debug)]
pub struct ReturnASTNode {
    /// The return value can be [any expression](ExpressionBox::Unspecified).
    value: ExpressionBox,
    span: Span,
}

impl ReturnASTNode {
    /// Creates a new `ReturnASTNode` with the given return value and span.
    pub fn new(value: ExpressionBox, span: Span) -> ReturnASTNode {
        ReturnASTNode { value, span }
    }
}

impl ASTNode for ReturnASTNode {
    ast_defaults!();

    fn children(&self) -> Option<crate::ast::ASTChildIterator> {
        let iter = iter::once(self.value.as_ast());
        Some(Box::new(iter))
    }
}

impl ExprASTNode for ReturnASTNode {}

impl ValueExprASTNode for ReturnASTNode {}

impl fmt::Display for ReturnASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return {}", self.span)
    }
}
