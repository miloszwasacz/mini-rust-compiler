//! A module containing Comparison operator AST node implementation.

use std::fmt;

use crate::ast::{as_ast, ast_defaults, ASTChildIterator, ASTNode, ExprASTNode};
use crate::token::Span;

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
