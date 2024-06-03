//! A module containing Return AST node implementation.

use std::{fmt, iter};

use inkwell::values::AnyValueEnum;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode,
    ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing a return expression.
#[derive(Debug)]
pub struct ReturnASTNode {
    /// The return value can be [any kind of expression](ExprASTNode).
    value: Option<Box<dyn ExprASTNode>>,
    span: Span,
}

impl ReturnASTNode {
    /// Creates a new `ReturnASTNode` with the given return value and span.
    pub fn new(value: Box<dyn ExprASTNode>, span: Span) -> ReturnASTNode {
        ReturnASTNode {
            value: Some(value),
            span,
        }
    }

    /// Creates a new `ReturnASTNode` with an empty return value and the given span.
    pub fn empty(span: Span) -> ReturnASTNode {
        ReturnASTNode { value: None, span }
    }
}

impl ASTNode for ReturnASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        self.value
            .as_ref()
            .map(|v| v.as_ast())
            .map(iter::once)
            .map(Box::new)
            .map(|b| b as ASTChildIterator)
    }
}

impl ExprASTNode for ReturnASTNode {
    fn try_as_place(&self) -> Option<&dyn PlaceExprASTNode> {
        None
    }

    fn try_as_value(&self) -> Option<&dyn ValueExprASTNode> {
        Some(self)
    }

    fn try_as_assignee(&self) -> Option<&dyn AssigneeExprASTNode> {
        None
    }
}

impl ValueExprASTNode for ReturnASTNode {}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for ReturnASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        todo!()
    }
}

impl fmt::Display for ReturnASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return {}", self.span)
    }
}
