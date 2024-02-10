//! A module containing the Block AST node implementation.

use std::fmt;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, ExprASTNode, StatementASTNode};
use crate::token::Span;

/// An AST node representing a block expression.
#[derive(Debug)]
pub struct BlockASTNode {
    statements: Vec<Box<dyn StatementASTNode>>,
    span: Span,
}

impl BlockASTNode {
    /// Creates a new `BlockASTNode` with the given statements and span.
    pub fn new(statements: Vec<Box<dyn StatementASTNode>>, span: Span) -> BlockASTNode {
        BlockASTNode { statements, span }
    }
}

impl ASTNode for BlockASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        Some(Box::new(self.statements.iter().map(|s| s.as_ast())))
    }
}

impl ExprASTNode for BlockASTNode {}

impl fmt::Display for BlockASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block {}", self.span)
    }
}
