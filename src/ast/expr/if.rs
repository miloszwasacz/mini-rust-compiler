//! A module containing If AST node implementation.

use std::{fmt, iter};

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, AssigneeExprASTNode, BlockASTNode,
    ExprASTNode, PlaceExprASTNode, ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing an if expression.
#[derive(Debug)]
pub struct IfASTNode {
    /// The condition can be [any kind of expression](ExprASTNode).
    condition: Box<dyn ExprASTNode>,
    then_block: Box<BlockASTNode>,
    else_block: Option<Box<BlockASTNode>>,
    span: Span,
}

impl IfASTNode {
    /// Creates a new `IfASTNode` with the given condition, then block, else block and span.
    pub fn new(
        condition: Box<dyn ExprASTNode>,
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
        condition: Box<dyn ExprASTNode>,
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

impl ExprASTNode for IfASTNode {
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

impl ValueExprASTNode for IfASTNode {}

impl fmt::Display for IfASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "If {}", self.span)
    }
}
