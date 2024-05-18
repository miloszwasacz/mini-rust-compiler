//! A concrete implementation of [LiteralASTNode] for integers.

use crate::ast::Type;
use crate::token::Span;

use super::{impl_ast, LiteralASTNode};

impl LiteralASTNode<i32> {
    /// Creates a new `LiteralASTNode<i32>` with the given value and span.
    pub fn new(value: i32, span: Span) -> LiteralASTNode<i32> {
        LiteralASTNode::new_generic(value, Type::I32, span)
    }
}

impl_ast!(i32);
