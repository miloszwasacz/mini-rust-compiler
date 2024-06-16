//! Error types for code generation.

use inkwell::support::LLVMString;

use crate::ast::error::SemanticError;

/// The type of error that can occur during code generation.
#[derive(Debug, PartialEq, Eq)]
pub enum CodeGenError {
    /// A [`SemanticError`] encountered during code generation.
    SemanticError(SemanticError),
    /// An error returned by [`Module::verify`](inkwell::module::Module::verify).
    ModuleVerificationFailed(LLVMString),
}

impl From<SemanticError> for CodeGenError {
    fn from(err: SemanticError) -> CodeGenError {
        CodeGenError::SemanticError(err)
    }
}
