use std::fmt;

use crate::fio::common::Span;

#[derive(Debug, PartialEq)]
pub enum ParseError<'a> {
    Nom(nom::Err<nom::error::Error<Span<'a>>>),
    TrailingGarbage(Span<'a>),
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Nom(e) => write!(f, "Parsing failed: {}", e),
            ParseError::TrailingGarbage(e) => write!(f, "File contains trailing garbage: {}", e),
        }
    }
}

impl std::error::Error for ParseError<'_> {}
