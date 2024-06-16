//! A module containing the symbol table implementation.

use std::collections::HashMap;
use std::rc::Rc;

use inkwell::values::AnyValueEnum;

/// A symbol table used during code generation to store symbols.
pub struct SymbolTable<'ctx> {
    symbol_stack: Vec<Scope<'ctx>>,
}

impl<'ctx> SymbolTable<'ctx> {
    /// Creates a new symbol table.
    ///
    /// The symbol table is initialized with a single scope, the global scope.
    pub fn new() -> SymbolTable<'ctx> {
        SymbolTable {
            symbol_stack: vec![Scope::new()],
        }
    }

    /// Opens a new scope and sets it as the current scope.
    pub fn open_scope(&mut self) {
        self.symbol_stack.push(Scope::new());
    }

    /// Closes the current scope.
    ///
    /// # Panics
    ///
    /// Panics if you try to pop the global scope.
    pub fn close_scope(&mut self) {
        if self.symbol_stack.len() == 1 {
            panic!("Cannot pop the global scope");
        }

        self.symbol_stack.pop();
    }

    /// Inserts a symbol with the given name and value into the current scope
    /// and returns the previous value associated with the name, if any.
    pub fn insert(&mut self, name: Rc<str>, value: AnyValueEnum<'ctx>) -> Option<Symbol<'ctx>> {
        // The stack is guaranteed to have at least one element (see `pop_scope`)
        let symbol = Symbol::new(name.clone(), value);
        self.symbol_stack.last_mut().unwrap().insert(name, symbol)
    }

    /// Iterates over the scopes, starting at the current one, and returns
    /// the first value associated with the given name, if any.
    ///
    /// Assuming that the scopes form a stack and the current scope is at the top of the stack,
    /// this method iterates over the stack from top to bottom.
    pub fn get(&self, name: &str) -> Option<&Symbol<'ctx>> {
        self.symbol_stack
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
    }

    /// Iterates over the scopes, starting at the current one, and returns `true`
    /// if any of the scopes contains the given name.
    ///
    /// Assuming that the scopes form a stack and the current scope is at the top of the stack,
    /// this method iterates over the stack from top to bottom.
    pub fn contains(&self, name: &str) -> bool {
        self.symbol_stack
            .iter()
            .rev()
            .any(|scope| scope.contains_key(name))
    }
}

impl Default for SymbolTable<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// A scope that maps symbol names to symbols.
type Scope<'ctx> = HashMap<Rc<str>, Symbol<'ctx>>;

/// A symbol declared in the program, like a function or a static.
pub struct Symbol<'ctx> {
    name: Rc<str>,
    value: AnyValueEnum<'ctx>,
}

impl<'ctx> Symbol<'ctx> {
    /// Creates a new symbol with the given name and value.
    pub fn new(name: Rc<str>, value: AnyValueEnum<'ctx>) -> Symbol<'ctx> {
        Symbol { name, value }
    }

    /// Returns the name of the symbol.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value of the symbol.
    pub fn value(&self) -> AnyValueEnum<'ctx> {
        self.value
    }
}
