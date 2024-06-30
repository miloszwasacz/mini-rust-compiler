//! Error types for code generation.

use inkwell::builder::BuilderError;
use inkwell::support::LLVMString;

use crate::ast::error::SemanticError;
use crate::ast::Type;
use crate::token::Span;

/// The type of error that can occur during code generation.
#[derive(Debug, PartialEq, Eq)]
pub enum CodeGenError {
    /// A [`SemanticError`] encountered during code generation.
    SemanticError(SemanticError),
    /// An error when a symbol that is not found in the symbol table is encountered.
    MissingSymbol {
        /// The name of the missing symbol.
        symbol: Box<str>,
        /// The span where the error occurred.
        span: Span,
    },
    /// An error when an [LLVM value](inkwell::values::AnyValueEnum) is not of the expected type.
    InvalidLLVMValueType {
        /// The description of the error.
        message: Box<str>,
        /// The span where the error occurred.
        span: Span,
    },
    /// An error when an underscore is used as a value.
    UnderscoreUsedAsValue {
        /// The span where the error occurred.
        span: Span,
    },
    /// An error when an unsupported type is encountered.
    UnsupportedType {
        /// The description of the error.
        message: Box<str>,
        /// The span where the error occurred.
        span: Span,
    },
    /// An error when an operation requires a different type than the one provided.
    TypeMismatch {
        /// The expected type.
        expected: Type,
        /// The actual type.
        actual: Type,
        /// The span where the error occurred.
        span: Span,
    },
    /// An error returned by all `inkwell::builder::Builder::build_*` methods.
    BuilderError(BuilderError),
    /// An error returned by [`Module::verify`](inkwell::module::Module::verify).
    ModuleVerificationFailed(LLVMString),
}

impl From<SemanticError> for CodeGenError {
    fn from(err: SemanticError) -> CodeGenError {
        CodeGenError::SemanticError(err)
    }
}

impl From<BuilderError> for CodeGenError {
    fn from(err: BuilderError) -> CodeGenError {
        CodeGenError::BuilderError(err)
    }
}
