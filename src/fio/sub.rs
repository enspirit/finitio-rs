use super::any::parse_any;
use super::builtin::parse_builtin;
use super::errors::ParseError;
use super::nil::parse_nil;
use super::r#ref::parse_ref;
use super::r#struct::parse_struct;
use super::seq::parse_seq;
use super::set::parse_set;
#[cfg(test)]
use super::{any, builtin, nil, r#ref};
#[cfg(test)]
use crate::fio::common::assert_parse;
use crate::fio::r#type::parse_subtypeable;

use super::{BuiltinType, NilType, RefType, SeqType, Type, UnionType};
use crate::common::FilePosition;
use crate::fio::common::Span;
use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, is_not, take_till, take_until1};
use nom::character::complete::alphanumeric1;
use nom::combinator::{map_parser, not, peek, recognize, rest};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::{bytes::complete::tag, combinator::map, sequence::delimited, IResult};

use super::common::{parse_identifier, take_until_unbalanced, ws};
use super::r#type::{parse_type, parse_type_but_union};

#[derive(Debug, PartialEq)]
pub struct Constraint {
    pub param: String,
    pub expr: String,
    pub position: FilePosition,
}

#[derive(Debug, PartialEq)]
pub struct SubType {
    pub base: Box<Type>,
    pub constraints: Vec<Constraint>,
    pub position: FilePosition,
}

pub fn parse_parenth_content(input: Span) -> IResult<Span, String> {
    let parser = escaped_transform(is_not("\\)"), '\\', alt((map(tag(")"), |_| "\\)"),)));

    map(parser, |s| s)(input)
}

pub fn parse_anonymous_constraint(input: Span) -> IResult<Span, Constraint> {
    let param = preceded(ws, terminated(alphanumeric1, preceded(ws, tag("|"))));
    let anonymous = preceded(ws, parse_parenth_content);

    let parser = delimited(tag("("), pair(param, anonymous), tag(")"));

    map(parser, |(param, content)| Constraint {
        param: param.to_string(),
        expr: content,
        position: input.into(),
    })(input)
}

pub fn check_looks_like_sub(input: Span) -> IResult<Span, bool> {
    // First we ensure that it looks like a subtype
    let types = alt((
        map(parse_builtin, |_| {}),
        map(parse_ref, |_| {}),
        map(parse_seq, |_| {}),
        map(parse_set, |_| {}),
        map(parse_struct, |_| {}),
        map(parse_any, |_| {}),
    ));
    let check = separated_pair(types, ws, tag("("));
    match peek(check)(input) {
        Ok(_s) => Ok((input, true)),
        Err(err) => Err(err),
    }
}

pub fn parse_sub(input: Span) -> IResult<Span, SubType> {
    // First we ensure that it looks like a subtype
    match peek(check_looks_like_sub)(input) {
        Err(err) => Err(err),
        Ok(_) => {
            map(
                separated_pair(parse_subtypeable, ws, parse_anonymous_constraint),
                |(ftype, constraint)| SubType {
                    base: Box::new(ftype),
                    constraints: vec![constraint],
                    position: input.into(),
                },
            )(input)
        }
    }
}

#[test]
fn test_parse_parenth_content() {
    let output = parse_parenth_content(Span::new(r"whichever expression\) we want )"));

    let output = output.unwrap();
    assert_eq!(output.0.fragment(), &")");
    assert_eq!(output.1, r"whichever expression\) we want ".to_string())
}

#[test]
fn test_parse_anonymous_constraint() {
    assert_parse(
        parse_anonymous_constraint(Span::new("(s | some anonymous constraint)")),
        Constraint {
            param: "s".to_string(),
            expr: "some anonymous constraint".to_string(),
            position: FilePosition { line: 1, column: 1 },
        },
    );
}

#[test]
fn test_check_looks_like_sub() {
    let output = check_looks_like_sub(Span::new("Number(s | foo bar baz)"));
    let output = output.unwrap();
    assert_eq!(output.0.fragment(), &"Number(s | foo bar baz)");
    assert_eq!(output.1, true);
}

#[test]
fn test_parse_sub_type_builtin() {
    assert_parse(
        parse_sub(Span::new(".Number(s | some anonymous constraint)")),
        SubType {
            position: FilePosition { line: 1, column: 1 },
            base: Box::new(Type::BuiltinType(BuiltinType {
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 1 },
            })),
            constraints: vec![Constraint {
                param: "s".to_string(),
                expr: "some anonymous constraint".to_string(),
                position: FilePosition { line: 1, column: 8 },
            }],
        },
    );
}

#[test]
fn test_parse_sub_type_seq() {
    assert_parse(
        parse_sub(Span::new("[.Number](s | some anonymous constraint)")),
        SubType {
            position: FilePosition { line: 1, column: 1 },
            base: Box::new(Type::SeqType(SeqType {
                elm_type: Box::new(Type::BuiltinType(BuiltinType {
                    name: "Number".to_string(),
                    position: FilePosition { line: 1, column: 2 },
                })),
                position: FilePosition { line: 1, column: 1 },
            })),
            constraints: vec![Constraint {
                param: "s".to_string(),
                expr: "some anonymous constraint".to_string(),
                position: FilePosition {
                    line: 1,
                    column: 10,
                },
            }],
        },
    );
}

#[test]
fn test_parse_sub_type_spacing() {
    assert_parse(
        parse_sub(Span::new(
            "[ .Number ] (   s  |   \nsome anonymous constraint)",
        )),
        SubType {
            position: FilePosition { line: 1, column: 1 },
            base: Box::new(Type::SeqType(SeqType {
                elm_type: Box::new(Type::BuiltinType(BuiltinType {
                    name: "Number".to_string(),
                    position: FilePosition { line: 1, column: 3 },
                })),
                position: FilePosition { line: 1, column: 1 },
            })),
            constraints: vec![Constraint {
                param: "s".to_string(),
                expr: "some anonymous constraint".to_string(),
                position: FilePosition {
                    line: 1,
                    column: 13,
                },
            }],
        },
    );
}
