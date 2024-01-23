//! A concrete implementation of [LiteralASTNode] for floating-point numbers.

use super::LiteralASTNode;
use crate::ast::VarType;
use crate::token::Span;

impl LiteralASTNode<f64> {
    /// Creates a new `LiteralASTNode<f64>` with the given value and span.
    pub fn new(value: f64, span: Span) -> LiteralASTNode<f64> {
        LiteralASTNode::new_generic(value, VarType::Float, span)
    }
}
