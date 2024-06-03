//! A concrete implementation of [LiteralASTNode] for booleans.

use inkwell::values::AnyValueEnum;

use crate::ast::Type;
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

use super::{impl_ast, LiteralASTNode};

impl LiteralASTNode<bool> {
    /// Creates a new `LiteralASTNode<bool>` with the given value and span.
    pub fn new(value: bool, span: Span) -> LiteralASTNode<bool> {
        LiteralASTNode::new_generic(value, Type::Bool, span)
    }
}

impl_ast!(bool);

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for LiteralASTNode<bool> {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        todo!()
    }
}
