#[cfg(test)]
use crate::fio::common::assert_parse;

use crate::fio::common::{Span, parse_identifier};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    error::context,
    multi::{many0, separated_list0},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Type {
  Nil,
  Any,
  Builtin(BuiltinType),
  Ref(RefType)
}

#[derive(Debug, PartialEq)]
pub struct BuiltinType {
  name: String
}

#[derive(Debug, PartialEq)]
pub struct RefType {
  name: String
}

pub fn parse_nil(input: Span) -> IResult<Span, Type> {
    map(tag("Nil"), |_| Type::Nil)(input)
}

pub fn parse_any(input: Span) -> IResult<Span, Type> {
  map(tag("."), |_| Type::Any)(input)
}

pub fn parse_builtin(input: Span) -> IResult<Span, Type> {
  map(preceded(tag("."), parse_identifier), |name| Type::Builtin(BuiltinType{
    name: String::from(name)
  }))(input)
}

pub fn parse_ref(input: Span) -> IResult<Span, Type> {
  map(parse_identifier, |name| Type::Ref(RefType{
    name: String::from(name)
  }))(input)
}

pub fn parse_type(input: Span) -> IResult<Span, Type> {
  alt((
    parse_nil,
    parse_builtin,
    parse_any,
    parse_ref,
  ))(input)
}

#[test]
fn test_parse_none() {
    assert_parse(parse_type(Span::new("Nil")), Type::Nil);
}

#[test]
fn test_parse_any() {
    assert_parse(parse_type(Span::new(".")), Type::Any);
}

#[test]
fn test_parse_builtin() {
    assert_parse(parse_type(Span::new(".Number")), Type::Builtin(BuiltinType {
      name: String::from("Number")
    }));
}

#[test]
fn test_parse_ref() {
    assert_parse(parse_type(Span::new("Number")), Type::Ref(RefType {
      name: String::from("Number")
    }));
}
