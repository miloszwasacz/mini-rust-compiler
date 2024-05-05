//TODO Add mention about the connection to LLVM types.
//! This is a module containing the Type AST meta-node implementation.

use std::fmt;
use std::str::FromStr;

use crate::token::Span;

/// An AST meta-node representing a type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeASTMetaNode {
    ty: Type,
    span: Span,
}

impl TypeASTMetaNode {
    /// Creates a new `TypeMetaASTNode` with the given type and span.
    pub fn new(ty: Type, span: Span) -> TypeASTMetaNode {
        TypeASTMetaNode { ty, span }
    }

    /// Returns the type of this meta-node.
    pub fn ty(&self) -> Type {
        self.ty
    }

    /// Returns the span of this meta-node.
    pub fn span(&self) -> Span {
        self.span
    }
}

//TODO Add support for more types.
//TODO Add a link to Inkwell's `AnyTypeEnum` documentation.
/// An enum representing a type.
///
/// This enum is closely related to LLVM types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    /// A 32-bit integer type.
    I32,
    /// A 64-bit floating-point type.
    F64,
    /// A boolean type.
    Bool,
    /// The unit type.
    Unit,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "i32"),
            Type::F64 => write!(f, "f64"),
            Type::Bool => write!(f, "bool"),
            Type::Unit => write!(f, "()"),
        }
    }
}

impl fmt::Display for TypeASTMetaNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.ty, f)
    }
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "i32" => Ok(Type::I32),
            "f64" => Ok(Type::F64),
            "bool" => Ok(Type::Bool),
            "()" => Ok(Type::Unit),
            _ => Err(()),
        }
    }
}
