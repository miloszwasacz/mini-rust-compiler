//! A module containing all the operator expression AST nodes.

pub use self::arith::*;
pub use self::comp::*;
pub use self::lazy_bool::*;
use self::macros::*;
pub use self::neg::*;

mod arith;
mod comp;
mod lazy_bool;
mod neg;

/// A trait for all binary operators.
trait BinOperator {
    /// Returns a string representation of the operator.
    fn as_str(&self) -> &'static str;
}

/// Macros for defining binary operator AST nodes.
mod macros {
    macro_rules! bin_op_ast_node {
        (
            $( #[$doc:meta] )*
            $name:ident {
                operator: $operator:ty,
                label: $label:expr $(,)?
            }
        ) => {
            pub use self::__bin_op_ast_node_impl::*;

            mod __bin_op_ast_node_impl {
                use super::*;
                use std::{fmt, iter};

                use crate::ast::{
                    ast_defaults, ASTChildIterator, ASTNode, AssigneeExprASTNode, ExprASTNode, PlaceExprASTNode,
                    ValueExprASTNode,
                };
                use crate::token::Span;

                #[derive(Debug)]
                $( #[$doc] )*
                pub struct $name {
                    pub(super) operator: $operator,
                    #[doc = "The left-hand side of the operator can be [any kind of expression](ExprASTNode)."]
                    pub(super) lhs: Box<dyn ExprASTNode>,
                    #[doc = "The right-hand side of the operator can be [any kind of expression](ExprASTNode)."]
                    pub(super) rhs: Box<dyn ExprASTNode>,
                    span: Span,
                }

                impl $name {
                    #[doc = concat!("Creates a new [`", stringify!($name), "`] with the given operator, lhs, rhs and span.")]
                    pub fn new(
                        operator: $operator,
                        lhs: Box<dyn ExprASTNode>,
                        rhs: Box<dyn ExprASTNode>,
                        span: Span,
                    ) -> $name {
                        $name {
                            operator,
                            lhs,
                            rhs,
                            span,
                        }
                    }
                }

                impl ASTNode for $name {
                    ast_defaults!();

                    fn children(&self) -> Option<ASTChildIterator> {
                        let lhs = iter::once(self.lhs.as_ast());
                        let rhs = iter::once(self.rhs.as_ast());
                        let iter = lhs.chain(rhs);
                        Some(Box::new(iter))
                    }
                }

                impl ExprASTNode for $name {
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

                impl ValueExprASTNode for $name {}

                impl fmt::Display for $name
                where
                    $operator: BinOperator,
                {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        write!(
                            f,
                            "Operator ({}) {} `{}`",
                            $label,
                            self.span,
                            self.operator.as_str(),
                        )
                    }
                }
            }
        };
    }

    macro_rules! operator_display {
        ($op:ty) => {
            mod __operator_display_impl {
                use super::*;
                use std::fmt;

                impl fmt::Display for $op {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        write!(f, "{}", self.as_str())
                    }
                }
            }
        };
    }

    pub(super) use bin_op_ast_node;
    pub(super) use operator_display;
}
