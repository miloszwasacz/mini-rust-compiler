//! A module containing If AST node implementation.

use std::{fmt, iter};

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, BlockASTNode, ExprASTNode, ExpressionBox,
    ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing an if expression.
#[derive(Debug)]
pub struct IfASTNode {
    /// The condition can be [any expression](ExpressionBox::Unspecified).
    condition: Box<ExpressionBox>,
    then_block: Box<BlockASTNode>,
    else_block: Option<Box<BlockASTNode>>,
    span: Span,
}

impl IfASTNode {
    /// Creates a new `IfASTNode` with the given condition, then block, else block and span.
    pub fn new(
        condition: Box<ExpressionBox>,
        then_block: Box<BlockASTNode>,
        else_block: Box<BlockASTNode>,
        span: Span,
    ) -> IfASTNode {
        IfASTNode {
            condition,
            then_block,
            else_block: Some(else_block),
            span,
        }
    }

    /// Creates a new `IfASTNode` with the given condition, then block and span.
    pub fn new_without_else(
        condition: Box<ExpressionBox>,
        then_block: Box<BlockASTNode>,
        span: Span,
    ) -> IfASTNode {
        IfASTNode {
            condition,
            then_block,
            else_block: None,
            span,
        }
    }
}

impl ASTNode for IfASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let condition = iter::once(self.condition.as_ast());
        let then_block = iter::once(self.then_block.as_ast());
        let else_block = self.else_block.iter().map(|b| b.as_ast());
        let iter = condition.chain(then_block).chain(else_block);
        Some(Box::new(iter))
    }
}

impl ExprASTNode for IfASTNode {}

impl ValueExprASTNode for IfASTNode {}

impl fmt::Display for IfASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "If {}", self.span)
    }
}
