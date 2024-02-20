//! A module containing Underscore AST node implementation.

use std::fmt;

use crate::ast::{
    as_ast, ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode,
};
use crate::token::Span;

/// An AST node representing an underscore.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UnderscoreASTNode {
    span: Span,
}

impl UnderscoreASTNode {
    /// Creates a new `UnderscoreASTNode` with the given span.
    pub fn new(span: Span) -> UnderscoreASTNode {
        UnderscoreASTNode { span }
    }
}

impl ASTNode for UnderscoreASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        None
    }
}

impl ExprASTNode for UnderscoreASTNode {}

impl AssigneeExprASTNode for UnderscoreASTNode {}

impl fmt::Display for UnderscoreASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Underscore {}", self.span)
    }
}
