//! A module containing the Expression Statement AST node implementation.

use std::{fmt, iter};

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, ExprASTNode, StatementASTNode};
use crate::token::Span;

/// An AST node representing an expression statement.
#[derive(Debug)]
pub struct ExprStmtASTNode {
    expr: Box<dyn ExprASTNode>,
    span: Span,
}

impl ExprStmtASTNode {
    /// Creates a new `ExprStmtASTNode` with the given expression and span.
    pub fn new(expr: Box<dyn ExprASTNode>, span: Span) -> ExprStmtASTNode {
        ExprStmtASTNode { expr, span }
    }

    /// Returns the expression.
    pub fn expr(&self) -> &dyn ExprASTNode {
        self.expr.as_ref()
    }
}

impl ASTNode for ExprStmtASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = iter::once(self.expr.as_ast());
        Some(Box::new(iter))
    }
}

impl StatementASTNode for ExprStmtASTNode {}

impl fmt::Display for ExprStmtASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExprStmt {}", self.span)
    }
}
