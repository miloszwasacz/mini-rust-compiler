//! A module containing errors & warnings that can occur during semantic analysis on the AST.

use std::error::Error;
use std::fmt;

use crate::token::Span;

/// An error that can occur during semantic analysis on the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticError {
    /// An error that occurs when an expression is expected to be of a certain kind,
    /// but is of a different kind.
    WrongExpressionKind {
        /// The message describing the problem with the expression.
        message: &'static str,
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

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticError::WrongExpressionKind { message, span } => {
                write!(f, "Wrong expression kind at {}: {}", span, message)
            }
            SemanticError::StaticWithoutInitializer { span } => {
                write!(f, "Static item declared without an initializer at {}", span)
            }
            SemanticError::ExternStaticWithInitializer { span } => {
                write!(
                    f,
                    "Static item declared with an initializer in an extern block at {}",
                    span
                )
            }
            SemanticError::ExternFunctionWithBody { span } => {
                write!(
                    f,
                    "Function declared with a body in an extern block at {}",
                    span
                )
            }
        }
    }
}

impl Error for SemanticError {}
