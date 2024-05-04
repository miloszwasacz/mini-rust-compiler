//! A module containing the Unsafe Block AST node implementation.

use std::fmt;

use crate::ast::{as_ast, ASTNode, BlockASTNode, ExprASTNode, ValueExprASTNode};
use crate::token::Span;

/// An AST node representing an unsafe block expression.
#[derive(Debug)]
pub struct UnsafeBlockASTNode {
    block: Box<BlockASTNode>,
    span: Span,
}

impl UnsafeBlockASTNode {
    /// Creates a new `UnsafeBlockASTNode` with the given block and span.
    pub fn new(block: Box<BlockASTNode>, span: Span) -> UnsafeBlockASTNode {
        UnsafeBlockASTNode { block, span }
    }
}

impl ASTNode for UnsafeBlockASTNode {
    as_ast!();

    fn span(&self) -> Span {
        self.span
    }

    fn children(&self) -> Option<crate::ast::ASTChildIterator> {
        self.block.children()
    }
}

impl ExprASTNode for UnsafeBlockASTNode {}

impl ValueExprASTNode for UnsafeBlockASTNode {}

impl fmt::Display for UnsafeBlockASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnsafeBlock {}", self.span)
    }
}
