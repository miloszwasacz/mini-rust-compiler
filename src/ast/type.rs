//TODO Add mention about the connection to LLVM types.
//! This is a module containing the Type AST meta-node implementation.

use std::fmt;
use std::str::FromStr;

use inkwell::context::Context;
use inkwell::types::{AnyType, AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::AnyValueEnum;

use crate::codegen;
use crate::codegen::error::CodeGenError;
use crate::codegen::{CodeGen, CodeGenState};
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

impl<'ctx> CodeGen<'ctx, AnyTypeEnum<'ctx>> for TypeASTMetaNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyTypeEnum<'ctx>> {
        CodeGen::<BasicTypeEnum>::code_gen(self, state).map(|bt| bt.as_any_type_enum())
    }
}

impl<'ctx> CodeGen<'ctx, BasicTypeEnum<'ctx>> for TypeASTMetaNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<BasicTypeEnum<'ctx>> {
        let context = state.context();
        Ok(match self.ty {
            Type::I32 => context.i32_type().as_basic_type_enum(),
            Type::F64 => context.f64_type().as_basic_type_enum(),
            Type::Bool => context.bool_type().as_basic_type_enum(),
            Type::Unit => context.struct_type(&[], false).as_basic_type_enum(),
        })
    }
}

impl<'ctx> CodeGen<'ctx, BasicMetadataTypeEnum<'ctx>> for TypeASTMetaNode {
    fn code_gen(
        &self,
        state: &mut CodeGenState<'ctx>,
    ) -> codegen::Result<BasicMetadataTypeEnum<'ctx>> {
        CodeGen::<BasicTypeEnum>::code_gen(self, state).map(|bt| bt.into())
    }
}

impl fmt::Display for TypeASTMetaNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.ty, f)
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

impl Type {
    /// Attempts to get the [`Type`] of an [`AnyValueEnum`].
    ///
    /// # Errors
    ///
    /// Returns a [`CodeGenError::UnsupportedType`] if the type is not supported.
    pub fn try_from_llvm_value<'ctx>(
        context: &'ctx Context,
        value: AnyValueEnum<'ctx>,
        span: Span,
    ) -> codegen::Result<Type> {
        match value.get_type() {
            // Bool
            AnyTypeEnum::IntType(i) if i.get_bit_width() == 1 => Ok(Type::Bool),
            // I32
            AnyTypeEnum::IntType(i) if i.get_bit_width() == 32 => Ok(Type::I32),
            // F64
            AnyTypeEnum::FloatType(f) if f == context.f64_type() => Ok(Type::F64),
            // Unit
            AnyTypeEnum::StructType(s) if s.count_fields() == 0 => Ok(Type::Unit),
            // Unsupported ints
            AnyTypeEnum::IntType(_) => Err(CodeGenError::UnsupportedType {
                message: "Unsupported int type (only `i32` is supported)".into(),
                span,
            }),
            // Unsupported floats
            AnyTypeEnum::FloatType(_) => Err(CodeGenError::UnsupportedType {
                message: "Unsupported float type (only `f64` is supported)".into(),
                span,
            }),
            ty => Err(CodeGenError::UnsupportedType {
                message: format!("Unsupported type: {}", ty).into_boxed_str(),
                span,
            }),
        }
    }
}
