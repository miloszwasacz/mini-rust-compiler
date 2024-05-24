//! A module containing Literal AST node implementations.

use std::fmt;

use debug_tree::TreeBuilder;

use crate::ast::{ASTChildIterator, ASTNode, AsExprASTNode, ExprASTNode, Type};
use crate::token::Span;

mod bool;
mod float;
mod int;
mod unit;

/// A macro for delegating method calls to the appropriate variant of a `LiteralBox`.
macro_rules! lit_box_auto_impl {
    ($self:expr, $delegate:path) => {
        match $self {
            LiteralBox::I32(expr) => $delegate(expr.as_ref()),
            LiteralBox::Bool(expr) => $delegate(expr.as_ref()),
            LiteralBox::F64(expr) => $delegate(expr.as_ref()),
            LiteralBox::Unit(expr) => $delegate(expr.as_ref()),
        }
    };
    ($self:expr, $delegate:path, $( $param:expr )* ) => {
        match $self {
            LiteralBox::I32(expr) => $delegate(expr.as_ref(), $( $param )*),
            LiteralBox::F64(expr) => $delegate(expr.as_ref(), $( $param )*),
            LiteralBox::Bool(expr) => $delegate(expr.as_ref(), $( $param )*),
            LiteralBox::Unit(expr) => $delegate(expr.as_ref(), $( $param )*),
        }
    };
}

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

/// A boxed literal AST node.
#[derive(Debug)]
pub enum LiteralBox {
    /// A boxed [`LiteralASTNode<i32>`].
    I32(Box<LiteralASTNode<i32>>),
    /// A boxed [`LiteralASTNode<f64>`].
    F64(Box<LiteralASTNode<f64>>),
    /// A boxed [`LiteralASTNode<bool>`].
    Bool(Box<LiteralASTNode<bool>>),
    /// A boxed [`LiteralASTNode<()>`].
    Unit(Box<LiteralASTNode<()>>),
}

impl LiteralBox {
    /// Returns the type of the literal inside the box.
    pub fn ty(&self) -> Type {
        lit_box_auto_impl!(self, LiteralASTNode::ty)
    }
}

impl fmt::Display for LiteralBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        lit_box_auto_impl!(self, fmt::Display::fmt, f)
    }
}

impl ASTNode for LiteralBox {
    fn span(&self) -> Span {
        lit_box_auto_impl!(self, ASTNode::span)
    }

    fn children(&self) -> Option<ASTChildIterator> {
        lit_box_auto_impl!(self, ASTNode::children)
    }

    fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
        lit_box_auto_impl!(self, ASTNode::add_to_tree_string, builder)
    }
}

impl AsExprASTNode for LiteralBox {
    fn as_expr(&self) -> &dyn ExprASTNode {
        lit_box_auto_impl!(self, AsExprASTNode::as_expr)
    }
}
