//TODO Add mention about the connection to LLVM types.
//! A module containing the types used in the AST.

use std::fmt;

//TODO Add support for more types.
//TODO Add a link to Inkwell's `AnyTypeEnum` documentation.
/// An enum representing the type of a variable.
///
/// This enum is closely related to LLVM types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarType {
    // For now, only i32
    /// An integer type.
    Int,
    // For now, only f64
    /// A floating-point type.
    Float,
    /// A boolean type.
    Bool,
    /// The unit type.
    Unit,
}

impl fmt::Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarType::Int => write!(f, "i32"),
            VarType::Float => write!(f, "f64"),
            VarType::Bool => write!(f, "bool"),
            VarType::Unit => write!(f, "()"),
        }
    }
}
