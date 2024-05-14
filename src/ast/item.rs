//! A module containing all the item AST nodes.

use std::fmt;

use debug_tree::TreeBuilder;

use crate::ast::{ASTChildIterator, ASTNode};
use crate::token::Span;

pub use self::func::*;
pub use self::proto::*;
pub use self::r#extern::*;
pub use self::r#static::*;

mod r#extern;
mod func;
mod proto;
mod r#static;

/// Delegate a method call to AST node contained in the ItemASTNode.
macro_rules! delegate_ast {
    ($self:expr, $method:ident, $($param:expr)*) => {
        match $self {
            ItemASTNode::Func(func) => func.$method($($param)*),
            ItemASTNode::Extern(ext) => ext.$method($($param)*),
            ItemASTNode::Static(stat) => stat.$method($($param)*),
        }
    };
}

/// An AST node for an item.
#[derive(Debug)]
pub enum ItemASTNode {
    /// A function declaration.
    Func(Box<FuncASTNode>),
    /// An extern block.
    Extern(Box<ExternASTNode>),
    /// A static item.
    Static(Box<StaticASTNode>),
}

impl ASTNode for ItemASTNode {
    fn span(&self) -> Span {
        delegate_ast!(&self, span,)
    }

    fn children(&self) -> Option<ASTChildIterator> {
        delegate_ast!(&self, children,)
    }

    fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
        delegate_ast!(&self, add_to_tree_string, builder)
    }
}

impl fmt::Display for ItemASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ItemASTNode::Func(func) => write!(f, "{}", func),
            ItemASTNode::Extern(ext) => write!(f, "{}", ext),
            ItemASTNode::Static(stat) => write!(f, "{}", stat),
        }
    }
}
