#[cfg(test)]
use crate::fio::common::assert_parse;

use crate::{fio::common::{parse_identifier, Span}, common::FilePosition};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use super::common::ws;

#[derive(Debug, PartialEq)]
pub struct NilType {
    pub position: FilePosition
}

pub fn parse_nil(input: Span) -> IResult<Span, NilType> {
    map(preceded(ws, tag("Nil")), |t| NilType {
        position: t.into()
    })(input)
}

#[test]
fn test_parse_nil() {
    assert_parse(
        parse_nil(Span::new("Nil")),
        NilType {
            position: FilePosition { line: 1, column: 1 }
        }
    );
    assert_parse(
        parse_nil(Span::new(" Nil")),
        NilType {
            position: FilePosition { line: 1, column: 2 }
        }
    );
    assert_parse(
        parse_nil(Span::new(" \n  Nil")),
        NilType {
            position: FilePosition { line: 2, column: 3 }
        }
    );
}
