//! A module containing the Function AST node implementation.

use std::{fmt, iter};

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, BlockASTNode, FuncProtoASTNode,
};
use crate::token::Span;

/// An AST node for a function declaration.
#[derive(Debug)]
pub struct FuncASTNode {
    proto: FuncProtoASTNode,
    body: Box<BlockASTNode>,
    span: Span,
}

impl FuncASTNode {
    /// Creates a new `FuncASTNode` with the given prototype, body and span.
    pub fn new(proto: FuncProtoASTNode, body: BlockASTNode, span: Span) -> FuncASTNode {
        FuncASTNode {
            proto,
            body: Box::new(body),
            span,
        }
    }
}

impl ASTNode for FuncASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let params = iter::once(self.proto.as_ast());
        let body = iter::once(self.body.as_ast());
        let iter = params.chain(body);
        Some(Box::new(iter))
    }
}

impl fmt::Display for FuncASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function \"{}\" {}", self.proto.name(), self.span)
    }
}
