#[cfg(test)]
use super::{any, builtin, nil, r#ref};
#[cfg(test)]
use crate::fio::common::assert_parse;

use super::{Type, NilType, RefType, SeqType, UnionType, BuiltinType};
use crate::common::FilePosition;
use crate::fio::common::Span;
use nom::branch::alt;
use nom::bytes::complete::{is_not, take_until1, escaped_transform, take_till};
use nom::character::complete::alphanumeric1;
use nom::combinator::{map_parser, rest, not, recognize};
use nom::multi::{separated_list1, separated_list0};
use nom::sequence::{preceded, terminated, pair, separated_pair};
use nom::{bytes::complete::tag, combinator::map, sequence::delimited, IResult};

use super::common::{ws, take_until_unbalanced};
use super::r#type::{parse_type, parse_type_but_union};

#[derive(Debug, PartialEq)]
pub struct Constraint {
    pub param: String,
    pub expr: String,
    pub position: FilePosition
}

#[derive(Debug, PartialEq)]
pub struct SubType {
    pub base: Box<Type>,
    pub constraints: Vec<Constraint>,
    pub position: FilePosition
}

pub fn parse_parenth_content(input: Span) -> IResult<Span, String> {
    let parser = escaped_transform(
        is_not("\\)"), '\\', alt((
            map(tag(")"), |_| "\\)"),
        )));

    map(parser, |s| s)(input)
}

pub fn parse_anonymous_constraint(input: Span) -> IResult<Span, Constraint> {
    let param = preceded(ws, terminated(alphanumeric1, preceded(ws, tag("|"))));
    let anonymous = preceded(ws, parse_parenth_content);

    let parser = delimited(
        tag("("),
        pair(param, anonymous),
        tag(")")
    );

    map(
        parser,
        |(param, content)| {
            Constraint {
                param: param.to_string(),
                expr: content,
                position: input.into()
            }
        }
    )(input)
}

pub fn parse_sub_type(input: Span) -> IResult<Span, SubType> {
    map(
        separated_pair(
            parse_type,
            ws,
            parse_anonymous_constraint
        ),
        |(ftype, constraint)| {
            SubType {
                base: Box::new(ftype),
                constraints: vec![constraint],
                position: input.into()
            }
        }
    )(input)
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
            position: FilePosition { line: 1, column: 1 }
        },
    );
}

#[test]
fn test_parse_sub_type_builtin() {
    assert_parse(
        parse_sub_type(Span::new(".Number(s | some anonymous constraint)")),
        SubType {
            position: FilePosition { line: 1, column: 1 },
            base: Box::new(Type::BuiltinType(BuiltinType {
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 1 }
            })),
            constraints: vec![
                Constraint {
                    param: "s".to_string(),
                    expr: "some anonymous constraint".to_string(),
                    position: FilePosition { line: 1, column: 8 }
                }
            ]
        },
    );
}

#[test]
fn test_parse_sub_type_seq() {
    assert_parse(
        parse_sub_type(Span::new("[.Number](s | some anonymous constraint)")),
        SubType {
            position: FilePosition { line: 1, column: 1 },
            base: Box::new(
                Type::SeqType(SeqType {
                    elm_type: Box::new(Type::BuiltinType(BuiltinType {
                        name: "Number".to_string(),
                        position: FilePosition { line: 1, column: 2 }
                    })),
                    position: FilePosition { line: 1, column: 1 },
                }),
            ),
            constraints: vec![
                Constraint {
                    param: "s".to_string(),
                    expr: "some anonymous constraint".to_string(),
                    position: FilePosition { line: 1, column: 10 }
                }
            ]
        },
    );
}

#[test]
fn test_parse_sub_type_spacing() {
    assert_parse(
        parse_sub_type(Span::new("[ .Number ] (   s  |   \nsome anonymous constraint)")),
        SubType {
            position: FilePosition { line: 1, column: 1 },
            base: Box::new(
                Type::SeqType(SeqType {
                    elm_type: Box::new(Type::BuiltinType(BuiltinType {
                        name: "Number".to_string(),
                        position: FilePosition { line: 1, column: 3 }
                    })),
                    position: FilePosition { line: 1, column: 1 },
                }),
            ),
            constraints: vec![
                Constraint {
                    param: "s".to_string(),
                    expr: "some anonymous constraint".to_string(),
                    position: FilePosition { line: 1, column: 13 }
                }
            ]
        },
    );
}
