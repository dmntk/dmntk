//! `FEEL` language parser

mod errors;
mod lexer;

pub use errors::{ParserError, Result};
pub use lexer::{Lexer, Token};

/// Parses an `expression` as defined in grammar rule `1`.
#[must_use]
pub const fn parse_expression() -> bool {
    true
}
