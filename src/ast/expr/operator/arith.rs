//! A module containing Arithmetic or Logical operator AST node implementation.

use inkwell::builder::Builder;
use inkwell::values::{AnyValue, AnyValueEnum};

use codegen::error::CodeGenError;

use crate::ast::{ASTNode, Type};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};

use super::{bin_op_ast_node, operator_display, BinOperator};

/// An enum representing either an arithmetic or a logical binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithOperator {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Remainder
    Rem,
    /// Bitwise AND
    BitAnd,
    /// Bitwise OR
    BitOr,
    /// Bitwise XOR
    BitXor,
}

impl BinOperator for ArithOperator {
    fn as_str(&self) -> &'static str {
        match self {
            ArithOperator::Add => "+",
            ArithOperator::Sub => "-",
            ArithOperator::Mul => "*",
            ArithOperator::Div => "/",
            ArithOperator::Rem => "%",
            ArithOperator::BitAnd => "&",
            ArithOperator::BitOr => "|",
            ArithOperator::BitXor => "^",
        }
    }
}

operator_display!(ArithOperator);

bin_op_ast_node! {
    /// An AST node representing an arithmetic or logical operator expression.
    ArithExprASTNode {
        operator: ArithOperator,
        label: "Arithmetic or Logical",
    }
}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for ArithExprASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        //TODO Refactor to use traits
        let lhs = CodeGen::<AnyValueEnum>::code_gen(self.lhs.as_ref(), state)?;
        let rhs = CodeGen::<AnyValueEnum>::code_gen(self.rhs.as_ref(), state)?;

        //TODO Split into HIR & MIR and run type checking on HIR->MIR
        //#region Type checking
        let expr_type = {
            let lhs_ty = Type::try_from_llvm_value(state.context(), lhs, self.lhs.span())?;
            let rhs_ty = Type::try_from_llvm_value(state.context(), rhs, self.rhs.span())?;

            if let (Type::Unit, _) | (_, Type::Unit) = (lhs_ty, rhs_ty) {
                return Err(CodeGenError::UnsupportedType {
                    message: "Cannot perform arithmetic operations on unit type".into(),
                    span: self.span(),
                });
            }
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

            match &self.operator {
                ArithOperator::Add => builder.build_int_add(lhs, rhs, "add"),
                ArithOperator::Sub => builder.build_int_sub(lhs, rhs, "sub"),
                ArithOperator::Mul => builder.build_int_mul(lhs, rhs, "mul"),
                ArithOperator::Div => builder.build_int_signed_div(lhs, rhs, "div"),
                ArithOperator::Rem => builder.build_int_signed_rem(lhs, rhs, "rem"),
                ArithOperator::BitAnd => builder.build_and(lhs, rhs, "and"),
                ArithOperator::BitOr => builder.build_or(lhs, rhs, "or"),
                ArithOperator::BitXor => builder.build_xor(lhs, rhs, "xor"),
            }
            .map(|v| v.as_any_value_enum())
            .map_err(Into::<CodeGenError>::into)
        };
        let codegen_float = |builder: &mut Builder<'ctx>| {
            let lhs = lhs.into_float_value();
            let rhs = rhs.into_float_value();

            match &self.operator {
                ArithOperator::Add => builder.build_float_add(lhs, rhs, "add"),
                ArithOperator::Sub => builder.build_float_sub(lhs, rhs, "sub"),
                ArithOperator::Mul => builder.build_float_mul(lhs, rhs, "mul"),
                ArithOperator::Div => builder.build_float_div(lhs, rhs, "div"),
                ArithOperator::Rem => builder.build_float_rem(lhs, rhs, "rem"),
                ArithOperator::BitAnd | ArithOperator::BitOr | ArithOperator::BitXor => {
                    let message = "Bitwise operations are not supported on floating point operands";
                    return Err(CodeGenError::UnsupportedType {
                        message: message.into(),
                        span: self.span(),
                    });
                }
            }
            .map(|v| v.as_any_value_enum())
            .map_err(Into::<CodeGenError>::into)
        };
        let codegen_bool = |builder: &mut Builder<'ctx>| {
            let lhs = lhs.into_int_value();
            let rhs = rhs.into_int_value();

            match &self.operator {
                ArithOperator::BitAnd => builder.build_and(lhs, rhs, "and"),
                ArithOperator::BitOr => builder.build_or(lhs, rhs, "or"),
                ArithOperator::BitXor => builder.build_xor(lhs, rhs, "xor"),
                ArithOperator::Add
                | ArithOperator::Sub
                | ArithOperator::Mul
                | ArithOperator::Div
                | ArithOperator::Rem => {
                    let message = "Only bitwise operations are supported on boolean operands";
                    return Err(CodeGenError::UnsupportedType {
                        message: message.into(),
                        span: self.span(),
                    });
                }
            }
            .map(|v| v.as_any_value_enum())
            .map_err(Into::<CodeGenError>::into)
        };
        //#endregion

        let builder = state.builder();
        match expr_type {
            Type::I32 => codegen_int(builder),
            Type::F64 => codegen_float(builder),
            Type::Bool => codegen_bool(builder),
            Type::Unit => unreachable!("Unit type should have been handled earlier."),
        }
    }
}
