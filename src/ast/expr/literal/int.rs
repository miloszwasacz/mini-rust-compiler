//! A concrete implementation of [LiteralASTNode] for integers.

use inkwell::values::AnyValueEnum;

use crate::ast::Type;
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

use super::{impl_ast, LiteralASTNode};

impl LiteralASTNode<i32> {
    /// Creates a new `LiteralASTNode<i32>` with the given value and span.
    pub fn new(value: i32, span: Span) -> LiteralASTNode<i32> {
        LiteralASTNode::new_generic(value, Type::I32, span)
    }
}

impl_ast!(i32);

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for LiteralASTNode<i32> {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        let i32_type = state.context().i32_type();
        let value = i32_type.const_int(self.value as u64, true);
        Ok(AnyValueEnum::IntValue(value))
    }
}
