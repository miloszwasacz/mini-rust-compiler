//! A module containing all the expression-related AST nodes.

use std::fmt;

use debug_tree::TreeBuilder;

use crate::ast::{ASTChildIterator, ASTNode};
use crate::token::Span;

pub use self::assign::*;
pub use self::block::*;
pub use self::cast::*;
pub use self::fun_call::*;
pub use self::literal::*;
pub use self::operator::*;
pub use self::path::*;
pub use self::r#if::*;
pub use self::r#loop::*;
pub use self::r#return::*;
pub use self::underscore::*;
pub use self::unsafe_block::*;

mod assign;
mod block;
mod cast;
mod fun_call;
mod r#if;
mod literal;
mod r#loop;
mod operator;
mod path;
mod r#return;
mod underscore;
mod unsafe_block;

/// A macro for delegating method calls to the appropriate variant of an `ExpressionBox`.
macro_rules! expr_box_auto_impl {
    ($self:expr, $delegate:path) => {
        match $self {
            ExpressionBox::Unspecified(expr) => $delegate(expr.as_ref()),
            ExpressionBox::Place(expr) => $delegate(expr.as_ref()),
            ExpressionBox::Value(expr) => $delegate(expr.as_ref()),
            ExpressionBox::Assignee(expr) => $delegate(expr.as_ref()),
        }
    };
    ($self:expr, $delegate:path, $( $param:expr )* ) => {
        match $self {
            ExpressionBox::Unspecified(expr) => $delegate(expr.as_ref(), $( $param )*),
            ExpressionBox::Place(expr) => $delegate(expr.as_ref(), $( $param )*),
            ExpressionBox::Value(expr) => $delegate(expr.as_ref(), $( $param )*),
            ExpressionBox::Assignee(expr) => $delegate(expr.as_ref(), $( $param )*),
        }
    };
}

/// A trait for all expression-related AST nodes.
pub trait ExprASTNode: ASTNode + AsExprASTNode {}

/// A trait for all [place expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
/// AST nodes.
pub trait PlaceExprASTNode: AssigneeExprASTNode {}

/// A trait for all [value expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
/// AST nodes.
pub trait ValueExprASTNode: ExprASTNode {}

/// A trait for all [assignee expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
/// AST nodes.
pub trait AssigneeExprASTNode: ExprASTNode {}

/// An auto-trait for converting a type to a reference to a [general expression](ExprASTNode) AST node.
///
/// It is automatically implemented for all types that implement [`ExprASTNode`].
pub trait AsExprASTNode {
    /// Returns the type as a reference to a `dyn ExprASTNode`.
    fn as_expr(&self) -> &dyn ExprASTNode;
}

impl<T: ExprASTNode> AsExprASTNode for T {
    fn as_expr(&self) -> &dyn ExprASTNode {
        self
    }
}

/// An enum representing different kinds of expression AST nodes.
#[derive(Debug)]
pub enum ExpressionBox {
    /// A boxed [general expression](ExprASTNode) AST node.
    Unspecified(Box<dyn ExprASTNode>),
    /// A boxed [place expression](PlaceExprASTNode) AST node.
    Place(Box<dyn PlaceExprASTNode>),
    /// A boxed [value expression](ValueExprASTNode) AST node.
    Value(Box<dyn ValueExprASTNode>),
    /// A boxed [assignee expression](AssigneeExprASTNode) AST node.
    Assignee(Box<dyn AssigneeExprASTNode>),
}

/// An enum representing different kinds of [`ExpressionBox`] kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpressionBoxKind {
    /// A [general expression](ExprASTNode).
    Unspecified,
    /// A [place expression](PlaceExprASTNode).
    Place,
    /// A [value expression](ValueExprASTNode).
    Value,
    /// A [assignee expression](AssigneeExprASTNode).
    Assignee,
}

impl ExpressionBox {
    /// Returns the kind of the expression box.
    pub fn kind(&self) -> ExpressionBoxKind {
        match self {
            ExpressionBox::Unspecified(_) => ExpressionBoxKind::Unspecified,
            ExpressionBox::Place(_) => ExpressionBoxKind::Place,
            ExpressionBox::Value(_) => ExpressionBoxKind::Value,
            ExpressionBox::Assignee(_) => ExpressionBoxKind::Assignee,
        }
    }
}

impl fmt::Display for ExpressionBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        expr_box_auto_impl!(self, fmt::Display::fmt, f)
    }
}

impl fmt::Display for ExpressionBoxKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpressionBoxKind::Unspecified => write!(f, "Unspecified"),
            ExpressionBoxKind::Place => write!(f, "Place"),
            ExpressionBoxKind::Value => write!(f, "Value"),
            ExpressionBoxKind::Assignee => write!(f, "Assignee"),
        }
    }
}

impl ASTNode for ExpressionBox {
    fn span(&self) -> Span {
        expr_box_auto_impl!(self, ASTNode::span)
    }

    fn children(&self) -> Option<ASTChildIterator> {
        expr_box_auto_impl!(self, ASTNode::children)
    }

    fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
        expr_box_auto_impl!(self, ASTNode::add_to_tree_string, builder)
    }
}

impl AsExprASTNode for ExpressionBox {
    fn as_expr(&self) -> &dyn ExprASTNode {
        match self {
            ExpressionBox::Unspecified(expr) => expr.as_ref(),
            ExpressionBox::Place(expr) => expr.as_expr(),
            ExpressionBox::Value(expr) => expr.as_expr(),
            ExpressionBox::Assignee(expr) => expr.as_expr(),
        }
    }
}
