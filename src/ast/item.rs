//! A module containing all the item AST nodes.

use std::fmt;

use debug_tree::TreeBuilder;

use crate::ast::{ASTChildIterator, ASTNode};
use crate::token::Span;

pub use self::func::*;
pub use self::proto::*;

mod func;
mod proto;

/// An AST node for an item.
#[derive(Debug)]
pub enum ItemASTNode {
    /// A function declaration.
    Func(Box<FuncASTNode>),
}

impl ASTNode for ItemASTNode {
    fn span(&self) -> Span {
        match self {
            ItemASTNode::Func(func) => func.span(),
        }
    }

    fn children(&self) -> Option<ASTChildIterator> {
        match self {
            ItemASTNode::Func(func) => func.children(),
        }
    }

    fn as_ast(&self) -> &dyn ASTNode {
        match self {
            ItemASTNode::Func(func) => func.as_ast(),
        }
    }

    fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
        match self {
            ItemASTNode::Func(func) => func.add_to_tree_string(builder),
        }
    }
}

impl fmt::Display for ItemASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemASTNode::Func(func) => write!(f, "{}", func),
        }
    }
}
