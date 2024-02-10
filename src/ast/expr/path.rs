//! A module containing Path AST node implementation.

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, ExprASTNode};
use crate::token::Span;

/// An AST node representing a path (i.e. a variable or item).
#[derive(Debug)]
pub struct PathASTNode {
    path: String,
    span: Span,
}

impl PathASTNode {
    /// Creates a new `PathASTNode` with the given path and span.
    pub fn new(path: String, span: Span) -> PathASTNode {
        PathASTNode { path, span }
    }
}

impl ASTNode for PathASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        None
    }
}

impl ExprASTNode for PathASTNode {}

impl std::fmt::Display for PathASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path {} {}", self.span, self.path)
    }
}
