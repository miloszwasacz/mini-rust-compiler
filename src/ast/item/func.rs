//! A module containing the Function AST node implementation.

use std::{fmt, iter};

use debug_tree::TreeBuilder;
use inkwell::values::{AnyValue, AnyValueEnum};

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, BlockASTNode, FuncProtoASTNode,
};
use crate::codegen;
use crate::codegen::error::CodeGenError;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node for a function declaration.
#[derive(Debug)]
pub struct FuncASTNode {
    proto: FuncProtoASTNode,
    body: Box<BlockASTNode>,
    span: Span,
}

impl FuncASTNode {
    /// Creates a new `FuncASTNode` with the given prototype, body and span.
    pub fn new(proto: FuncProtoASTNode, body: BlockASTNode, span: Span) -> FuncASTNode {
        FuncASTNode {
            proto,
            body: Box::new(body),
            span,
        }
    }

    /// Returns the prototype of the function.
    pub fn proto(&self) -> &FuncProtoASTNode {
        &self.proto
    }
}

impl ASTNode for FuncASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let proto = iter::once(self.proto.as_ast());
        let body = iter::once(self.body.as_ast());
        let iter = proto.chain(body);
        Some(Box::new(iter))
    }

    fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
        let proto = self.proto.as_ast();
        let body = self.body.as_ast();

        let mut branch = builder.add_branch(format!("{self}").as_str());
        proto.add_to_tree_string(builder);
        {
            let mut branch = builder.add_branch("Body");
            body.add_to_tree_string(builder);
            branch.release();
        }
        branch.release();
    }
}

impl<'ctx> CodeGen<'ctx, ()> for FuncASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<()> {
        state.symbol_table().open_scope();

        let fn_value = state.symbol_table().get(self.proto().name()).map_or_else(
            || panic!(
                "Function \"{}\" could not be found in the symbol table. Did you forget to run the Collection Phase?",
                self.proto.name()
            ),
            |s| match s.value() {
                AnyValueEnum::FunctionValue(v) => v,
                _ => panic!(
                    "Function \"{}\" is not a function value.",
                    self.proto.name()
                ),
            },
        );

        //#region Parameters
        let param_iter = fn_value.get_param_iter().zip(self.proto().get_param_iter());
        for (llvm_param, param) in param_iter {
            let param = param.assignee().map_err(|e| {
                state.symbol_table().close_scope();
                CodeGenError::from(e)
            })?;

            if let Some(param) = param.pattern() {
                let name = param.clone();
                llvm_param.set_name(&name);
                state
                    .symbol_table()
                    .insert(name, llvm_param.as_any_value_enum());
            }
        }
        //#endregion

        //#region Body
        let body = state.context().append_basic_block(fn_value, "start");
        state.builder().position_at_end(body);
        self.body.code_gen(state).map_err(|e| {
            state.symbol_table().close_scope();
            e
        })?;
        //#endregion

        state.symbol_table().close_scope();
        Ok(())
    }
}

impl fmt::Display for FuncASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function \"{}\" {}", self.proto.name(), self.span)
    }
}
