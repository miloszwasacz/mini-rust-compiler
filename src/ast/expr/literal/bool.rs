//! A concrete implementation of [LiteralASTNode] for booleans.

use crate::ast::Type;
use crate::token::Span;

use super::{impl_ast, LiteralASTNode};

impl LiteralASTNode<bool> {
    /// Creates a new `LiteralASTNode<bool>` with the given value and span.
    pub fn new(value: bool, span: Span) -> LiteralASTNode<bool> {
        LiteralASTNode::new_generic(value, Type::Bool, span)
    }
}

impl_ast!(bool);
