//! A module containing Literal AST node implementations.

use crate::ast::Type;
use crate::token::Span;

mod bool;
mod float;
mod int;
mod unit;

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
    /// which calls this one while specifying the correct type `ty`.
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

macro_rules! impl_ast {
    ($ty:ty) => {
        impl_ast! {
            Type = $ty;

            impl fmt::Display for LiteralASTNode<$ty> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Literal {} {} {}", self.span, self.ty, self.value)
                }
            }
        }
    };
    (
        Type = $ty:ty;
        $display_impl:item
    ) => {
        use crate::ast::{
            ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode,
            PlaceExprASTNode, ValueExprASTNode,
        };
        use std::fmt;

        impl ASTNode for LiteralASTNode<$ty> {
            ast_defaults!();

            fn children(&self) -> Option<ASTChildIterator> {
                None
            }
        }

        impl ExprASTNode for LiteralASTNode<$ty> {
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

        impl ValueExprASTNode for LiteralASTNode<$ty> {}

        $display_impl
    };
}
use impl_ast;
