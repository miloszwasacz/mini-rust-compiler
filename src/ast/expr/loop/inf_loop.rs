//! A module containing the Infinite Loop AST node implementation.

use std::{fmt, iter};

use inkwell::values::AnyValueEnum;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, AssigneeExprASTNode, BlockASTNode,
    ExprASTNode, PlaceExprASTNode, ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing an infinite loop expression.
#[derive(Debug)]
pub struct InfLoopASTNode {
    block: Box<BlockASTNode>,
    span: Span,
}

impl InfLoopASTNode {
    /// Creates a new `InfLoopASTNode` with the given block and span.
    pub fn new(block: Box<BlockASTNode>, span: Span) -> InfLoopASTNode {
        InfLoopASTNode { block, span }
    }
}

impl ASTNode for InfLoopASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = iter::once(self.block.as_ast());
        Some(Box::new(iter))
    }
}

impl ExprASTNode for InfLoopASTNode {
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

impl ValueExprASTNode for InfLoopASTNode {}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for InfLoopASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        todo!()
    }
}

impl fmt::Display for InfLoopASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loop {}", self.span)
    }
}
