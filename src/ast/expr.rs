//! A module containing all the expression-related AST nodes.

use crate::ast::ASTNode;

pub use self::block::*;
pub use self::literal::*;
pub use self::operator::*;
pub use self::path::*;

pub mod block;
pub mod literal;
pub mod operator;
pub mod path;

/// A trait for all expression-related AST nodes.
pub trait ExprASTNode: ASTNode {}
