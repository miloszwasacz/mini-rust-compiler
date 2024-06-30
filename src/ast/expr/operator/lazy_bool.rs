//! A module containing Lazy Boolean operator AST node implementation.

use inkwell::values::{AnyValue, AnyValueEnum};

use crate::codegen;
use crate::codegen::error::CodeGenError;
use crate::codegen::{CodeGen, CodeGenState};

use super::{bin_op_ast_node, operator_display, BinOperator};

/// An enum representing a lazy boolean operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyBoolOperator {
    /// Logical AND
    And,
    /// Logical OR
    Or,
}

impl BinOperator for LazyBoolOperator {
    fn as_str(&self) -> &'static str {
        match self {
            LazyBoolOperator::And => "&&",
            LazyBoolOperator::Or => "||",
        }
    }
}

operator_display!(LazyBoolOperator);

bin_op_ast_node! {
    /// An AST node representing a lazy boolean operator expression.
    LazyBoolExprASTNode {
        operator: LazyBoolOperator,
        label: "Lazy Boolean",
    }
}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for LazyBoolExprASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        let no_bb_panic = || panic!("Builder not in a block");

        let parent_fn = state
            .get_current_function()
            .unwrap_or_else(|| panic!("Expression outside of function"));

        //#region Labels
        let lhs_bb = state.context().append_basic_block(parent_fn, "lhs");
        let rhs_bb = state.context().append_basic_block(parent_fn, "rhs");
        let merge_bb = state.context().append_basic_block(parent_fn, "merge");
        state
            .builder()
            .build_unconditional_branch(lhs_bb)
            .map_err(CodeGenError::from)?;
        //#endregion

        //#region LHS
        state.builder().position_at_end(lhs_bb);

        let lhs = state.build_condition(
            self.lhs.as_ref(),
            match self.operator {
                LazyBoolOperator::And => true,
                LazyBoolOperator::Or => false,
            },
        )?;
        state
            .builder()
            .build_conditional_branch(lhs, rhs_bb, merge_bb)
            .map_err(CodeGenError::from)?;

        // The block may have changed because LHS could have added its own blocks
        let lhs_bb = state
            .builder()
            .get_insert_block()
            .unwrap_or_else(no_bb_panic);
        //#endregion

        //#region RHS
        state.builder().position_at_end(rhs_bb);

        let rhs = state.build_bool(self.rhs.as_ref())?;
        state
            .builder()
            .build_unconditional_branch(merge_bb)
            .map_err(CodeGenError::from)?;

        // The block may have changed because RHS could have added its own blocks
        let rhs_bb = state
            .builder()
            .get_insert_block()
            .unwrap_or_else(no_bb_panic);
        //#endregion

        //#region Merge
        state.builder().position_at_end(merge_bb);

        let ty = state.context().bool_type();
        let phi = state
            .builder()
            .build_phi(ty, "lazybool")
            .map_err(CodeGenError::from)?;
        phi.add_incoming(&[(&lhs, lhs_bb), (&rhs, rhs_bb)]);
        //#endregion

        Ok(phi.as_any_value_enum())
    }
}
