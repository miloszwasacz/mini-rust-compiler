//! A module containing the types and traits used for code generation.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicTypeEnum, FunctionType};
use inkwell::values::{AnyValue, AnyValueEnum, FunctionValue, IntValue};
use inkwell::IntPredicate;

use crate::ast::{
    CrateASTNode, ExprASTNode, ExternItem, FuncProtoASTNode, ItemASTNode, StaticASTNode, Type,
};

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

    /// Extracts the LLVM module that is being generated.
    pub fn take_module(self) -> Module<'ctx> {
        self.module
    }

    /// Returns the builder that is being used to generate LLVM IR.
    pub fn builder(&mut self) -> &mut Builder<'ctx> {
        &mut self.builder
    }

    /// Returns the symbol table that is being used to store symbols.
    pub fn symbol_table(&mut self) -> &mut SymbolTable<'ctx> {
        &mut self.symbol_table
    }

    /// Returns the parent function of the basic block currently being used by the builder, if any.
    pub fn get_current_function(&mut self) -> Option<FunctionValue<'ctx>> {
        self.builder()
            .get_insert_block()
            .and_then(|bb| bb.get_parent())
    }

    /// Runs [`CodeGen::<AnyValueEnum>::code_gen`] on the given `expr`,
    /// checks if the result is a boolean value, and returns it.
    ///
    /// # Errors
    ///
    /// If the result is not a boolean value, [`CodeGenError::TypeMismatch`] is returned.
    pub fn build_bool(&mut self, expr: &dyn ExprASTNode) -> Result<IntValue<'ctx>> {
        CodeGen::<AnyValueEnum>::code_gen(expr, self).and_then(|value| match value {
            AnyValueEnum::IntValue(i) if i.get_type().get_bit_width() == 1 => Ok(i),
            value => {
                let span = expr.span();
                let ty = Type::try_from_llvm_value(self.context(), value, span)?;
                Err(CodeGenError::TypeMismatch {
                    expected: Type::Bool,
                    actual: ty,
                    span,
                })
            }
        })
    }

    /// Generates LLVM IR for the given `cond` expression and a comparison with the `expected_result`.
    /// The comparison is then returned as an integer value.
    ///
    /// The `cond` expression must be a boolean expression, see [`build_bool`](CodeGenState::build_bool).
    pub fn build_condition(
        &mut self,
        cond: &dyn ExprASTNode,
        expected_result: bool,
    ) -> Result<IntValue<'ctx>> {
        let cond = self.build_bool(cond)?;
        let pred = if expected_result {
            IntPredicate::NE
        } else {
            IntPredicate::EQ
        };

        let const_zero = self.context().bool_type().const_zero();
        self.builder()
            .build_int_compare(pred, cond, const_zero, "cond")
            .map_err(CodeGenError::from)
    }
}

/// A trait for types that can generate LLVM IR.
pub trait CodeGen<'ctx, T> {
    /// Generates LLVM IR for the type using the given `state`.
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> Result<T>;
}

/// A result of [code generation](CodeGen).
pub type Result<T> = std::result::Result<T, CodeGenError>;

impl CrateASTNode {
    /// Populates the symbol table with the symbols that are declared in the crate.
    ///
    /// It collects the global symbol declarations (functions, statics, externals, etc.)
    /// and adds them to the [`module`](CodeGenState::module) that is being generated.
    ///
    /// __This method should be called before [generating the LLVM IR](CodeGen::code_gen) for the crate!__
    pub fn collect_symbols(&self, state: &mut CodeGenState) -> Result<()> {
        for item in self.items() {
            item.add_to_symbol_table(state)?;
        }
        Ok(())
    }
}

impl ItemASTNode {
    /// Adds the item declaration to the symbol table.
    fn add_to_symbol_table(&self, state: &mut CodeGenState) -> Result<()> {
        match &self {
            ItemASTNode::Func(func) => func.proto().add_to_symbol_table(state)?,
            ItemASTNode::Static(stat) => stat.add_to_symbol_table(state, false)?,
            ItemASTNode::Extern(ext) => {
                for item in ext.items() {
                    match item {
                        ExternItem::Func(func) => func.add_to_symbol_table(state)?,
                        ExternItem::Static(stat) => stat.add_to_symbol_table(state, true)?,
                    }
                }
            }
        };
        Ok(())
    }
}

impl FuncProtoASTNode {
    /// Adds the function prototype to the symbol table.
    fn add_to_symbol_table(&self, state: &mut CodeGenState) -> Result<()> {
        let name = self.name_owned();

        let fn_type = CodeGen::<FunctionType>::code_gen(self, state)?;
        let fn_value = state.module().add_function(&name, fn_type, None);
        //TODO Handle redeclarations
        state
            .symbol_table()
            .insert(name, fn_value.as_any_value_enum());

        Ok(())
    }
}

impl StaticASTNode {
    /// Adds the static item declaration to the symbol table.
    fn add_to_symbol_table(&self, state: &mut CodeGenState, is_external: bool) -> Result<()> {
        let name = self.name_owned();
        let linkage = if is_external {
            Linkage::External
        } else {
            Linkage::Internal
        };

        let ty = CodeGen::<BasicTypeEnum>::code_gen(&self.ty(), state)?;
        let stat = state.module().add_global(ty, None, &name);
        stat.set_linkage(linkage);

        //TODO Handle redeclarations
        state.symbol_table().insert(name, stat.as_any_value_enum());
        Ok(())
    }
}
