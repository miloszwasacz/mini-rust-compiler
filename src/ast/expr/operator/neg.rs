//! A module containing Negation operator AST node implementation.

use std::{fmt, iter};

use inkwell::values::{AnyValue, AnyValueEnum};

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode,
    Type, ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::error::CodeGenError;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

use super::{operator_display, BinOperator};

/// An enum representing either a negation or a logical negation operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NegOperator {
    /// Numerical negation.
    Neg,
    /// Bitwise or logical negation.
    Not,
}

impl BinOperator for NegOperator {
    fn as_str(&self) -> &'static str {
        match self {
            NegOperator::Neg => "-",
            NegOperator::Not => "!",
        }
    }
}

operator_display!(NegOperator);

/// An AST node representing a negation or logical negation operator expression.
#[derive(Debug)]
pub struct NegExprASTNode {
    operator: NegOperator,
    /// The expression can be [any kind of expression](ExprASTNode).
    expr: Box<dyn ExprASTNode>,
    span: Span,
}

impl NegExprASTNode {
    /// Creates a new [`NegExprASTNode`] with the given operator, expression and span.
    pub fn new(operator: NegOperator, expr: Box<dyn ExprASTNode>, span: Span) -> NegExprASTNode {
        NegExprASTNode {
            operator,
            expr,
            span,
        }
    }
}

impl ASTNode for NegExprASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = iter::once(self.expr.as_ast());
        Some(Box::new(iter))
    }
}

impl ExprASTNode for NegExprASTNode {
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

impl ValueExprASTNode for NegExprASTNode {}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for NegExprASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        //TODO Refactor to use traits
        let expr = CodeGen::<AnyValueEnum>::code_gen(self.expr.as_ref(), state)?;

        //TODO Split into HIR & MIR and run type checking on HIR->MIR
        let expr_ty = Type::try_from_llvm_value(state.context(), expr, self.expr.span())?;

        let builder = state.builder();
        match self.operator {
            NegOperator::Neg => match expr_ty {
                Type::I32 => builder
                    .build_int_neg(expr.into_int_value(), "neg")
                    .map(|v| v.as_any_value_enum()),
                Type::F64 => builder
                    .build_float_neg(expr.into_float_value(), "neg")
                    .map(|v| v.as_any_value_enum()),
                _ => {
                    return Err(CodeGenError::UnsupportedType {
                        message: "Cannot perform numerical negation on non-numeric type".into(),
                        span: self.span,
                    });
                }
            },
            NegOperator::Not => match expr_ty {
                Type::Bool => builder
                    .build_not(expr.into_int_value(), "not")
                    .map(|v| v.as_any_value_enum()),
                _ => {
                    return Err(CodeGenError::UnsupportedType {
                        message: "Cannot perform logical negation on non-boolean type".into(),
                        span: self.span,
                    });
                }
            },
        }
        .map_err(Into::<CodeGenError>::into)
    }
}

impl fmt::Display for NegExprASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Operator (Negation) {} `{}`",
            self.span,
            self.operator.as_str(),
        )
    }
}
