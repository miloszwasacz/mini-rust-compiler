//! A concrete implementation of [LiteralASTNode] for unit.

use crate::ast::Type;
use crate::token::Span;

use super::{impl_ast, LiteralASTNode};

impl LiteralASTNode<()> {
    /// Creates a new `LiteralASTNode<()>` with the given span.
    pub fn new(span: Span) -> LiteralASTNode<()> {
        LiteralASTNode::new_generic((), Type::Unit, span)
    }
}

impl_ast! {
    Type = ();

    impl fmt::Display for LiteralASTNode<()> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Literal {} {} {:?}", self.span, self.ty, self.value)
        }
    }
}
