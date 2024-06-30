//! A module containing the Infinite Loop AST node implementation.

use std::{fmt, iter};

use inkwell::values::AnyValueEnum;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, AssigneeExprASTNode, BlockASTNode,
    ExprASTNode, PlaceExprASTNode, ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::error::CodeGenError;
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
        //TODO Type checking -> loop has type `!` (unless it has a `break` statement)
        let parent_fn = state
            .get_current_function()
            .unwrap_or_else(|| panic!("Statement outside of function"));

        //#region Label
        let start_bb = state.context().append_basic_block(parent_fn, "loop");
        state
            .builder()
            .build_unconditional_branch(start_bb)
            .map_err(CodeGenError::from)?;
        //#endregion

        //#region Body
        state.builder().position_at_end(start_bb);
        self.block.code_gen(state)?;
        state
            .builder()
            .build_unconditional_branch(start_bb)
            .map_err(CodeGenError::from)?;
        //#endregion

        Ok(state.build_unit_value(self.span.end()))
    }
}

impl fmt::Display for InfLoopASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loop {}", self.span)
    }
}
