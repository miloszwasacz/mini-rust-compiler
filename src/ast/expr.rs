//! A module containing all the expression-related AST nodes.

use crate::ast::ASTNode;

pub mod literal;

/// A trait for all expression-related AST nodes.
pub trait ExpressionASTNode: ASTNode {}
