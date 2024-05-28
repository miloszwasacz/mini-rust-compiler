//! A module containing Grouped Expression AST node implementation.

use std::{fmt, iter};

use crate::ast::{
    ast_defaults, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode, ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing a grouped expression (i.e. an expression wrapped in parentheses).
#[derive(Debug)]
pub struct GroupedExprASTNode {
    /// The expression can be [any kind of expression](ExprASTNode).
    expr: Box<dyn ExprASTNode>,
    span: Span,
}

impl GroupedExprASTNode {
    /// Creates a new `GroupedExprASTNode` with the given expression and span.
    pub fn new(expr: Box<dyn ExprASTNode>, span: Span) -> GroupedExprASTNode {
        GroupedExprASTNode { expr, span }
    }
}

impl ASTNode for GroupedExprASTNode {
    ast_defaults!();

    fn children(&self) -> Option<crate::ast::ASTChildIterator> {
        let iter = iter::once(self.expr.as_ast());
        Some(Box::new(iter))
    }
}

impl ExprASTNode for GroupedExprASTNode {
    fn try_as_place(&self) -> Option<&dyn PlaceExprASTNode> {
        None
    }

    fn try_as_value(&self) -> Option<&dyn ValueExprASTNode> {
        Some(self)
    }

    fn try_as_assignee(&self) -> Option<&dyn AssigneeExprASTNode> {
        None
    }
}

impl ValueExprASTNode for GroupedExprASTNode {}

impl fmt::Display for GroupedExprASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grouped {}", self.span)
    }
}
