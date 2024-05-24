//! A module containing Negation operator AST node implementation.

use super::{bin_op_ast_node, operator_display, BinOperator};

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

bin_op_ast_node! {
    /// An AST node representing a negation or logical negation operator expression.
    NegExprASTNode {
        operator: NegOperator,
        label: "Negation",
    }
}
