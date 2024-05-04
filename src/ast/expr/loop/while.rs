//! A module containing the While Loop AST node implementation.

use std::{fmt, iter};

use crate::ast::{
    as_ast, ast_defaults, ASTChildIterator, ASTNode, BlockASTNode, ExprASTNode, ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing a while loop expression.
#[derive(Debug)]
pub struct WhileASTNode {
    condition: Box<dyn ExprASTNode>,
    block: Box<BlockASTNode>,
    span: Span,
}

impl WhileASTNode {
    /// Creates a new `WhileASTNode` with the given condition, block and span.
    pub fn new(
        condition: Box<dyn ExprASTNode>,
        block: Box<BlockASTNode>,
        span: Span,
    ) -> WhileASTNode {
        WhileASTNode {
            condition,
            block,
            span,
        }
    }
}

impl ASTNode for WhileASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let condition = iter::once(self.condition.as_ast());
        let block = iter::once(self.block.as_ast());
        let iter = condition.chain(block);
        Some(Box::new(iter))
    }
}

impl ExprASTNode for WhileASTNode {}

impl ValueExprASTNode for WhileASTNode {}

impl fmt::Display for WhileASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "While {}", self.span)
    }
}
