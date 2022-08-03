#[cfg(test)]
use crate::fio::common::assert_parse;

use crate::{fio::common::{parse_identifier, Span}, common::FilePosition};
use nom::{
    combinator::{map},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct RefType {
    pub name: String,
    pub position: FilePosition
}

pub fn parse_ref(input: Span) -> IResult<Span, RefType> {
    map(
        parse_identifier,
        |name| {
            RefType {
                name: String::from(name),
                position: input.into()
            }
        }
    )(input)
}

#[test]
fn test_parse_ref() {
    assert_parse(
        parse_ref(Span::new("Number")),
        RefType {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 1 }
        },
    );
}
