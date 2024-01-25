//! A module containing Literal AST node implementations.

use std::fmt;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, ExpressionASTNode, VarType};
use crate::token::Span;

pub mod bool;
pub mod float;
pub mod int;

/// A generic AST node representing a literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralASTNode<T> {
    value: T,
    var_type: VarType,
    span: Span,
}

impl<T> LiteralASTNode<T> {
    /// Creates a new `LiteralASTNode` with the given value, type and span.
    ///
    /// This is the generic implementation of the `LiteralASTNode` constructor.
    /// Each concrete type [T] should have its own implementation of a constructor
    /// which calls this one while specifying the correct `var_type`.
    ///
    /// # Example
    /// ```ignore
    /// impl LiteralASTNode<i32> {
    ///     pub fn new(value: i32, span: Span) -> LiteralASTNode<i32> {
    ///         LiteralASTNode::new_generic(value, VarType::Int, span)
    ///     }
    /// }
    /// ```
    fn new_generic(value: T, var_type: VarType, span: Span) -> LiteralASTNode<T> {
        LiteralASTNode {
            value,
            var_type,
            span,
        }
    }

    /// Returns a reference to the value of the literal.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns the type of the literal.
    pub fn var_type(&self) -> VarType {
        self.var_type
    }
}

impl<T: fmt::Debug + fmt::Display> ASTNode for LiteralASTNode<T> {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        None
    }
}

impl ExpressionASTNode for LiteralASTNode<bool> {}

impl<T: fmt::Display> fmt::Display for LiteralASTNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Literal {} {} {}", self.span, self.var_type, self.value)
    }
}
