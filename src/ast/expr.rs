//! A module containing all the expression-related AST nodes.

use crate::ast::ASTNode;

pub use self::assign::*;
pub use self::block::*;
pub use self::cast::*;
pub use self::literal::*;
pub use self::operator::*;
pub use self::path::*;

pub mod assign;
pub mod block;
pub mod cast;
mod fun_call;
pub mod literal;
pub mod operator;
pub mod path;

/// A trait for all expression-related AST nodes.
pub trait ExprASTNode: ASTNode {}

/// A trait for all [place expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
/// AST nodes.
pub trait PlaceExprASTNode: AssigneeExprASTNode {}

/// A trait for all [value expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
/// AST nodes.
pub trait ValueExprASTNode: ExprASTNode {}

/// A trait for all [assignee expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
/// AST nodes.
pub trait AssigneeExprASTNode: ASTNode {}
