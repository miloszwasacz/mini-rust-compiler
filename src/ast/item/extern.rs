//! A module containing the Extern Block AST node implementation.

use std::fmt;
use std::rc::Rc;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, FuncProtoASTNode};
use crate::token::Span;

pub use self::r#static::*;

mod r#static;

/// An AST node representing an extern block.
#[derive(Debug)]
pub struct ExternASTNode {
    abi: Rc<str>,
    items: Vec<ExternItem>,
    span: Span,
}

/// An item in an extern block.
#[derive(Debug)]
pub enum ExternItem {
    /// A declaration of an extern function.
    Func(Box<FuncProtoASTNode>),
    /// A declaration of an extern static item.
    Static(Box<ExternStaticASTNode>),
}

impl ExternASTNode {
    /// Creates a new `ExternASTNode` with the given ABI, items and span.
    pub fn new(abi: Rc<str>, items: Vec<ExternItem>, span: Span) -> ExternASTNode {
        ExternASTNode { abi, items, span }
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
            ExternItem::Static(stat) => stat.as_ast(),
        }
    }
}

impl fmt::Display for ExternASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Extern {}", self.span)
    }
}
