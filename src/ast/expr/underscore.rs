//! A module containing Underscore AST node implementation.

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

/// An AST node representing an underscore.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UnderscoreASTNode {
    span: Span,
}

impl UnderscoreASTNode {
    /// Creates a new `UnderscoreASTNode` with the given span.
    pub fn new(span: Span) -> UnderscoreASTNode {
        UnderscoreASTNode { span }
    }
}

impl ASTNode for UnderscoreASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        None
    }
}

impl ExprASTNode for UnderscoreASTNode {
    fn try_as_place(&self) -> Option<&dyn PlaceExprASTNode> {
        None
    }

    fn try_as_value(&self) -> Option<&dyn ValueExprASTNode> {
        None
    }

    fn try_as_assignee(&self) -> Option<&dyn AssigneeExprASTNode> {
        Some(self)
    }
}

impl AssigneeExprASTNode for UnderscoreASTNode {
    fn pattern(&self) -> Option<Rc<str>> {
        None
    }
}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for UnderscoreASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        todo!()
    }
}

impl fmt::Display for UnderscoreASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Underscore {}", self.span)
    }
}
