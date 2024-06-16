//! A module containing the Function Parameter AST node implementation.

use std::{fmt, iter};

use inkwell::types::BasicMetadataTypeEnum;

use crate::ast::error::SemanticError;
use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, TypeASTMetaNode,
};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing a function parameter.
#[derive(Debug)]
pub struct ParamASTNode {
    /// The assignee has to be an [assignee expression](AssigneeExprASTNode).
    assignee: Box<dyn ExprASTNode>,
    ty: TypeASTMetaNode,
    mutable: bool,
    span: Span,
}

impl ParamASTNode {
    /// Creates a new `ParamASTNode` with the given assignee, type, mutability and span.
    pub fn new(
        assignee: Box<dyn ExprASTNode>,
        ty: TypeASTMetaNode,
        mutable: bool,
        span: Span,
    ) -> ParamASTNode {
        ParamASTNode {
            assignee,
            ty,
            mutable,
            span,
        }
    }

    /// Returns the assignee if it is an [assignee expression](AssigneeExprASTNode).
    ///
    /// # Errors
    ///
    /// If the assignee is not an _assignee expression_,
    /// the method returns [`SemanticError::WrongExpressionKind`].
    pub fn assignee(&self) -> Result<&dyn AssigneeExprASTNode, SemanticError> {
        self.assignee
            .try_as_assignee()
            .ok_or_else(|| SemanticError::WrongExpressionKind {
                message: "Expected <pattern>",
                span: self.assignee.span(),
            })
    }

    /// Returns the type.
    pub fn ty(&self) -> TypeASTMetaNode {
        self.ty
    }

    /// Returns whether the parameter is mutable.
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
}

impl ASTNode for ParamASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = iter::once(self.assignee.as_ast());
        Some(Box::new(iter))
    }
}

impl<'ctx> CodeGen<'ctx, ()> for ParamASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<()> {
        CodeGen::<BasicMetadataTypeEnum>::code_gen(self, state).map(|_| ())
    }
}

impl<'ctx> CodeGen<'ctx, BasicMetadataTypeEnum<'ctx>> for ParamASTNode {
    fn code_gen(
        &self,
        state: &mut CodeGenState<'ctx>,
    ) -> codegen::Result<BasicMetadataTypeEnum<'ctx>> {
        CodeGen::<BasicMetadataTypeEnum>::code_gen(&self.ty, state)
    }
}

impl fmt::Display for ParamASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mutability = if self.is_mutable() { "Mut" } else { "" };
        write!(f, "Param {} {}", mutability, self.span)
    }
}
