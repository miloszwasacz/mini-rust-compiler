//! Defines the different types of tokens that can be found in a μRust program.

use std::fmt;
use std::rc::Rc;

pub use span::*;

mod span;

/// The different types of [Token]s that can be found in a μRust program
/// defined by regular languages.
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    //#region Patterns
    /// Matches a valid UTF-8 identifier
    Ident(Rc<str>),
    /// `_`
    Underscore,
    /// Only `"C"` ABI is valid at this point
    Abi(Rc<str>),
    //#endregion

    //#region Keywords
    /// `fn`
    Fn,
    /// `static`
    Static,
    /// `extern`
    Extern,
    /// `let`
    Let,
    /// `mut`
    Mut,
    /// `as`
    As,
    /// `loop`
    Loop,
    /// `while`
    While,
    /// `if`
    If,
    /// `else`
    Else,
    /// `unsafe`
    Unsafe,
    /// `return`
    Return,
    //#endregion

    //#region Delimiters
    /// `(`
    LPar,
    /// `)`
    RPar,
    /// `{`
    LBra,
    /// `}`
    RBra,
    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `->`
    Arrow,
    //#endregion

    //#region Literals
    /// Matches `[0-9]+`
    IntLit(i32),
    /// Matches `[0-9]+.[0-9]+`
    FloatLit(f64),
    /// Matches `true` or `false`
    BoolLit(bool),
    //#endregion

    //#region Operators
    /// `=`
    Assign,

    //#region Arithmetic or Logical operators
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Asterisk,
    /// `/`
    Div,
    /// `%`
    Mod,
    /// `&`
    BitAnd,
    /// `|`
    BitOr,
    /// `^`
    BitXor,
    /// `!`
    Not,
    //#endregion

    //#region Lazy Boolean operators
    /// `&&`
    And,
    /// `||`
    Or,
    //#endregion

    //#region Comparison operators
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `>`
    Gt,
    /// `<`
    Lt,
    /// `>=`
    Ge,
    /// `<=`
    Le,
    //#endregion

    //#endregion
    
    /// Represents the end of the file
    EOF,
}

impl TokenType {
    /// Converts the provided string slice to a [TokenType] if it is a keyword,
    /// a delimiter, or an operator
    ///
    /// # Examples
    ///
    /// ```
    /// # use mini_rust_compiler_components::token::TokenType;
    ///
    /// let tok = TokenType::extract_keyword_or_symbol("fn");
    /// assert_eq!(tok, Some(TokenType::Fn));
    ///
    /// let tok = TokenType::extract_keyword_or_symbol("true");
    /// assert_eq!(tok, Some(TokenType::BoolLit(true)));
    ///
    /// let tok = TokenType::extract_keyword_or_symbol("}");
    /// assert_eq!(tok, Some(TokenType::RBra));
    ///
    /// let tok = TokenType::extract_keyword_or_symbol("||");
    /// assert_eq!(tok, Some(TokenType::Or));
    pub fn extract_keyword_or_symbol(s: &str) -> Option<Self> {
        Some(match s {
            "fn" => TokenType::Fn,
            "static" => TokenType::Static,
            "extern" => TokenType::Extern,
            "let" => TokenType::Let,
            "mut" => TokenType::Mut,
            "as" => TokenType::As,
            "loop" => TokenType::Loop,
            "while" => TokenType::While,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "unsafe" => TokenType::Unsafe,
            "return" => TokenType::Return,
            "(" => TokenType::LPar,
            ")" => TokenType::RPar,
            "{" => TokenType::LBra,
            "}" => TokenType::RBra,
            ";" => TokenType::Semi,
            "," => TokenType::Comma,
            ":" => TokenType::Colon,
            "->" => TokenType::Arrow,
            "true" => TokenType::BoolLit(true),
            "false" => TokenType::BoolLit(false),
            "=" => TokenType::Assign,
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "*" => TokenType::Asterisk,
            "/" => TokenType::Div,
            "%" => TokenType::Mod,
            "&" => TokenType::BitAnd,
            "|" => TokenType::BitOr,
            "^" => TokenType::BitXor,
            "!" => TokenType::Not,
            "&&" => TokenType::And,
            "||" => TokenType::Or,
            "==" => TokenType::Eq,
            "!=" => TokenType::Ne,
            ">" => TokenType::Gt,
            "<" => TokenType::Lt,
            ">=" => TokenType::Ge,
            "<=" => TokenType::Le,
            _ => return None,
        })
    }
}

/// A primitive production in μRust's grammar defined by regular languages.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    span: Span,
}

impl Token {
    /// Creates a new [Token] with the given [TokenType] starting and ending
    /// at the given [Position]s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mini_rust_compiler_components::token::{Token, TokenType, Position};
    ///
    /// let start = Position::new_at(1, 1);
    /// let end = Position::new_at(1, 2);
    /// let tok = Token::new(TokenType::Plus, start, end);
    /// assert_eq!(tok.ty(), &TokenType::Plus);
    /// assert_eq!(tok.span().start(), start);
    /// assert_eq!(tok.span().end(), end);
    /// ```
    pub fn new(token_type: TokenType, start: Position, end: Position) -> Token {
        let span = Span::new(start, end);
        Token { token_type, span }
    }

    /// Creates a new [Token] that represents the end of the file.
    pub fn eof(pos: Position) -> Token {
        Token::new(TokenType::EOF, pos, pos)
    }

    /// Returns the [TokenType] of this [Token].
    pub fn ty(&self) -> &TokenType {
        &self.token_type
    }

    /// Returns the [Span] of this [Token].
    pub fn span(&self) -> Span {
        self.span
    }

    /// Whether this [Token] is the end of the file.
    pub fn is_eof(&self) -> bool {
        self.token_type == TokenType::EOF
    }
}

impl From<Token> for TokenType {
    fn from(token: Token) -> Self {
        token.token_type
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at {}", self.token_type, self.span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_extract_keyword_or_symbol() {
        assert_eq!(
            TokenType::extract_keyword_or_symbol("fn"),
            Some(TokenType::Fn)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("static"),
            Some(TokenType::Static)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("extern"),
            Some(TokenType::Extern)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("let"),
            Some(TokenType::Let)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("mut"),
            Some(TokenType::Mut)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("as"),
            Some(TokenType::As)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("loop"),
            Some(TokenType::Loop)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("while"),
            Some(TokenType::While)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("if"),
            Some(TokenType::If)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("else"),
            Some(TokenType::Else)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("unsafe"),
            Some(TokenType::Unsafe)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("return"),
            Some(TokenType::Return)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("("),
            Some(TokenType::LPar)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol(")"),
            Some(TokenType::RPar)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("{"),
            Some(TokenType::LBra)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("}"),
            Some(TokenType::RBra)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol(";"),
            Some(TokenType::Semi)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol(","),
            Some(TokenType::Comma)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol(":"),
            Some(TokenType::Colon)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("->"),
            Some(TokenType::Arrow)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("true"),
            Some(TokenType::BoolLit(true))
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("false"),
            Some(TokenType::BoolLit(false))
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("="),
            Some(TokenType::Assign)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("+"),
            Some(TokenType::Plus)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("-"),
            Some(TokenType::Minus)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("*"),
            Some(TokenType::Asterisk)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("/"),
            Some(TokenType::Div)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("%"),
            Some(TokenType::Mod)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("&"),
            Some(TokenType::BitAnd)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("|"),
            Some(TokenType::BitOr)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("^"),
            Some(TokenType::BitXor)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("!"),
            Some(TokenType::Not)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("&&"),
            Some(TokenType::And)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("||"),
            Some(TokenType::Or)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("=="),
            Some(TokenType::Eq)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("!="),
            Some(TokenType::Ne)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol(">"),
            Some(TokenType::Gt)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("<"),
            Some(TokenType::Lt)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol(">="),
            Some(TokenType::Ge)
        );
        assert_eq!(
            TokenType::extract_keyword_or_symbol("<="),
            Some(TokenType::Le)
        );
        assert_eq!(TokenType::extract_keyword_or_symbol(""), None);
        assert_eq!(TokenType::extract_keyword_or_symbol("invalid"), None);
        assert_eq!(TokenType::extract_keyword_or_symbol("== "), None);
        assert_eq!(TokenType::extract_keyword_or_symbol(" !"), None);
        assert_eq!(TokenType::extract_keyword_or_symbol("|||"), None);
        assert_eq!(TokenType::extract_keyword_or_symbol("f n"), None);
        assert_eq!(TokenType::extract_keyword_or_symbol("Return"), None);
        assert_eq!(TokenType::extract_keyword_or_symbol("0123"), None);
        assert_eq!(TokenType::extract_keyword_or_symbol(r#""C""#), None);
    }
}
