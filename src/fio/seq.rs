use std::fs::File;

#[cfg(test)]
use crate::fio::common::assert_parse;
#[cfg(test)]
use super::{any, nil, builtin, r#ref};

use super::Type;
use crate::common::FilePosition;
use crate::fio::common::{Span};
use nom::{
    bytes::complete::tag,
    combinator::{map},
    sequence::{preceded, separated_pair, terminated, tuple, delimited},
    IResult,
};

use super::common::ws;
use super::r#type::parse_type;

#[derive(Debug, PartialEq)]
pub struct SeqType {
    pub elm_type: Box<Type>,
    pub position: FilePosition,
}

pub fn parse_seq(input: Span) -> IResult<Span, SeqType> {
    map(
        delimited(
            tag("["),
            delimited(ws, parse_type, ws),
            tag("]")
        ), |elm_type| {
            SeqType {
                elm_type: Box::new(elm_type),
                position: input.into()
            }
        }
    )(input)
}

#[test]
fn test_parse_seq() {
    assert_parse(
        parse_seq(Span::new("[Nil]")),
        SeqType {
            elm_type: Box::new(Type::NilType(nil::NilType{
                position: FilePosition { line: 1, column: 2 }
            })),
            position: FilePosition { line: 1, column: 1 }
        }
    );
    assert_parse(
        parse_seq(Span::new("[.]")),
        SeqType {
            elm_type: Box::new(Type::AnyType(any::AnyType{
                position: FilePosition { line: 1, column: 2 }
            })),
            position: FilePosition { line: 1, column: 1 }
        }
    );
    assert_parse(
        parse_seq(Span::new("[.Number]")),
        SeqType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: Box::new(Type::BuiltinType(builtin::BuiltinType {
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 2 }
            }))
        }
    );
    assert_parse(
        parse_seq(Span::new("[Number]")),
        SeqType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: Box::new(Type::RefType(r#ref::RefType{
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 2 }
            }))
        }
    );

    ///// Spacing
    assert_parse(
        parse_seq(Span::new("[   Number \n \t ]")),
        SeqType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: Box::new(Type::RefType(r#ref::RefType{
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 5 }
            }))
        }
    );

}
