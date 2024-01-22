//! The module containing the lexer for the μRust compiler.

use std::ffi::OsStr;
use std::io;
use std::iter::Peekable;
use std::path::Path;

use unicode_ident::{is_xid_continue, is_xid_start};

use crate::lexer::error::{LResult, LexerError, LexerErrorKind};
use crate::lexer::file_reader::{FileReader, FileReaderIter};
use crate::lexer::helper::{is_digit, is_new_line, is_whitespace};
use crate::token::{Position, Span, Token, TokenType};

mod error;
mod file_reader;
mod helper;

/// The lexer for the μRust compiler.
///
/// # Examples
///
/// The Lexer should be used as an iterator over `LResult<Token>`.
/// ```no_run
/// # use mini_rust_compiler_components::lexer::Lexer;
///
/// let mut lexer = Lexer::new("path/to/file").unwrap();
/// for result in lexer {
///    match result {  
///         Ok(token) => {
///            // Do something with the token
///         }
///         Err(err) => {
///           // Handle the error
///         }
///    }
/// }
/// ```
pub struct Lexer {
    file_name: String,
    position: Position,
    iter: Peekable<FileReaderIter>,
}

impl Lexer {
    //TODO Add tests
    /// Creates a new `Lexer` that will lex the file at the given path.
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Lexer> {
        let file_name = Lexer::get_file_name_from_path(&path)?;
        let iter = FileReader::new(path).try_iter()?.peekable();
        Ok(Lexer {
            file_name,
            position: Position::new(),
            iter,
        })
    }

    //TODO Add tests
    /// Extracts the file name from the given path.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` with `io::ErrorKind::InvalidInput` if the given
    /// path is invalid (ends with `..` or is not valid UTF-8).
    fn get_file_name_from_path<P: AsRef<Path>>(path: &P) -> io::Result<String> {
        let file_name = path.as_ref().file_name().and_then(OsStr::to_str);
        match file_name {
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid file name",
            )),
            Some(file_name) => Ok(file_name.to_string()),
        }
    }

    /// Returns the name of the file being lexed.
    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }

    //TODO Add tests
    /// Returns the next token from the source file.
    fn get_token(&mut self) -> LResult<Option<Token>> {
        // Skip any whitespace
        while let Some(c) = self.iter.next_if(|&c| is_whitespace(c)) {
            self.position.col_inc();
            if is_new_line(c) {
                self.position.line_inc();
            }
        }

        let start_pos = self.position;
        let c = match self.iter.next() {
            Some(c) => c,
            None => return Ok(None),
        };
        self.position.col_inc();

        // Two-character delimiters or operators
        if let Some(n) = self.iter.peek() {
            let n = *n;
            let tt = TokenType::extract_keyword_or_symbol(match (c, n) {
                ('-', '>') => "->",
                ('/', '/') => {
                    // Skip comment
                    while self.iter.next_if(|&c| !is_new_line(c)).is_some() {
                        self.position.col_inc();
                    }
                    return self.get_token();
                }
                ('&', '&') => "&&",
                ('|', '|') => "||",
                ('=', '=') => "==",
                ('!', '=') => "!=",
                ('<', '=') => "<=",
                ('>', '=') => ">=",
                _ => "",
            });
            if let Some(tt) = tt {
                self.iter.next();
                self.position.col_inc();
                let token = Token::new(tt, start_pos, self.position);
                return Ok(Some(token));
            }
        }

        // One-character delimiters or operators
        let tt = TokenType::extract_keyword_or_symbol(c.to_string().as_str());
        if let Some(tt) = tt {
            let token = Token::new(tt, start_pos, self.position);
            return Ok(Some(token));
        }

        // Number literals
        if c.is_ascii_digit() {
            let mut num_str = String::from(c);
            while let Some(c) = self.iter.next_if(|&c| is_digit(c)) {
                num_str.push(c);
                self.position.col_inc();
            }

            let tt = if self.iter.peek().is_some_and(|c| *c == '.') {
                // Floating point literal
                num_str.push(self.iter.next().unwrap());
                self.position.col_inc();
                while let Some(c) = self.iter.next_if(|&c| is_digit(c)) {
                    num_str.push(c);
                    self.position.col_inc();
                }

                let float_val = num_str.parse::<f64>().map_err(|_| {
                    let err_kind = LexerErrorKind::InvalidFloatLiteral(num_str);
                    LexerError::new(err_kind, Span::new(start_pos, self.position))
                })?;

                TokenType::FloatLit(float_val)
            } else {
                // Integer literal
                let int_val = num_str.parse::<i32>().map_err(|_| {
                    let err_kind = LexerErrorKind::InvalidIntLiteral(num_str);
                    LexerError::new(err_kind, Span::new(start_pos, self.position))
                })?;

                TokenType::IntLit(int_val)
            };

            return Ok(Some(Token::new(tt, start_pos, self.position)));
        }

        // String literals (not supported; can only appear as an ABI) //TODO Add support for string literals
        if c == '"' {
            let mut str_lit = String::new();
            while let Some(c) = self.iter.next_if(|&c| c != '"') {
                str_lit.push(c);
                self.position.col_inc();
            }
            if self.iter.next().is_none() {
                let err_kind = LexerErrorKind::UnterminatedStringLiteral;
                return Err(LexerError::new(
                    err_kind,
                    Span::new(start_pos, self.position),
                ));
            }
            self.position.col_inc();

            return Ok(Some(Token::new(
                TokenType::Abi(str_lit),
                start_pos,
                self.position,
            )));
        }

        // Identifier or keyword
        if is_xid_start(c) || c == '_' {
            let mut id_str = String::new();
            id_str.push(c);
            while let Some(c) = self.iter.next_if(|&c| is_xid_continue(c)) {
                id_str.push(c);
                self.position.col_inc();
            }

            let tt = TokenType::extract_keyword_or_symbol(id_str.as_str())
                .unwrap_or(TokenType::Ident(id_str));

            return Ok(Some(Token::new(tt, start_pos, self.position)));
        }

        // Unknown token
        let err_kind = LexerErrorKind::UnknownToken(c);
        Err(LexerError::new(
            err_kind,
            Span::new(start_pos, self.position),
        ))
    }
}

impl Iterator for Lexer {
    type Item = LResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.get_token() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
