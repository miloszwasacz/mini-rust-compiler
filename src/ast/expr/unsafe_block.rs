//! A module containing the Unsafe Block AST node implementation.

use std::fmt;

use inkwell::values::AnyValueEnum;

use crate::ast::{
    ASTNode, AssigneeExprASTNode, BlockASTNode, ExprASTNode, PlaceExprASTNode, ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing an unsafe block expression.
#[derive(Debug)]
pub struct UnsafeBlockASTNode {
    block: Box<BlockASTNode>,
    span: Span,
}

impl UnsafeBlockASTNode {
    /// Creates a new `UnsafeBlockASTNode` with the given block and span.
    pub fn new(block: Box<BlockASTNode>, span: Span) -> UnsafeBlockASTNode {
        UnsafeBlockASTNode { block, span }
    }
}

impl ASTNode for UnsafeBlockASTNode {
    fn span(&self) -> Span {
        self.span
    }

    fn children(&self) -> Option<crate::ast::ASTChildIterator> {
        self.block.children()
    }
}

impl ExprASTNode for UnsafeBlockASTNode {
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

impl ValueExprASTNode for UnsafeBlockASTNode {}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for UnsafeBlockASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        CodeGen::<AnyValueEnum>::code_gen(self.block.as_ref(), state)
    }
}

impl fmt::Display for UnsafeBlockASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unsafe Block {}", self.span)
    }
}
