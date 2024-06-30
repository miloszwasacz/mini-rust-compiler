//! A concrete implementation of [LiteralASTNode] for unit.

use inkwell::values::{AnyValue, AnyValueEnum};

use crate::ast::Type;
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::{Position, Span};

use super::{impl_ast, LiteralASTNode};

impl LiteralASTNode<()> {
    /// Creates a new `LiteralASTNode<()>` with the given span.
    pub fn new(span: Span) -> LiteralASTNode<()> {
        LiteralASTNode::new_generic((), Type::Unit, span)
    }
}

impl_ast! {
    Type = ();

    impl fmt::Display for LiteralASTNode<()> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Literal {} `{}` \"{:?}\"", self.span, self.ty, self.value)
        }
    }
}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for LiteralASTNode<()> {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        let unit_type = state.context().struct_type(&[], false);
        let value = unit_type.const_zero();
        Ok(value.as_any_value_enum())
    }
}

impl<'ctx> CodeGenState<'ctx> {
    //TODO Add Examples to the documentation
    /// Generates a new unit [`LLVM value`](AnyValueEnum) with the span that starts and ends at `end_pos`.
    pub fn build_unit_value(
        self: &mut CodeGenState<'ctx>,
        end_pos: Position,
    ) -> AnyValueEnum<'ctx> {
        let span = Span::new(end_pos, end_pos);
        let unit = LiteralASTNode::<()>::new(span);
        CodeGen::<AnyValueEnum>::code_gen(&unit, self).unwrap()
    }
}
