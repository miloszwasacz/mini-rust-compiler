//! A module containing Comparison operator AST node implementation.

use inkwell::builder::Builder;
use inkwell::values::{AnyValue, AnyValueEnum};
use inkwell::{FloatPredicate, IntPredicate};

use crate::ast::{ASTNode, Type};
use crate::codegen;
use crate::codegen::error::CodeGenError;
use crate::codegen::{CodeGen, CodeGenState};

use super::{bin_op_ast_node, operator_display, BinOperator};

/// An enum representing a comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompOperator {
    /// Equality
    Eq,
    /// Inequality
    Ne,
    /// Greater than
    Gt,
    /// Less than
    Lt,
    /// Greater than or equal to
    Ge,
    /// Less than or equal to
    Le,
}

impl BinOperator for CompOperator {
    fn as_str(&self) -> &'static str {
        match self {
            CompOperator::Eq => "==",
            CompOperator::Ne => "!=",
            CompOperator::Gt => ">",
            CompOperator::Lt => "<",
            CompOperator::Ge => ">=",
            CompOperator::Le => "<=",
        }
    }
}

operator_display!(CompOperator);

bin_op_ast_node! {
    /// An AST node representing a comparison operator expression.
    CompExprASTNode {
        operator: CompOperator,
        label: "Comparison",
    }
}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for CompExprASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        //TODO Refactor to use traits
        let lhs = CodeGen::<AnyValueEnum>::code_gen(self.lhs.as_ref(), state)?;
        let rhs = CodeGen::<AnyValueEnum>::code_gen(self.rhs.as_ref(), state)?;

        //TODO Split into HIR & MIR and run type checking on HIR->MIR
        //#region Type checking
        let expr_ty = {
            let lhs_ty = Type::try_from_llvm_value(state.context(), lhs, self.lhs.span())?;
            let rhs_ty = Type::try_from_llvm_value(state.context(), rhs, self.rhs.span())?;

            if lhs_ty != rhs_ty {
                return Err(CodeGenError::TypeMismatch {
                    expected: lhs_ty,
                    actual: rhs_ty,
                    span: self.rhs.span(),
                });
            }

            lhs_ty
        };
        //#endregion

        //#region Type-specific code generation
        let codegen_int = |builder: &mut Builder<'ctx>| {
            let lhs = lhs.into_int_value();
            let rhs = rhs.into_int_value();
            let pred = match &self.operator {
                CompOperator::Eq => IntPredicate::EQ,
                CompOperator::Ne => IntPredicate::NE,
                CompOperator::Gt => IntPredicate::SGT,
                CompOperator::Lt => IntPredicate::SLT,
                CompOperator::Ge => IntPredicate::SGE,
                CompOperator::Le => IntPredicate::SLE,
            };

            builder.build_int_compare(pred, lhs, rhs, "cmp")
        };
        let codegen_float = |builder: &mut Builder<'ctx>| {
            let lhs = lhs.into_float_value();
            let rhs = rhs.into_float_value();
            let pred = match &self.operator {
                CompOperator::Eq => FloatPredicate::OEQ,
                CompOperator::Ne => FloatPredicate::UNE,
                CompOperator::Gt => FloatPredicate::OGT,
                CompOperator::Lt => FloatPredicate::OLT,
                CompOperator::Ge => FloatPredicate::OGE,
                CompOperator::Le => FloatPredicate::OLE,
            };

            builder.build_float_compare(pred, lhs, rhs, "cmp")
        };
        let codegen_bool = |builder: &mut Builder<'ctx>| {
            let lhs = lhs.into_int_value();
            let rhs = rhs.into_int_value();
            let pred = match &self.operator {
                CompOperator::Eq => IntPredicate::EQ,
                CompOperator::Ne => IntPredicate::NE,
                CompOperator::Gt => IntPredicate::UGT,
                CompOperator::Lt => IntPredicate::ULT,
                CompOperator::Ge => IntPredicate::UGE,
                CompOperator::Le => IntPredicate::ULE,
            };

            builder.build_int_compare(pred, lhs, rhs, "cmp")
        };
        //#endregion

        let builder = state.builder();
        match expr_ty {
            Type::I32 => codegen_int(builder),
            Type::F64 => codegen_float(builder),
            Type::Bool => codegen_bool(builder),
            Type::Unit => {
                //TODO Split into HIR & MIR and set this result as always true
                return Err(CodeGenError::UnsupportedType {
                    message: "Cannot perform comparison operations on unit type".into(),
                    span: self.span(),
                });
            }
        }
        .map(|v| v.as_any_value_enum())
        .map_err(Into::<CodeGenError>::into)
    }
}
