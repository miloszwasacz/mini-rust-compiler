//! A module containing the Block AST node implementation.

use std::fmt;

use inkwell::values::AnyValueEnum;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode,
    StatementASTNode, ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// A type alias for a list of statements.
pub type Statements = Vec<Box<dyn StatementASTNode>>;

/// A type alias for the last expression in a block, if present (instead of a statement).
pub type BlockReturnExpr = Option<Box<dyn ExprASTNode>>;

/// An AST node representing a block expression.
#[derive(Debug)]
pub struct BlockASTNode {
    statements: Statements,
    return_expr: BlockReturnExpr,
    span: Span,
}

impl BlockASTNode {
    /// Creates a new `BlockASTNode` with the given statements and span.
    pub fn new(statements: Vec<Box<dyn StatementASTNode>>, span: Span) -> BlockASTNode {
        BlockASTNode {
            statements,
            return_expr: None,
            span,
        }
    }

    /// Creates a new `BlockASTNode` with the given statements, return expression, and span.
    pub fn new_with_return(
        statements: Vec<Box<dyn StatementASTNode>>,
        return_expr: Box<dyn ExprASTNode>,
        span: Span,
    ) -> BlockASTNode {
        BlockASTNode {
            statements,
            return_expr: Some(return_expr),
            span,
        }
    }
}

impl ASTNode for BlockASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let statements = self.statements.iter().map(|s| s.as_ast());
        let return_expr = self.return_expr.iter().map(|e| e.as_ast());
        let iter = statements.chain(return_expr);
        Some(Box::new(iter))
    }
}

impl ExprASTNode for BlockASTNode {
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

impl ValueExprASTNode for BlockASTNode {}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for BlockASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        state.symbol_table().open_scope();

        for statement in &self.statements {
            statement.code_gen(state).map_err(|e| {
                state.symbol_table().close_scope();
                e
            })?;
        }

        let ret_value = match &self.return_expr {
            Some(expr) => CodeGen::<AnyValueEnum>::code_gen(expr.as_ref(), state),
            None => Ok(state.build_unit_value(self.span.end())),
        };

        state.symbol_table().close_scope();

        ret_value
    }
}

impl fmt::Display for BlockASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block {}", self.span)
    }
}
