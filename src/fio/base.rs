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
pub enum BaseType {
    Nil,
    Any,
    Builtin(BuiltinType),
    Ref(RefType),
}

#[derive(Debug, PartialEq)]
pub struct BuiltinType {
    pub name: String,
    pub position: FilePosition
}

#[derive(Debug, PartialEq)]
pub struct RefType {
    pub name: String,
    pub position: FilePosition
}

pub fn parse_nil(input: Span) -> IResult<Span, BaseType> {
    map(tag("Nil"), |_| BaseType::Nil)(input)
}

pub fn parse_any(input: Span) -> IResult<Span, BaseType> {
    map(tag("."), |_| BaseType::Any)(input)
}

pub fn parse_builtin(input: Span) -> IResult<Span, BaseType> {
    map(preceded(tag("."), parse_identifier), |name| {
        BaseType::Builtin(BuiltinType {
            name: String::from(name),
            position: input.into()
        })
    })(input)
}

pub fn parse_ref(input: Span) -> IResult<Span, BaseType> {
    map(parse_identifier, |name| {
        BaseType::Ref(RefType {
            name: String::from(name),
            position: input.into()
        })
    })(input)
}

pub fn parse_base_type(input: Span) -> IResult<Span, BaseType> {
    preceded(
        ws,
        alt((
            parse_nil,
            parse_builtin,
            parse_any,
            parse_ref
        ))
    )(input)
}

#[test]
fn test_parse_none() {
    assert_parse(parse_base_type(Span::new("Nil")), BaseType::Nil);
}

#[test]
fn test_parse_any() {
    assert_parse(parse_base_type(Span::new(".")), BaseType::Any);
}

#[test]
fn test_parse_builtin() {
    assert_parse(
        parse_base_type(Span::new(".Number")),
        BaseType::Builtin(BuiltinType {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 1 }
        }),
    );
}

#[test]
fn test_parse_ref() {
    assert_parse(
        parse_base_type(Span::new("Number")),
        BaseType::Ref(RefType {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 1 }
        }),
    );
}
