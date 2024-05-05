//! The module containing the lexer for the μRust compiler.

use std::io;
use std::iter::Peekable;
use std::path::Path;

use fallible_iterator::FallibleIterator;
use unicode_ident::{is_xid_continue, is_xid_start};

use crate::token::{Position, Span, Token, TokenType};

use self::error::*;
use self::file_reader::*;

pub mod error;
mod file_reader;
mod helper;

/// A result of a lexing operation.
pub type Result<T> = std::result::Result<T, LexerError>;

/// The lexer for the μRust compiler.
///
/// The `Lexer` is a [FallibleIterator] with [Token] as items and [LexerError] as error type.
/// It reads a file character by character and produces a stream of tokens.
/// The last token produced will always be [TokenType::EOF].
/// # Examples
///
/// The Lexer would typically be used in a parser by manually calling [next](FallibleIterator::next) on it.
/// ```no_run
/// # use fallible_iterator::FallibleIterator;
/// # use mini_rust_compiler_components::lexer::Lexer;
/// # use mini_rust_compiler_components::token::{Token, TokenType};
///
/// let mut lexer = Lexer::new("path/to/file").unwrap();
/// loop {
///     match lexer.next() {
///         Ok(Some(token)) if token.is_eof() => {
///             // Reached the end of file, stop parsing
///             break;
///         }
///         Ok(Some(token)) => {
///             // Do something with the token
///         }
///         Ok(None) => unreachable!("EOF token should have been produced by now."),
///         Err(err) => {
///             // Handle the error
///         }
///     }
/// }
/// ```
///
/// The Lexer can also be used as a normal iterator over `lexer::Result<Token>`.
/// ```no_run
/// # use fallible_iterator::FallibleIterator;
/// # use mini_rust_compiler_components::lexer::Lexer;
///
/// let mut lexer = Lexer::new("path/to/file").unwrap();
/// for result in lexer.iterator() {
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
    filename: String,
    position: Position,
    iter: Peekable<FileReaderIter>,
    finished: bool,
}

impl Lexer {
    //TODO Add tests
    /// Creates a new `Lexer` that will lex the file at the given path.
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Lexer> {
        let filename = helper::filename_from_path(&path)?;
        let iter = FileReader::new(path).try_iter()?.peekable();
        Ok(Lexer {
            filename,
            position: Position::new(),
            iter,
            finished: false,
        })
    }

    /// Returns the name of the file being lexed.
    pub fn filename(&self) -> &str {
        &self.filename
    }

    //TODO Add tests
    /// Returns the next token from the source file.
    fn next_token(&mut self) -> Result<Token> {
        // Skip any whitespace
        while let Some(c) = self.iter.next_if(|&c| helper::is_whitespace(c)) {
            self.position.col_inc();
            if helper::is_new_line(c) {
                self.position.line_inc();
            }
        }

        let start_pos = self.position;
        let c = match self.iter.next() {
            Some(c) => c,
            None => {
                self.finished = true;
                return Ok(Token::eof(self.position));
            }
        };
        self.position.col_inc();

        // Two-character delimiters or operators
        if let Some(n) = self.iter.peek() {
            let n = *n;
            let tt = TokenType::extract_keyword_or_symbol(match (c, n) {
                ('-', '>') => "->",
                ('/', '/') => {
                    // Skip comment
                    while self.iter.next_if(|&c| !helper::is_new_line(c)).is_some() {
                        self.position.col_inc();
                    }
                    return self.next_token();
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
                return Ok(token);
            }
        }

        // One-character delimiters or operators
        let tt = TokenType::extract_keyword_or_symbol(c.to_string().as_str());
        if let Some(tt) = tt {
            let token = Token::new(tt, start_pos, self.position);
            return Ok(token);
        }

        // Number literals
        if c.is_ascii_digit() {
            let mut num_str = String::from(c);
            while let Some(c) = self.iter.next_if(|&c| helper::is_digit(c)) {
                num_str.push(c);
                self.position.col_inc();
            }

            let tt = if self.iter.peek().is_some_and(|c| *c == '.') {
                // Floating point literal
                num_str.push(self.iter.next().unwrap());
                self.position.col_inc();
                while let Some(c) = self.iter.next_if(|&c| helper::is_digit(c)) {
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

            return Ok(Token::new(tt, start_pos, self.position));
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

            return Ok(Token::new(
                TokenType::Abi(str_lit),
                start_pos,
                self.position,
            ));
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

            return Ok(Token::new(tt, start_pos, self.position));
        }

        // Unknown token
        let err_kind = LexerErrorKind::UnknownToken(c);
        Err(LexerError::new(
            err_kind,
            Span::new(start_pos, self.position),
        ))
    }
}

impl FallibleIterator for Lexer {
    type Item = Token;
    type Error = LexerError;

    fn next(&mut self) -> std::result::Result<Option<Self::Item>, Self::Error> {
        if self.finished {
            return Ok(None);
        }

        self.next_token().map(Some)
    }
}
