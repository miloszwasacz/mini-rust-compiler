//! A concrete implementation of [LiteralASTNode] for booleans.

use super::LiteralASTNode;
use crate::ast::VarType;
use crate::token::Span;

impl LiteralASTNode<bool> {
    /// Creates a new `LiteralASTNode<bool>` with the given value and span.
    pub fn new(value: bool, span: Span) -> LiteralASTNode<bool> {
        LiteralASTNode::new_generic(value, VarType::Bool, span)
    }
}
