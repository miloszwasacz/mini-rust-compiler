//! A module containing all the statement-related AST nodes.

use crate::ast::ASTNode;

pub use self::param::*;

pub mod param;

/// A trait for all statement-related AST nodes.
pub trait StatementASTNode: ASTNode {}
