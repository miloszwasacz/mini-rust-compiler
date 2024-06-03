//! A module containing the Expression Statement AST node implementation.

use std::{fmt, iter};

use crate::ast::{ast_defaults, ASTChildIterator, ASTNode, ExprASTNode, StatementASTNode};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing an expression statement.
#[derive(Debug)]
pub struct ExprStmtASTNode {
    /// The expression can be [any kind of expression](ExprASTNode).
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
        self.expr.as_expr()
    }

    /// Extracts the expression.
    pub fn into_expr(self) -> Box<dyn ExprASTNode> {
        self.expr
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

impl<'ctx> CodeGen<'ctx, ()> for ExprStmtASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<()> {
        self.expr.code_gen(state)
    }
}

impl fmt::Display for ExprStmtASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expression Statement {}", self.span)
    }
}
