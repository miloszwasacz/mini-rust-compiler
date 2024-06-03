//! A crate containing all the individual components of the μRust compiler.
#![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)] //TODO Enable this

pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod token;
