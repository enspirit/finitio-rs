use nom::{
  bytes::complete::{tag, take_while1, take_until1, is_not},
  character::complete::{char, newline},
  combinator::map,
  sequence::{preceded, terminated},
  IResult,
};

use crate::common::FilePosition;

use super::{
  common::{ws, ws1},
  Span,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
  pub filename: String,
  pub position: FilePosition,
}

pub fn parse_import(input: Span) -> IResult<Span, Import> {
  preceded(
      ws,
      map(
          preceded(
              terminated(tag("@import"), ws1),
              parse_filename,
          ),
          |filename| Import {
              filename: filename.to_string(),
              position: filename.into(),
          },
      ),
  )(input)
}

pub fn parse_filename(input: Span) -> IResult<Span, Span> {
  take_while1(|c: char| !c.is_whitespace())(input)
}

#[test]
fn test_parse_import() {
  use super::common::assert_parse;
  use super::*;
  let content = "@import other.fio";
  assert_parse(
      parse_import(Span::new(content)),
      Import {
          filename: String::from("other.fio"),
          position: FilePosition { line: 1, column: 9 },
      },
  );
}

#[test]
fn test_parse_import_with_directory() {
  use super::common::assert_parse;
  use super::*;
  let content = "@import flair/data";
  assert_parse(
      parse_import(Span::new(content)),
      Import {
          filename: String::from("flair/data"),
          position: FilePosition { line: 1, column: 9 },
      },
  );
}
