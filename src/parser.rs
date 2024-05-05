//! The module containing the parser for the μRust compiler.

use std::io;
use std::path::Path;
use std::rc::Rc;

use fallible_iterator::{FallibleIterator, Peekable};

use crate::ast::Crate;
use crate::lexer::Lexer;

use self::error::*;

pub mod error;
mod productions;

/// A result of a parsing operation.
pub type Result<T> = std::result::Result<T, ParserError>;

//TODO Improve documentation
/// The parser for the μRust compiler.
pub struct Parser {
    lexer: Peekable<Lexer>,
    filename: Rc<str>,
}

impl Parser {
    /// Creates a new `Parser` that will parse the file at the given path.
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Parser> {
        let lexer = Lexer::new(path)?;
        let filename = lexer.get_filename_owned();
        Ok(Parser {
            lexer: lexer.peekable(),
            filename,
        })
    }

    /// Returns the name of the file being parsed.
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Parses the input file and returns `ast::Crate`
    pub fn parse(self) -> Result<Crate> {
        let root = self.parse_crate()?;
        let root = Box::new(root);
        Ok(Crate::new(root))
    }
}
