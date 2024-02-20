//! A module containing Function Call AST node implementation.

use std::fmt;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, ExprASTNode};
use crate::token::Span;

/// An AST node representing a function call.
#[derive(Debug)]
pub struct FunCallASTNode {
    name: String,
    args: Vec<Box<dyn ExprASTNode>>,
    span: Span,
}

impl FunCallASTNode {
    /// Creates a new `FunCallASTNode` with the given name, arguments and span.
    pub fn new(name: String, args: Vec<Box<dyn ExprASTNode>>, span: Span) -> FunCallASTNode {
        FunCallASTNode { name, args, span }
    }

    /// Returns the name of the function being called.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl ASTNode for FunCallASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        Some(Box::new(self.args.iter().map(|arg| arg.as_ast())))
    }
}

impl ExprASTNode for FunCallASTNode {}

impl fmt::Display for FunCallASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FunCall {} {}", self.span, self.name)
    }
}
