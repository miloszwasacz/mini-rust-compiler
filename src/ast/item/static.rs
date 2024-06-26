//! A module containing the Static Item AST node implementation.

use std::fmt;
use std::rc::Rc;

use crate::ast::{ast_defaults, ASTChildIterator, ASTNode, ExprASTNode, TypeASTMetaNode};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

/// An AST node representing a static item.
#[derive(Debug)]
pub struct StaticASTNode {
    name: Rc<str>,
    /// The value has to be a [value expression](crate::ast::ValueExprASTNode).
    value: Option<Box<dyn ExprASTNode>>,
    ty: TypeASTMetaNode,
    mutable: bool,
    span: Span,
}

impl StaticASTNode {
    /// Creates a new `StaticASTNode` with the given name, type, mutability and span.
    pub fn new(name: Rc<str>, ty: TypeASTMetaNode, mutable: bool, span: Span) -> StaticASTNode {
        StaticASTNode {
            name,
            value: None,
            ty,
            mutable,
            span,
        }
    }

    /// Creates a new `StaticASTNode` with the given name, assigned value, type, mutability and span.
    pub fn new_with_assignment(
        name: Rc<str>,
        value: Box<dyn ExprASTNode>,
        ty: TypeASTMetaNode,
        mutable: bool,
        span: Span,
    ) -> StaticASTNode {
        StaticASTNode {
            name,
            value: Some(value),
            ty,
            mutable,
            span,
        }
    }

    /// Returns the type.
    pub fn ty(&self) -> TypeASTMetaNode {
        self.ty
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a shared strong reference to the name.
    pub fn name_owned(&self) -> Rc<str> {
        self.name.clone()
    }

    /// Returns whether the item is mutable.
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    /// Returns whether the item is initialized,
    /// i.e. was created with [StaticASTNode::new_with_assignment].  
    pub fn is_initialized(&self) -> bool {
        self.value.is_some()
    }
}

impl ASTNode for StaticASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = self.value.iter().map(|v| v.as_ast());
        Some(Box::new(iter))
    }
}

impl<'ctx> CodeGen<'ctx, ()> for StaticASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<()> {
        // Here we only generate the initializer, the declaration is done during Collection Phase.
        todo!()
        //TODO Add support for more complicated initializers (evaluated at compile time)
    }
}

impl fmt::Display for StaticASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mutability = if self.mutable { "Mut" } else { "" };
        write!(f, "Static {} {} \"{}\"", mutability, self.span, self.name())
    }
}
