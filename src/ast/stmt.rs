//! A module containing all the statement-related AST nodes.

use crate::ast::ASTNode;

pub use self::param::*;
pub use self::r#let::*;

pub mod r#let;
pub mod param;

/// A trait for all statement-related AST nodes.
pub trait StatementASTNode: ASTNode {}
