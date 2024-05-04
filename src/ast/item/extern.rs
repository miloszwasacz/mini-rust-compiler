//! A module containing the Extern Block AST node implementation.

use std::fmt;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, FuncProtoASTNode};
use crate::token::Span;

/// An AST node representing an extern block.
#[derive(Debug)]
pub struct ExternASTNode {
    abi: String,
    items: Vec<ExternItem>,
    span: Span,
}

/// An item in an extern block.
#[derive(Debug)]
pub enum ExternItem {
    /// An extern function declaration.
    Func(Box<FuncProtoASTNode>),
}

impl ExternASTNode {
    /// Creates a new `ExternASTNode` with the given ABI, items and span.
    pub fn new(abi: &str, items: Vec<ExternItem>, span: Span) -> ExternASTNode {
        ExternASTNode {
            abi: abi.to_string(),
            items,
            span,
        }
    }

    /// Returns the used ABI.
    pub fn abi(&self) -> &str {
        &self.abi
    }
}

impl ASTNode for ExternASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = self.items.iter().map(|i| i.as_ast());
        Some(Box::new(iter))
    }
}

impl ExternItem {
    /// Returns a reference to this item as a `dyn ASTNode`.
    pub fn as_ast(&self) -> &dyn ASTNode {
        match self {
            ExternItem::Func(func) => func.as_ast(),
        }
    }
}

impl fmt::Display for ExternASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Extern {}", self.span)
    }
}
