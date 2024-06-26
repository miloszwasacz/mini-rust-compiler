//! A module containing all the expression-related AST nodes.

use std::rc::Rc;

use inkwell::values::AnyValueEnum;

use codegen::CodeGenState;

use crate::ast::ASTNode;
use crate::codegen;
use crate::codegen::CodeGen;

pub use self::assign::*;
pub use self::block::*;
pub use self::cast::*;
pub use self::fun_call::*;
pub use self::grouped::*;
pub use self::literal::*;
pub use self::operator::*;
pub use self::path::*;
pub use self::r#if::*;
pub use self::r#loop::*;
pub use self::r#return::*;
pub use self::underscore::*;
pub use self::unsafe_block::*;

mod assign;
mod block;
mod cast;
mod fun_call;
mod grouped;
mod r#if;
mod literal;
mod r#loop;
mod operator;
mod path;
mod r#return;
mod underscore;
mod unsafe_block;

/// A trait for all expression-related AST nodes.
///
/// It is very important to implement the conversion methods correctly -- e.g. if the type
/// implements [`PlaceExprASTNode`], it should return `Some(self)` in the `try_as_place` method.
///
/// # Example
/// ```
/// # use std::fmt;
/// # use std::rc::Rc;
/// # use mini_rust_compiler_components::token::Span;
/// # use crate::mini_rust_compiler_components::ast::{
/// #     ASTNode, ASTChildIterator, ExprASTNode, PlaceExprASTNode, ValueExprASTNode,
/// #     AssigneeExprASTNode
/// # };
/// # use mini_rust_compiler_components::codegen;
/// # use inkwell::values::AnyValueEnum;
///
/// # #[derive(Debug)]
/// struct MyExprASTNode;
///
/// impl ExprASTNode for MyExprASTNode {
///     fn try_as_place(&self) -> Option<&dyn PlaceExprASTNode> {
///         Some(self)
///     }
///
///     fn try_as_value(&self) -> Option<&dyn ValueExprASTNode> {
///         None
///     }
///
///     fn try_as_assignee(&self) -> Option<&dyn AssigneeExprASTNode> {
///         // `AssigneeExprASTNode` is a super-trait of `PlaceExprASTNode`
///         // so it is also implemented
///         Some(self)
///     }
/// }
///
/// impl PlaceExprASTNode for MyExprASTNode {}
///
/// impl AssigneeExprASTNode for MyExprASTNode {
///     // ...
/// # fn pattern(&self) -> Option<Rc<str>> { None }
/// }
///
/// # impl ASTNode for MyExprASTNode {
/// #     fn span(&self) -> Span { unimplemented!() }
/// #     fn children(&self) -> Option<ASTChildIterator> { unimplemented!() }
/// # }
/// # impl<'ctx> codegen::CodeGen<'ctx, AnyValueEnum<'ctx>> for MyExprASTNode {
/// #     fn code_gen<'a>(&self, state: &mut codegen::CodeGenState<'a>) -> codegen::Result<AnyValueEnum<'ctx>> {
/// #         unimplemented!()
/// #     }
/// # }
/// # impl fmt::Display for MyExprASTNode {
/// #    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { unimplemented!() }
/// # }
///
/// # fn main() {
/// let my_expr = MyExprASTNode;
///
/// let place = my_expr.try_as_place();
/// assert!(place.is_some());
/// assert!(std::ptr::eq(place.unwrap(), &my_expr));
///
/// let value = my_expr.try_as_value();
/// assert!(value.is_none());
///
/// let assignee = my_expr.try_as_assignee();
/// assert!(assignee.is_some());
/// assert!(std::ptr::eq(assignee.unwrap(), &my_expr));
/// # }
/// ```
pub trait ExprASTNode:
    ASTNode + for<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> + AsExprASTNode
{
    /// Tries to convert the expression to a [`PlaceExprASTNode`].
    fn try_as_place(&self) -> Option<&dyn PlaceExprASTNode>;

    /// Tries to convert the expression to a [`ValueExprASTNode`].
    fn try_as_value(&self) -> Option<&dyn ValueExprASTNode>;

    /// Tries to convert the expression to an [`AssigneeExprASTNode`].
    fn try_as_assignee(&self) -> Option<&dyn AssigneeExprASTNode>;
}

/// A trait for all [place expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
/// AST nodes.
///
/// It is very important to implement the conversion methods for [`ExprASTNode`] correctly -- if
/// a type implements this trait, it should return `Some(self)` in the `try_as_place` method.
pub trait PlaceExprASTNode: AssigneeExprASTNode {}

/// A trait for all [value expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
/// AST nodes.
///
/// It is very important to implement the conversion methods for [`ExprASTNode`] correctly -- if
/// a type implements this trait, it should return `Some(self)` in the `try_as_value` method.
pub trait ValueExprASTNode: ExprASTNode {}

/// A trait for all [assignee expression](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
/// AST nodes.
///
/// It is very important to implement the conversion methods for [`ExprASTNode`] correctly -- if
/// a type implements this trait, it should return `Some(self)` in the `try_as_assignee` method.
pub trait AssigneeExprASTNode: ExprASTNode {
    //TODO Update the documentation when Patterns are implemented
    /// Returns the name of the assigned variable.
    fn pattern(&self) -> Option<Rc<str>>;
}

/// An auto-trait for converting a type to a reference to a [general expression](ExprASTNode) AST node.
///
/// It is automatically implemented for all types that implement [`ExprASTNode`].
pub trait AsExprASTNode {
    /// Returns the type as a reference to a `dyn ExprASTNode`.
    fn as_expr(&self) -> &dyn ExprASTNode;
}

impl<T: ExprASTNode> AsExprASTNode for T {
    fn as_expr(&self) -> &dyn ExprASTNode {
        self
    }
}

impl<'ctx, T: ExprASTNode> CodeGen<'ctx, ()> for T {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<()> {
        CodeGen::<AnyValueEnum>::code_gen(self, state).map(|_| ())
    }
}
