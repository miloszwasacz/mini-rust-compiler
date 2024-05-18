//! A concrete implementation of [LiteralASTNode] for floating-point numbers.

use crate::ast::Type;
use crate::token::Span;

use super::{impl_ast, LiteralASTNode};

impl LiteralASTNode<f64> {
    /// Creates a new `LiteralASTNode<f64>` with the given value and span.
    pub fn new(value: f64, span: Span) -> LiteralASTNode<f64> {
        LiteralASTNode::new_generic(value, Type::F64, span)
    }
}

impl_ast!(f64);
