//! A module containing Return AST node implementation.

use std::{fmt, iter};

use crate::ast::{as_ast, ast_defaults, ASTNode, ExprASTNode, ValueExprASTNode};
use crate::token::Span;

/// An AST node representing a return expression.
#[derive(Debug)]
pub struct ReturnASTNode {
    value: Box<dyn ExprASTNode>,
    span: Span,
}

impl ReturnASTNode {
    /// Creates a new `ReturnASTNode` with the given return value and span.
    pub fn new(value: Box<dyn ExprASTNode>, span: Span) -> ReturnASTNode {
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
