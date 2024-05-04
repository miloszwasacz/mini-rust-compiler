//! A module containing the Function Prototype AST node implementation.

use std::fmt;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, ParamASTNode, VarType};
use crate::token::Span;

/// An AST node representing a function prototype.
#[derive(Debug)]
pub struct FuncProtoASTNode {
    name: String,
    params: Vec<ParamASTNode>,
    return_type: VarType,
    span: Span,
}

impl FuncProtoASTNode {
    /// Creates a new `FuncProtoASTNode` with the given name, parameters, return type and span.
    pub fn new(
        name: String,
        params: Vec<ParamASTNode>,
        return_type: VarType,
        span: Span,
    ) -> FuncProtoASTNode {
        FuncProtoASTNode {
            name,
            params,
            return_type,
            span,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the return type.
    pub fn return_type(&self) -> VarType {
        self.return_type
    }
}

impl ASTNode for FuncProtoASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = self.params.iter().map(|p| p.as_ast());
        Some(Box::new(iter))
    }
}

impl fmt::Display for FuncProtoASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function Prototype: {}", self.name)
    }
}
