//! A module containing the types and traits used for code generation.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::FunctionType;
use inkwell::values::AnyValue;

use crate::ast::{CrateASTNode, ExternItem, FuncProtoASTNode, ItemASTNode, StaticASTNode};

use self::error::CodeGenError;
use self::symbol_table::*;

pub mod error;
mod symbol_table;

/// The state of the code generation process.
pub struct CodeGenState<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    symbol_table: SymbolTable<'ctx>,
}

impl<'ctx> CodeGenState<'ctx> {
    /// Creates a new code generation state with the `module_name` based on the given `context`.
    pub fn new(context: &'ctx Context, module_name: &str) -> CodeGenState<'ctx> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let symbol_table = SymbolTable::new();

        CodeGenState {
            context,
            module,
            builder,
            symbol_table,
        }
    }

    /// Returns the LLVM context of the code generation state.
    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    /// Returns the LLVM module that is being generated.
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Returns the builder that is being used to generate LLVM IR.
    pub fn builder(&mut self) -> &mut Builder<'ctx> {
        &mut self.builder
    }

    /// Returns the symbol table that is being used to store symbols.
    pub fn symbol_table(&mut self) -> &mut SymbolTable<'ctx> {
        &mut self.symbol_table
    }
}

/// A trait for types that can generate LLVM IR.
pub trait CodeGen<'ctx, T> {
    /// Generates LLVM IR for the type using the given `state`.
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> Result<T>;
}

/// A result of [code generation](CodeGen).
pub type Result<T> = std::result::Result<T, CodeGenError>;
