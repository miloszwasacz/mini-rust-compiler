//! A module containing the Infinite Loop AST node implementation.

use std::fmt;

use crate::ast::{
    as_ast, ast_defaults, ASTChildIterator, ASTNode, BlockASTNode, ExprASTNode, ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing an infinite loop expression.
#[derive(Debug)]
pub struct InfLoopASTNode {
    block: Box<BlockASTNode>,
    span: Span,
}

impl InfLoopASTNode {
    /// Creates a new `InfLoopASTNode` with the given block and span.
    pub fn new(block: Box<BlockASTNode>, span: Span) -> InfLoopASTNode {
        InfLoopASTNode { block, span }
    }
}

impl ASTNode for InfLoopASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        Some(Box::new(std::iter::once(self.block.as_ast())))
    }
}

impl ExprASTNode for InfLoopASTNode {}

impl ValueExprASTNode for InfLoopASTNode {}

impl fmt::Display for InfLoopASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loop {}", self.span)
    }
}
