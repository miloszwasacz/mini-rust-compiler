//! A module containing If AST node implementation.

use std::{fmt, iter};

use debug_tree::TreeBuilder;
use inkwell::values::AnyValueEnum;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, AssigneeExprASTNode, BlockASTNode,
    ExprASTNode, PlaceExprASTNode, ValueExprASTNode,
};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing an if expression.
#[derive(Debug)]
pub struct IfASTNode {
    /// The condition can be [any kind of expression](ExprASTNode).
    condition: Box<dyn ExprASTNode>,
    then_block: Box<BlockASTNode>,
    else_node: ElseExpr,
    span: Span,
}

/// An enum representing the possible else expressions in an if statement.
#[derive(Debug)]
pub enum ElseExpr {
    /// No else block.
    None,
    /// An else block.
    Else(Box<BlockASTNode>),
    /// An else if block.
    ElseIf(Box<IfASTNode>),
}

impl IfASTNode {
    /// Creates a new `IfASTNode` with the given condition, then block, else expression and span.
    pub fn new(
        condition: Box<dyn ExprASTNode>,
        then_block: Box<BlockASTNode>,
        else_node: ElseExpr,
        span: Span,
    ) -> IfASTNode {
        IfASTNode {
            condition,
            then_block,
            else_node,
            span,
        }
    }

    /// Collects all the else if nodes and the final else block (if any) in a flat vector.
    fn flatten_else_ifs(&self) -> (Vec<&IfASTNode>, Option<&BlockASTNode>) {
        let mut nodes = Vec::new();
        let mut current = self;
        while let ElseExpr::ElseIf(if_node) = &current.else_node {
            nodes.push(if_node.as_ref());
            current = if_node;
        }
        let else_block = match &current.else_node {
            ElseExpr::None => None,
            ElseExpr::Else(block) => Some(block.as_ref()),
            ElseExpr::ElseIf(_) => unreachable!(),
        };

        (nodes, else_block)
    }

    /// Custom implementation for [`ASTNode::add_to_tree_string`] that properly
    /// handles the else if case.
    ///
    /// In the case of an else if, the tree is flattened so that all the else ifs
    /// and the final else (if any) are at the same level.
    fn add_to_tree_string(&self, builder: &mut TreeBuilder, is_else_if: bool) {
        // We add a new branch only if this is the main if node, not a nested else if
        let branch = if !is_else_if {
            Some(builder.add_branch(format!("{self}").as_str()))
        } else {
            None
        };

        // Condition
        {
            let condition = self.condition.as_ast();
            let mut branch = builder.add_branch("Condition");
            condition.add_to_tree_string(builder);
            branch.release();
        }

        // Then block
        {
            let then_block = self.then_block.as_ast();
            let mut branch = builder.add_branch("Then");
            then_block.add_to_tree_string(builder);
            branch.release();
        }

        // The branch only exists if this is the main if node, not a nested else if
        if let Some(mut branch) = branch {
            let (else_ifs, else_block) = self.flatten_else_ifs();

            // Else if blocks
            for else_if in else_ifs {
                let mut branch = builder.add_branch(format!("Else {else_if}").as_str());
                IfASTNode::add_to_tree_string(else_if, builder, true);
                branch.release();
            }

            // Else block
            if let Some(else_block) = else_block {
                let mut branch = builder.add_branch("Else");
                else_block.add_to_tree_string(builder);
                branch.release();
            }

            branch.release();
        }
    }
}

impl ASTNode for IfASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let condition = iter::once(self.condition.as_ast());
        let then_block = iter::once(self.then_block.as_ast());
        let iter = condition.chain(then_block);
        Some(match &self.else_node {
            ElseExpr::None => Box::new(iter),
            ElseExpr::Else(block) => {
                let else_block = iter::once(block.as_ast());
                let iter = iter.chain(else_block);
                Box::new(iter)
            }
            ElseExpr::ElseIf(if_node) => {
                let else_if = iter::once(if_node.as_ast());
                let iter = iter.chain(else_if);
                Box::new(iter)
            }
        })
    }

    fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
        IfASTNode::add_to_tree_string(self, builder, false);
    }
}

impl ExprASTNode for IfASTNode {
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

impl ValueExprASTNode for IfASTNode {}

impl<'ctx> CodeGen<'ctx, AnyValueEnum<'ctx>> for IfASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<AnyValueEnum<'ctx>> {
        todo!()
    }
}

impl fmt::Display for IfASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "If {}", self.span)
    }
}
