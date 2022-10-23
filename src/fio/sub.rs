use super::any::parse_any;
use super::builtin::parse_builtin;
use super::r#ref::parse_ref;
use super::r#struct::parse_struct;
use super::relation::parse_relation;
use super::seq::parse_seq;
use super::set::parse_set;
use super::tuple::parse_tuple;
use crate::fio::r#type::parse_subtypeable;

use super::{Type};
use crate::common::FilePosition;
use crate::fio::common::Span;
use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, is_not, take_until1};
use nom::character::complete::alphanumeric1;
use nom::combinator::{peek, eof};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::{bytes::complete::tag, combinator::map, sequence::delimited, IResult};
use serde::{Serialize, Deserialize};
use super::common::{ws, take_until_unbalanced};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    pub param: String,
    pub expr: String,
    pub position: FilePosition,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubType {
    pub base: Box<Type>,
    pub constraints: Vec<Constraint>,
    pub position: FilePosition,
}

pub fn parse_anonymous_constraint(input: Span) -> IResult<Span, Constraint> {
    let constraint = take_until_unbalanced('(', ')');

    let (rest, parsed) = match constraint(input) {
        Ok(c) => c,
        Err(err) => {
            return Err(err)
        }
    };
    println!("parsed --> {}", parsed);
    println!("res --> {}", rest);

    let mut param = preceded(ws, terminated(alphanumeric1, preceded(ws, tag("|"))));
    match param(parsed) {
        Ok((expr, param)) => {
            let c = Constraint {
                param: param.to_string(),
                expr: expr.to_string(),
                position: parsed.into(),
            };
            Ok((rest, c))
        }
        Err(err) => return Err(err)
    }
}

pub fn check_looks_like_sub(input: Span) -> IResult<Span, bool> {
    // First we ensure that it looks like a subtype
    let types = alt((
        map(parse_builtin, |_| {}),
        map(parse_ref, |_| {}),
        map(parse_seq, |_| {}),
        map(parse_set, |_| {}),
        map(parse_struct, |_| {}),
        map(parse_tuple, |_| {}),
        map(parse_relation, |_| {}),
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

#[cfg(test)]
use super::{BuiltinType, SeqType};
#[cfg(test)]
use crate::fio::common::assert_parse;

#[test]
fn test_parse_anonymous_constraint() {
    assert_parse(
        parse_anonymous_constraint(Span::new("(s | some anonymous constraint)")),
        Constraint {
            param: "s".to_string(),
            expr: " some anonymous constraint".to_string(),
            position: FilePosition { line: 1, column: 2 },
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
                expr: " some anonymous constraint".to_string(),
                position: FilePosition { line: 1, column: 9 },
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
                expr: " some anonymous constraint".to_string(),
                position: FilePosition {
                    line: 1,
                    column: 11,
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
                expr: "   \nsome anonymous constraint".to_string(),
                position: FilePosition {
                    line: 1,
                    column: 14,
                },
            }],
        },
    );
}


#[test]
fn test_parse_sub_type_functions_in_expressions() {
    assert_parse(
        parse_sub(Span::new(
            ".String(s | len(s) > 8)",
        )),
        SubType {
            position: FilePosition { line: 1, column: 1 },
            base: Box::new(Type::BuiltinType(BuiltinType {
                name: "String".to_string(),
                position: FilePosition { line: 1, column: 1 },
            })),
            constraints: vec![Constraint {
                param: "s".to_string(),
                expr: " len(s) > 8".to_string(),
                position: FilePosition { line: 1, column: 9 },
            }],
        },
    );
}
