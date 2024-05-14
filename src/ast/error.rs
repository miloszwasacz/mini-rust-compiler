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
    /// An error that occurs when a static item is declared without an initializer.
    StaticWithoutInitializer {
        /// The span of the static item.
        span: Span,
    },
    /// An error that occurs when a static item is declared with an initializer
    /// in an extern block.
    ExternStaticWithInitializer {
        /// The span of the static item initializer.
        span: Span,
    },
    /// An error that occurs when a function is declared with a body
    /// in an extern block.
    ExternFunctionWithBody {
        /// The span of the function body.
        span: Span,
    },
}
