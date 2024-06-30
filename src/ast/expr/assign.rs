//! A module containing Assignment AST node implementation.

use std::{fmt, iter};

use debug_tree::TreeBuilder;
use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::ast::error::SemanticError;
use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, LiteralASTNode,
    PlaceExprASTNode, ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::error::CodeGenError;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing an assignment.
#[derive(Debug)]
pub struct AssignASTNode {
    /// The assignee has to be an [assignee expression](AssigneeExprASTNode).
    assignee: Box<dyn ExprASTNode>,
    /// The value has to be a [value expression](ValueExprASTNode).
    value: Box<dyn ExprASTNode>,
    span: Span,
}

impl AssignASTNode {
    /// Creates a new `AssignASTNode` with the given assignee, value and span.
    pub fn new(
        assignee: Box<dyn ExprASTNode>,
        value: Box<dyn ExprASTNode>,
        span: Span,
    ) -> AssignASTNode {
        AssignASTNode {
            assignee,
            value,
            span,
        }
    }
}

impl ASTNode for AssignASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let assignee = iter::once(self.assignee.as_ast());
        let value = iter::once(self.value.as_ast());
        let iter = assignee.chain(value);
        Some(Box::new(iter))
    }

    fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
        let assignee = self.assignee.as_ast();
        let value = self.value.as_ast();

        let mut branch = builder.add_branch(format!("{self}").as_str());
        {
            let mut branch = builder.add_branch("Assignee");
            assignee.add_to_tree_string(builder);
            branch.release()
        }
        {
            let mut branch = builder.add_branch("Value");
            value.add_to_tree_string(builder);
            branch.release()
        }
        branch.release();
    }
}

impl ExprASTNode for AssignASTNode {
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

impl ValueExprASTNode for AssignASTNode {}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for AssignASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        // Returns the result of the assignment, which is always a unit value.
        let assignment_result = |state: &mut CodeGenState<'ctx>| {
            let value = LiteralASTNode::<()>::new(self.value.span());
            CodeGen::<AnyValueEnum>::code_gen(&value, state).unwrap()
        };

        let value = CodeGen::<AnyValueEnum>::code_gen(self.value.as_ref(), state)?;
        let value =
            BasicValueEnum::try_from(value).map_err(|_| CodeGenError::InvalidLLVMValueType {
                message: "The RHS of the assignment must be a basic value".into(),
                span: self.span,
            })?;

        let assignee = self
            .assignee
            .try_as_assignee()
            .ok_or(SemanticError::WrongExpressionKind {
                message: "Expected an assignee expression",
                span: self.span,
            })?
            .pattern();

        let pat = match assignee {
            Some(pat) => pat,
            None => return Ok(assignment_result(state)),
        };
        let ptr = state.symbol_table().get(pat.as_ref()).map_or_else(
            || {
                Err(CodeGenError::MissingSymbol {
                    symbol: pat.to_string().into_boxed_str(),
                    span: self.assignee.span(),
                })
            },
            |s| match s.value() {
                AnyValueEnum::PointerValue(p) => Ok(p),
                _ => Err(CodeGenError::InvalidLLVMValueType {
                    message: "Expected a pointer value".into(),
                    span: self.assignee.span(),
                }),
            },
        )?;

        // If the value is a unit struct, we don't need to store it.
        match value {
            BasicValueEnum::StructValue(s) if s.count_fields() == 0 => {}
            _ => {
                state
                    .builder()
                    .build_store(ptr, value)
                    .map_err(CodeGenError::from)?;
            }
        }

        Ok(assignment_result(state))
    }
}

impl fmt::Display for AssignASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assignment {}", self.span)
    }
}
