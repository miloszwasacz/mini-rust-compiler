//! Defines the AST for the μRust language.

// TODO Create all the AST node types
// Declarations:
// - [ ] Crate
// - [ ] Function & function prototype
// - [ ] Static item (i.e. a global variable)
// - [ ] Extern block
//
// Statements:
// - [ ] Let statement
// - [x] Function parameter
// - [ ] Expression statement
//
// Expressions:
// - [x] Block
// - [x] Literal
// - [x] Variable & underscore expression
// - [x] Function call
// - [x] Assignment
// - [x] Operator
//   - [x] Arithmetic or logical
//   - [x] Comparison
//   - [x] Negation
//   - [x] Lazy boolean
// - [x] Type cast
// - [x] Loop
//   - [x] Infinite loop
//   - [x] While loop
// - [x] If
// - [x] Unsafe block
// - [x] Return

pub use self::expr::*;
pub use self::node::*;
pub use self::stmt::*;
pub use self::types::*;

mod expr;
mod stmt;
mod types;

mod node {
    use std::fmt;

    use debug_tree::TreeBuilder;

    use crate::token::Span;

    /// A type alias for an iterator over the children of an AST node.
    pub type ASTChildIterator<'a> = Box<dyn Iterator<Item = &'a dyn ASTNode> + 'a>;

    //TODO Add examples to all the doc comments.
    /// A trait defining the common interface for all AST nodes.
    pub trait ASTNode: fmt::Debug + fmt::Display {
        /// Returns the span that defines the location of this AST node.
        fn span(&self) -> Span;

        /// Returns an iterator over the children of this AST node, if any.
        fn children(&self) -> Option<ASTChildIterator>;

        /// Returns a reference to this AST node as a `dyn ASTNode`.
        fn as_ast(&self) -> &dyn ASTNode;

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

    /// A macro that can be used as a shorthand for implementing [ASTNode::as_ast].
    ///
    /// # Example
    /// ```ignore
    /// use debug_tree::TreeBuilder;
    /// use crate::ast::{ASTNode, ASTChildIterator};
    /// use crate::ast::as_ast;
    /// use crate::token::Span;
    ///
    /// struct MyNode {
    ///     // ...
    /// }
    ///     
    /// impl ASTNode for MyNode {
    ///     fn span(&self) -> Span {
    ///         // Your implementation for `span`.
    /// #       unimplemented!()
    ///     }
    ///
    ///     fn children(&self) -> Option<ASTChildIterator> {
    ///         // Your implementation for `children`.
    /// #       unimplemented!()
    ///     }
    ///
    ///     as_ast!();
    ///     
    ///     fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
    ///         // Your implementation for `add_to_tree_string`
    ///         // (or just use the trait's base implementation).
    /// #       unimplemented!()
    ///     }
    /// }
    /// ```
    macro_rules! as_ast {
        () => {
            fn as_ast(&self) -> &dyn ASTNode {
                self
            }
        };
    }

    /// A macro that can be used as a shorthand for implementing [ASTNode::span] and [ASTNode::as_ast].
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

            as_ast!();
        };
    }

    pub(super) use as_ast;
    pub(super) use ast_defaults;
}
