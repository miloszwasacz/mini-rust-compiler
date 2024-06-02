//! A module containing Assignment AST node implementation.

use std::{fmt, iter};

use debug_tree::TreeBuilder;

use crate::ast::{
    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode,
    ValueExprASTNode,
};
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

impl fmt::Display for AssignASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assignment {}", self.span)
    }
}
