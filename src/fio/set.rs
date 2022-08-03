#[cfg(test)]
use crate::fio::common::assert_parse;
use crate::common::FilePosition;
use crate::fio::common::{parse_identifier, Span};
use crate::fio::base::{parse_base_type, BaseType, RefType};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map},
    sequence::{preceded, separated_pair, terminated, tuple, delimited},
    IResult,
};

use super::base::BuiltinType;
use super::common::ws;

#[derive(Debug, PartialEq)]
pub struct SetType {
    pub elm_type: BaseType,
    pub position: FilePosition,
}

pub fn parse_set(input: Span) -> IResult<Span, SetType> {
    map(
        delimited(
            tag("{"),
            delimited(ws, parse_base_type, ws),
            tag("}")
        ), |elm_type| {
            SetType {
                elm_type,
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
            elm_type: BaseType::Nil,
            position: FilePosition { line: 1, column: 1 }
        }
    );
    assert_parse(
        parse_set(Span::new("{.}")),
        SetType {
            elm_type: BaseType::Any,
            position: FilePosition { line: 1, column: 1 }
        }
    );
    assert_parse(
        parse_set(Span::new("{.Number}")),
        SetType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: BaseType::Builtin(BuiltinType{
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 2 }
            })
        }
    );
    assert_parse(
        parse_set(Span::new("{Number}")),
        SetType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: BaseType::Ref(RefType{
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 2 }
            })
        }
    );

    ///// Spacing
    assert_parse(
        parse_set(Span::new("{   Number \n \t }")),
        SetType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: BaseType::Ref(RefType{
                name: "Number".to_string(),
                position: FilePosition { line: 1, column: 5 }
            })
        }
    );

}
