//! A module containing Lazy Boolean operator AST node implementation.

use inkwell::values::AnyValueEnum;

use crate::codegen;
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
        todo!()
    }
}
