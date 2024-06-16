//! A module containing Path AST node implementation.

use std::fmt;
use std::rc::Rc;

use inkwell::values::AnyValueEnum;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode,
    ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing a path (i.e. a variable or item).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathASTNode {
    path: Rc<str>,
    span: Span,
}

impl PathASTNode {
    /// Creates a new `PathASTNode` with the given path and span.
    pub fn new(path: Rc<str>, span: Span) -> PathASTNode {
        PathASTNode { path, span }
    }

    /// Returns the path.
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl ASTNode for PathASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        None
    }
}

impl ExprASTNode for PathASTNode {
    fn try_as_place(&self) -> Option<&dyn PlaceExprASTNode> {
        Some(self)
    }

    fn try_as_value(&self) -> Option<&dyn ValueExprASTNode> {
        Some(self)
    }

    fn try_as_assignee(&self) -> Option<&dyn AssigneeExprASTNode> {
        Some(self)
    }
}

impl PlaceExprASTNode for PathASTNode {}

impl ValueExprASTNode for PathASTNode {}

impl AssigneeExprASTNode for PathASTNode {
    fn pattern(&self) -> Option<Rc<str>> {
        Some(self.path.clone())
    }
}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for PathASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        todo!()
    }
}

impl fmt::Display for PathASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path {} \"{}\"", self.span, self.path)
    }
}
