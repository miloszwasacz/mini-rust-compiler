//! Defines the AST for the μRust language.

// TODO Create all the AST node types
// Declarations:
// - Crate
// - Function & function prototype
// - Static item (i.e. a global variable)
// - Extern block
//
// Statements:
// - Let statement & parameters (i.e. variable declaration)
// - Expression statement
//
// Expressions:
// - Block
// - Literal
// - Variable & underscore expression
// - Function call
// - Operator
//   - Assignment
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

pub use self::node::*;

mod node {
    use std::fmt;

    use debug_tree::TreeBuilder;

    use crate::token::Span;

    /// A type alias for an iterator over the children of an AST node.
    pub type ASTChildIterator<'a> = Box<dyn Iterator<Item = &'a dyn ASTNode> + 'a>;

    //TODO Add examples to all the doc comments.
    /// A trait defining the common interface for all AST nodes.
    pub trait ASTNode: fmt::Display {
        /// Returns the span that defines the location of this AST node.
        fn span(&self) -> Span;

        /// Returns an iterator over the children of this AST node, if any.
        fn children(&self) -> Option<ASTChildIterator>;

        /// Returns a reference to this AST node as a `dyn ASTNode`.
        fn as_ast(&self) -> &dyn ASTNode;

        /// Adds this AST node to the given tree builder.
        ///
        /// This method is used to generate a pretty representation of the AST suitable for display.
        fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
            let mut branch = builder.add_branch(format!("{self}").as_str());
            if let Some(children) = self.children() {
                for child in children {
                    child.add_to_tree_string(builder);
                }
            }
            branch.release();
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

    pub(super) use as_ast;
}
