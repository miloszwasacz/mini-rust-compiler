//! A module containing macros used by the production rules of the parser.

//TODO Improve documentation

/// A macro that consumes the next token and returns
/// [`ParserError::UnexpectedToken`](crate::parser::ParserError::UnexpectedToken)
/// with the consumed token.
macro_rules! unknown_token {
    ($self:expr, $expected:expr) => {{
        let token = $self
            .consume()
            .expect("Unsuccessful consuming after successful peeking should be impossible.");
        unknown_token!($self, token, $expected)
    }};
    ($self:expr, $token:expr, $expected:expr) => {
        Err(ParserError::UnexpectedToken {
            actual: $token,
            expected: $expected,
        })
    };
}

/// Consumes the next token or uses the provided one, checks if it matches
/// the expected [token type](crate::token::Token::ty), and returns its
/// [span](crate::token::Token::span).
///
/// # Errors
///
/// If the consumed token does not match the expected token type, [`unknown_token`] is returned
/// by the calling function (using the `?` operator).
macro_rules! assert_token {
    ($self:expr, $expected:pat, $expected_display:expr) => {{
        let token = $self.consume()?;
        assert_token!($self, token, $expected, $expected_display)
    }};
    ($self:expr, $token:expr, $expected:pat, $expected_display:expr) => {
        match $token.ty() {
            $expected => $token.span(),
            _ => unknown_token!($self, $token, $expected_display)?,
        }
    };
}

//TODO Refactor some usages of assert_token to use expect_token instead
/// Peeks the next token and checks if it matches the expected [token type](crate::token::Token::ty).
/// If it does, consumes the token and returns its [span](crate::token::Token::span)
/// wrapped in `Some`; otherwise, returns `None`.
macro_rules! expect_token {
    ($self:expr, $expected:pat) => {{
        match $self.peek()?.ty() {
            $expected => {
                let token = $self.consume()?;
                let token = match token.ty() {
                    $expected => token,
                    _ => unreachable!(),
                };
                Some(token.span())
            }
            _ => None,
        }
    }};
}

/// Consumes the next token or uses the provided one, checks if it is an identifier
/// and returns its value.
///
/// # Errors
///
/// If the consumed token is not an identifier, [`unknown_token`] is returned
/// by the calling function (using the `?` operator).
macro_rules! assert_ident {
    ($self:expr, $expected_display:expr) => {{
        let token = $self.consume()?;
        assert_ident!($self, token, $expected_display)
    }};
    ($self:expr, $token:expr, $expected_display:expr) => {
        match $token.ty() {
            Ident(ident) => ident.clone(),
            _ => unknown_token!($self, $token, $expected_display)?,
        }
    };
}

/// Consumes the next token or uses the provided one, checks if it is an identifier or an underscore:
/// - If it is an identifier, returns its value wrapped in `Some`.
/// - If it is an underscore, returns `None`.
///
/// # Errors
///
/// If the consumed token is neither an identifier nor an underscore, [`unknown_token`] is returned
/// by the calling function (using the `?` operator).
macro_rules! assert_ident_or_underscore {
    ($self:expr) => {{
        let token = $self.consume()?;
        assert_ident_or_underscore!($self, token)
    }};
    ($self:expr, $token:expr) => {
        match $token.ty() {
            Ident(ident) => Some(ident.clone()),
            Underscore => None,
            //TODO Improve error message
            _ => unknown_token!($self, $token, "<pattern>")?,
        }
    };
}

pub(super) use assert_ident;
pub(super) use assert_ident_or_underscore;
pub(super) use assert_token;
pub(super) use expect_token;
pub(super) use unknown_token;
