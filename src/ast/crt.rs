//! A module containing the Crate AST node implementation.

use std::fmt;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, ItemASTNode};
use crate::token::Span;

/// An AST node representing a crate.
#[derive(Debug)]
pub struct CrateASTNode {
    name: String,
    items: Vec<ItemASTNode>,
    span: Span,
}

impl CrateASTNode {
    /// Creates a new `CrateASTNode` with the given name, items and span.
    pub fn new(name: &str, items: Vec<ItemASTNode>, span: Span) -> CrateASTNode {
        CrateASTNode {
            name: name.to_string(),
            items,
            span,
        }
    }

    /// Returns the name of the crate.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl ASTNode for CrateASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = self.items.iter().map(|i| i.as_ast());
        Some(Box::new(iter))
    }
}

impl fmt::Display for CrateASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Crate")
    }
}
