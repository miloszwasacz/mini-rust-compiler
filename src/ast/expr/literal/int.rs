//! A concrete implementation of [LiteralASTNode] for integers.

use super::LiteralASTNode;
use crate::ast::VarType;
use crate::token::Span;

impl LiteralASTNode<i32> {
    /// Creates a new `LiteralASTNode<i32>` with the given value and span.
    pub fn new(value: i32, span: Span) -> LiteralASTNode<i32> {
        LiteralASTNode::new_generic(value, VarType::Int, span)
    }
}
