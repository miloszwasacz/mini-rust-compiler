//! A module containing the While Loop AST node implementation.

use std::{fmt, iter};

use debug_tree::TreeBuilder;
use inkwell::values::AnyValueEnum;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, AssigneeExprASTNode, BlockASTNode,
    ExprASTNode, PlaceExprASTNode, ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::error::CodeGenError;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing a while loop expression.
#[derive(Debug)]
pub struct WhileASTNode {
    /// The condition can be [any kind of expression](ExprASTNode).
    condition: Box<dyn ExprASTNode>,
    body: Box<BlockASTNode>,
    span: Span,
}

impl WhileASTNode {
    /// Creates a new `WhileASTNode` with the given condition, body and span.
    pub fn new(
        condition: Box<dyn ExprASTNode>,
        body: Box<BlockASTNode>,
        span: Span,
    ) -> WhileASTNode {
        WhileASTNode {
            condition,
            body,
            span,
        }
    }
}

impl ASTNode for WhileASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let condition = iter::once(self.condition.as_ast());
        let body = iter::once(self.body.as_ast());
        let iter = condition.chain(body);
        Some(Box::new(iter))
    }

    fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
        let condition = self.condition.as_ast();
        let body = self.body.as_ast();

        let mut branch = builder.add_branch(format!("{self}").as_str());
        {
            let mut branch = builder.add_branch("Condition");
            condition.add_to_tree_string(builder);
            branch.release()
        }
        {
            let mut branch = builder.add_branch("Body");
            body.add_to_tree_string(builder);
            branch.release()
        }
        branch.release()
    }
}

impl ExprASTNode for WhileASTNode {
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

impl ValueExprASTNode for WhileASTNode {}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for WhileASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        //TODO Type checking -> loop has type `!` (unless it has a `break` statement)
        let parent_fn = state
            .get_current_function()
            .unwrap_or_else(|| panic!("Statement outside of function"));

        //#region Labels
        let cond_bb = state.context().append_basic_block(parent_fn, "cond");
        let body_bb = state.context().append_basic_block(parent_fn, "body");
        let end_bb = state.context().append_basic_block(parent_fn, "end");
        state
            .builder()
            .build_unconditional_branch(cond_bb)
            .map_err(CodeGenError::from)?;
        //#endregion

        //#region Condition
        state.builder().position_at_end(cond_bb);
        let cond = state.build_condition(self.condition.as_ref(), true)?;
        state
            .builder()
            .build_conditional_branch(cond, body_bb, end_bb)
            .map_err(CodeGenError::from)?;
        //#endregion

        //#region Body
        state.builder().position_at_end(body_bb);
        self.body.code_gen(state)?;
        state
            .builder()
            .build_unconditional_branch(cond_bb)
            .map_err(CodeGenError::from)?;
        //#endregion

        state.builder().position_at_end(end_bb);

        Ok(state.build_unit_value(self.span.end()))
    }
}

impl fmt::Display for WhileASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "While {}", self.span)
    }
}
