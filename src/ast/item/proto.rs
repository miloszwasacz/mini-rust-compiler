//! A module containing the Function Prototype AST node implementation.

use std::fmt;
use std::rc::Rc;

use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};

use crate::ast::{ast_defaults, ASTChildIterator, ASTNode, AsASTNode, Type, TypeASTMetaNode};
use crate::codegen;
use crate::codegen::{CodeGen, CodeGenState};
use crate::token::Span;

pub use self::param::*;

mod param;

/// An AST node representing a function prototype.
#[derive(Debug)]
pub struct FuncProtoASTNode {
    name: Rc<str>,
    params: Vec<ParamASTNode>,
    return_type: TypeASTMetaNode,
    span: Span,
}

impl FuncProtoASTNode {
    /// Creates a new `FuncProtoASTNode` with the given name, parameters, return type and span.
    pub fn new(
        name: Rc<str>,
        params: Vec<ParamASTNode>,
        return_type: TypeASTMetaNode,
        span: Span,
    ) -> FuncProtoASTNode {
        FuncProtoASTNode {
            name,
            params,
            return_type,
            span,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a shared strong reference to the name.
    pub fn name_owned(&self) -> Rc<str> {
        self.name.clone()
    }

    /// Returns the return type meta-node.
    pub fn return_type(&self) -> &TypeASTMetaNode {
        &self.return_type
    }

    /// Returns an iterator over the parameters.
    pub fn get_param_iter(&self) -> impl Iterator<Item = &ParamASTNode> {
        self.params.iter()
    }
}

impl ASTNode for FuncProtoASTNode {
    ast_defaults!();

    fn children(&self) -> Option<ASTChildIterator> {
        let iter = self.params.iter().map(|p| p.as_ast());
        Some(Box::new(iter))
    }
}

impl<'ctx> CodeGen<'ctx, ()> for FuncProtoASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<()> {
        CodeGen::<FunctionType>::code_gen(self, state).map(|_| ())
    }
}

impl<'ctx> CodeGen<'ctx, FunctionType<'ctx>> for FuncProtoASTNode {
    fn code_gen(&self, state: &mut CodeGenState<'ctx>) -> codegen::Result<FunctionType<'ctx>> {
        let params = self
            .params
            .iter()
            .map(|p| CodeGen::<BasicMetadataTypeEnum>::code_gen(p, state))
            .collect::<codegen::Result<Vec<_>>>()?;

        Ok(match self.return_type.ty() {
            Type::Unit => state.context().void_type().fn_type(&params, false),
            _ => CodeGen::<BasicTypeEnum>::code_gen(&self.return_type, state)?
                .fn_type(&params, false),
        })
    }
}

impl fmt::Display for FuncProtoASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function Prototype: \"{}\"", self.name)
    }
}
