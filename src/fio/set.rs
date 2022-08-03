#[cfg(test)]
use crate::fio::common::assert_parse;

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
            }
        }
    )(input)
}

#[test]
fn test_parse_set() {
    assert_parse(
        parse_set(Span::new("{Nil}")),
        SetType {
            elm_type: BaseType::Nil
        }
    );
    assert_parse(
        parse_set(Span::new("{.}")),
        SetType {
            elm_type: BaseType::Any
        }
    );
    assert_parse(
        parse_set(Span::new("{.Number}")),
        SetType {
            elm_type: BaseType::Builtin(BuiltinType{
                name: "Number".to_string()
            })
        }
    );
    assert_parse(
        parse_set(Span::new("{Number}")),
        SetType {
            elm_type: BaseType::Ref(RefType{
                name: "Number".to_string()
            })
        }
    );

    ///// Spacing
    assert_parse(
        parse_set(Span::new("{   Number \n \t }")),
        SetType {
            elm_type: BaseType::Ref(RefType{
                name: "Number".to_string()
            })
        }
    );

}
