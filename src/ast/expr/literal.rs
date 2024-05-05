//! A module containing Literal AST node implementations.

use std::fmt;

use crate::ast::{
    as_ast, ast_defaults, ASTChildIterator, ASTNode, ExprASTNode, Type, ValueExprASTNode,
};
use crate::token::Span;

mod bool;
mod float;
mod int;

/// A generic AST node representing a literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralASTNode<T> {
    value: T,
    ty: Type,
    span: Span,
}

impl<T> LiteralASTNode<T> {
    /// Creates a new `LiteralASTNode` with the given value, type and span.
    ///
    /// This is a generic implementation of the `LiteralASTNode` constructor.
    /// Each concrete type [T] should have its own implementation of the constructor
    /// which calls this one while specifying the correct `ty`.
    ///
    /// # Example
    /// ```ignore
    /// impl LiteralASTNode<i32> {
    ///     pub fn new(value: i32, span: Span) -> LiteralASTNode<i32> {
    ///         LiteralASTNode::new_generic(value, Type::I32, span)
    ///     }
    /// }
    /// ```
    fn new_generic(value: T, ty: Type, span: Span) -> LiteralASTNode<T> {
        LiteralASTNode { value, ty, span }
    }

    /// Returns a reference to the value of the literal.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns the type of the literal.
    pub fn ty(&self) -> Type {
        self.ty
    }
}

impl<T: fmt::Debug + fmt::Display> ASTNode for LiteralASTNode<T> {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        None
    }
}

impl<T: fmt::Debug + fmt::Display> ExprASTNode for LiteralASTNode<T> {}

impl<T: fmt::Debug + fmt::Display> ValueExprASTNode for LiteralASTNode<T> {}

impl<T: fmt::Display> fmt::Display for LiteralASTNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Literal {} {} {}", self.span, self.ty, self.value)
    }
}
