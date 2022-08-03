use std::fs::File;

#[cfg(test)]
use crate::fio::common::assert_parse;
use crate::common::FilePosition;
use crate::fio::common::{parse_identifier, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map},
    sequence::{preceded, separated_pair, terminated, tuple, delimited},
    IResult,
};

use super::{Type, nil, any, builtin, r#ref};
use super::common::ws;
use super::r#type::parse_type;

#[derive(Debug, PartialEq)]
pub struct SetType {
    pub elm_type: Box<Type>,
    pub position: FilePosition,
}

pub fn parse_set(input: Span) -> IResult<Span, SetType> {
    map(
        delimited(
            tag("{"),
            delimited(ws, parse_type, ws),
            tag("}")
        ), |elm_type| {
            SetType {
                elm_type: Box::new(elm_type),
                position: input.into(),
            }
        }
    )(input)
}

#[test]
fn test_parse_set() {
    assert_parse(
        parse_set(Span::new("{Nil}")),
        SetType {
            elm_type: Box::new(Type::NilType(nil::NilType{
                position: FilePosition{ line: 1, column: 2 }
            })),
            position: FilePosition { line: 1, column: 1 }
        }
    );
    assert_parse(
        parse_set(Span::new("{.}")),
        SetType {
            elm_type: Box::new(Type::AnyType(any::AnyType {
                position: FilePosition { line: 1, column: 2 }
            })),
            position: FilePosition { line: 1, column: 1 }
        }
    );
    assert_parse(
        parse_set(Span::new("{.Number}")),
        SetType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: Box::new(Type::BuiltinType(builtin::BuiltinType {
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 2 }
            }))
        }
    );
    assert_parse(
        parse_set(Span::new("{Number}")),
        SetType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: Box::new(Type::RefType(r#ref::RefType {
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 2 }
            }))
        }
    );

    ///// Spacing
    assert_parse(
        parse_set(Span::new("{   Number \n \t }")),
        SetType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: Box::new(Type::RefType(r#ref::RefType{
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 5 }
            }))
        }
    );

}
