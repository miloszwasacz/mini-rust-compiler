//! A module containing all the expression-related AST nodes.

use crate::ast::ASTNode;

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
