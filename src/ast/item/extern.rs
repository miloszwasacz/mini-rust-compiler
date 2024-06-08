//! A module containing the Extern Block AST node implementation.

use std::fmt;
use std::rc::Rc;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, FuncProtoASTNode, StaticASTNode,
};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

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
    Static(Box<StaticASTNode>),
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

    /// Returns the items in the extern block.
    pub fn items(&self) -> &[ExternItem] {
        &self.items
    }
}

impl ASTNode for ExternASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = self.items.iter().map(|i| i.as_ast());
        Some(Box::new(iter))
    }
}

impl<'ctx> CodeGen<'ctx, ()> for ExternASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<()> {
        todo!()
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
