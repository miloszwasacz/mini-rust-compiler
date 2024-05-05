//! A module containing Path AST node implementation.

use std::rc::Rc;

use crate::ast::{
    as_ast, ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode,
    PlaceExprASTNode, ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing a path (i.e. a variable or item).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathASTNode {
    path: Rc<str>,
    span: Span,
}

impl PathASTNode {
    /// Creates a new `PathASTNode` with the given path and span.
    pub fn new(path: Rc<str>, span: Span) -> PathASTNode {
        PathASTNode { path, span }
    }

    /// Returns the path.
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl ASTNode for PathASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        None
    }
}

impl ExprASTNode for PathASTNode {}

impl PlaceExprASTNode for PathASTNode {}

impl ValueExprASTNode for PathASTNode {}

impl AssigneeExprASTNode for PathASTNode {}

impl std::fmt::Display for PathASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path {} {}", self.span, self.path)
    }
}
