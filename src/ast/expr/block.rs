//! A module containing the Block AST node implementation.

use std::fmt;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode,
    StatementASTNode, ValueExprASTNode,
};
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
        let iter = self.statements.iter().map(|s| s.as_ast());
        Some(Box::new(iter))
    }
}

impl ExprASTNode for BlockASTNode {
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

impl ValueExprASTNode for BlockASTNode {}

impl fmt::Display for BlockASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block {}", self.span)
    }
}
