//! A module containing Function Call AST node implementation.

use std::fmt;

use inkwell::values::AnyValueEnum;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PathASTNode,
    PlaceExprASTNode, ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing a function call.
#[derive(Debug)]
pub struct FunCallASTNode {
    path: Box<PathASTNode>,
    /// The arguments can be [any kind of expressions](ExprASTNode).
    args: Vec<Box<dyn ExprASTNode>>,
    span: Span,
}

impl FunCallASTNode {
    /// Creates a new `FunCallASTNode` with the given path, arguments and span.
    pub fn new(
        path: Box<PathASTNode>,
        args: Vec<Box<dyn ExprASTNode>>,
        span: Span,
    ) -> FunCallASTNode {
        FunCallASTNode { path, args, span }
    }

    /// Returns the path to the called function.
    pub fn path(&self) -> &str {
        self.path.path()
    }
}

impl ASTNode for FunCallASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = self.args.iter().map(|arg| arg.as_ast());
        Some(Box::new(iter))
    }
}

impl ExprASTNode for FunCallASTNode {
    fn try_as_place(&self) -> Option<&dyn PlaceExprASTNode> {
        None
    }

    fn try_as_value(&self) -> Option<&dyn ValueExprASTNode> {
        Some(self)
    }

    fn try_as_assignee(&self) -> Option<&dyn AssigneeExprASTNode> {
        None
    }
}

impl ValueExprASTNode for FunCallASTNode {}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for FunCallASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        todo!()
    }
}

impl fmt::Display for FunCallASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function Call {} \"{}\"", self.span, self.path())
    }
}
