//! A module containing errors & warnings that can occur during semantic analysis on the AST.

use crate::ast::ExpressionBoxKind;
use crate::token::Span;

/// An error that can occur during semantic analysis on the AST.
pub enum SemanticError {
    /// An error that occurs when an expression is expected to be of a certain kind,
    /// but is of a different kind.
    WrongExpressionKind {
        /// The expected kind of the expression.
        expected: ExpressionBoxKind,
        /// The actual kind of the expression found.
        actual: ExpressionBoxKind,
        /// The span of the expression.
        span: Span,
    },
}
