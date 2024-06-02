//! A module containing the While Loop AST node implementation.

use std::{fmt, iter};

use debug_tree::TreeBuilder;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AsASTNode, AssigneeExprASTNode, BlockASTNode,
    ExprASTNode, PlaceExprASTNode, ValueExprASTNode,
};
use crate::token::Span;

/// An AST node representing a while loop expression.
#[derive(Debug)]
pub struct WhileASTNode {
    /// The condition can be [any kind of expression](ExprASTNode).
    condition: Box<dyn ExprASTNode>,
    body: Box<BlockASTNode>,
    span: Span,
}

impl WhileASTNode {
    /// Creates a new `WhileASTNode` with the given condition, body and span.
    pub fn new(
        condition: Box<dyn ExprASTNode>,
        body: Box<BlockASTNode>,
        span: Span,
    ) -> WhileASTNode {
        WhileASTNode {
            condition,
            body,
            span,
        }
    }
}

impl ASTNode for WhileASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let condition = iter::once(self.condition.as_ast());
        let body = iter::once(self.body.as_ast());
        let iter = condition.chain(body);
        Some(Box::new(iter))
    }

    fn add_to_tree_string(&self, builder: &mut TreeBuilder) {
        let condition = self.condition.as_ast();
        let body = self.body.as_ast();

        let mut branch = builder.add_branch(format!("{self}").as_str());
        {
            let mut branch = builder.add_branch("Condition");
            condition.add_to_tree_string(builder);
            branch.release()
        }
        {
            let mut branch = builder.add_branch("Body");
            body.add_to_tree_string(builder);
            branch.release()
        }
        branch.release()
    }
}

impl ExprASTNode for WhileASTNode {
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

impl ValueExprASTNode for WhileASTNode {}

impl fmt::Display for WhileASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "While {}", self.span)
    }
}
