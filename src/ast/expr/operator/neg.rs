//! A module containing Negation operator AST node implementation.

use std::{fmt, iter};

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode,
    ValueExprASTNode,
};
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

impl fmt::Display for NegExprASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Operator (Negation) {} {}",
            self.span,
            self.operator.as_str(),
        )
    }
}
