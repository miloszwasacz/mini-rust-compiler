//! A module containing Function Call AST node implementation.

use std::fmt;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, ExprASTNode, ExpressionBox, PathASTNode,
    ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing a function call.
#[derive(Debug)]
pub struct FunCallASTNode {
    path: Box<PathASTNode>,
    /// The arguments can be [any kind of expressions](ExpressionBox::Unspecified).
    args: Vec<ExpressionBox>,
    span: Span,
}

impl FunCallASTNode {
    /// Creates a new `FunCallASTNode` with the given path, arguments and span.
    pub fn new(path: Box<PathASTNode>, args: Vec<ExpressionBox>, span: Span) -> FunCallASTNode {
        FunCallASTNode { path, args, span }
    }

    /// Returns the path to the called function.
    pub fn path(&self) -> &str {
        self.path.path()
    }
}

impl ASTNode for FunCallASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = self.args.iter().map(|arg| arg.as_ast());
        Some(Box::new(iter))
    }
}

impl ExprASTNode for FunCallASTNode {}

impl ValueExprASTNode for FunCallASTNode {}

impl fmt::Display for FunCallASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function Call {} {}", self.span, self.path())
    }
}
