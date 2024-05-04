//! A module containing Arithmetic or Logical operator AST node implementation.

use std::{fmt, iter};

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, ExprASTNode, ValueExprASTNode};
use crate::token::Span;

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
