//! Defines the AST for the μRust language.

// Available AST nodes:
//
// - Crate
//
// Items:
// - Function (+ function prototype)
// - Static item (i.e. a global variable)
// - Extern block
//
// Statements:
// - Let statement
// - Function parameter
// - Expression statement
//
// Expressions:
// - Block
// - Literal
// - Variable & underscore expression
// - Function call
// - Assignment
// - Operator
//   - Arithmetic or logical
//   - Comparison
//   - Negation
//   - Lazy boolean
// - Type cast
// - Loop
//   - Infinite loop
//   - While loop
// - If
// - Unsafe block
// - Return

use std::fmt;
use std::fmt::Debug;

use debug_tree::{TreeBuilder, TreeConfig, TreeSymbols};
use inkwell::context::Context;
use inkwell::module::Module;

use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};

pub use self::crt::*;
pub use self::expr::*;
pub use self::item::*;
use self::node::*;
pub use self::node::{ASTChildIterator, ASTNode, AsASTNode};
pub use self::r#type::*;
pub use self::stmt::*;

mod crt;
pub mod error;
mod expr;
mod item;
mod stmt;
mod r#type;

/// The AST for the μRust language.
#[derive(Debug)]
pub struct Crate {
    root: Box<CrateASTNode>,
}

impl Crate {
    /// Creates a new `Crate` with the given root node.
    pub fn new(root: Box<CrateASTNode>) -> Crate {
        Crate { root }
    }

    /// Generates the LLVM IR for this crate given the context.
    pub fn code_gen<'ctx>(&self, context: &'ctx Context) -> codegen::Result<Module<'ctx>> {
        let module_name = self.root.name();
        let mut state = CodeGenState::new(context, module_name);

        self.root.collect_symbols(&mut state)?;
        self.root.code_gen(&mut state)?;

        let module = state.take_module();
        module.verify().map_err(|mess| todo!("Handle errors"))?;

        Ok(module)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = TreeBuilder::new();
        builder.set_config_override(TreeConfig::new().symbols(TreeSymbols::with_rounded()));

        self.root.add_to_tree_string(&mut builder);

        write!(f, "{}", builder.string())
    }
}

/// A submodule containing the common AST node interface and utilities.
mod node {
    use std::fmt;

    use debug_tree::TreeBuilder;

    use crate::codegen::CodeGen;
    use crate::token::Span;

    /// A type alias for an iterator over the children of an AST node.
    pub type ASTChildIterator<'a> = Box<dyn Iterator<Item = &'a dyn ASTNode> + 'a>;

    //TODO Add examples to all the doc comments.
    /// A trait defining the common interface for all AST nodes.
    pub trait ASTNode: AsASTNode + for<'ctx> CodeGen<'ctx, ()> + fmt::Debug + fmt::Display {
        /// Returns the span that defines the location of this AST node.
        fn span(&self) -> Span;

        /// Returns an iterator over the children of this AST node, if any.
        fn children(&self) -> Option<ASTChildIterator>;

        /// Adds this AST node to the given tree builder.
        ///
        /// This method is used to generate a pretty representation of the AST suitable for display.
        /// Usually, the default implementation of this method is sufficient -- make a branch if the node
        /// has [children][^note], otherwise make a leaf -- but it can be overridden if necessary.
        ///
        /// [children]: ASTNode::children
        ///
        /// [^note]: The returned iterator is [Some].
        fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
            match self.children() {
                Some(children) => {
                    let mut branch = builder.add_branch(format!("{self}").as_str());
                    for child in children {
                        child.add_to_tree_string(builder);
                    }
                    branch.release();
                }
                None => {
                    builder.add_leaf(format!("{self}").as_str());
                }
            }
        }
    }

    /// An auto-trait for converting a type into a reference to a `dyn ASTNode`.
    ///
    /// It is automatically implemented for all types that implement [`ASTNode`].
    pub trait AsASTNode {
        /// Returns a reference to this AST node as a `dyn ASTNode`.
        fn as_ast(&self) -> &dyn ASTNode;
    }

    impl<T: ASTNode> AsASTNode for T {
        fn as_ast(&self) -> &dyn ASTNode {
            self
        }
    }

    /// A macro that can be used as a shorthand for implementing [`ASTNode::span`].
    ///
    /// # Example
    /// ```ignore
    /// use debug_tree::TreeBuilder;
    /// use crate::ast::{as_ast, ast_defaults, ASTNode, ASTChildIterator};
    /// use crate::token::Span;
    ///
    /// struct MyNode {
    ///     // ...
    /// }
    ///     
    /// impl ASTNode for MyNode {
    ///     ast_defaults!();
    ///
    ///     fn children(&self) -> Option<ASTChildIterator> {
    ///         // Your implementation for `children`.
    ///     }
    ///
    ///     fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
    ///         // Your implementation for `add_to_tree_string`
    ///         // (or just use the trait's base implementation).
    ///     }
    /// }
    /// ```
    macro_rules! ast_defaults {
        () => {
            fn span(&self) -> Span {
                self.span
            }
        };
    }

    pub(super) use ast_defaults;
}
