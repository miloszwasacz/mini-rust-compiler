//! A module containing all the statement-related AST nodes.

use crate::ast::ASTNode;

pub use self::expr::*;
pub use self::r#let::*;

mod expr;
mod r#let;

/// A trait for all statement-related AST nodes.
pub trait StatementASTNode: ASTNode {}
