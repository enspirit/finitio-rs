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
pub struct SeqType {
    pub elm_type: BaseType,
}

pub fn parse_seq(input: Span) -> IResult<Span, SeqType> {
    map(
        delimited(
            tag("["),
            delimited(ws, parse_base_type, ws),
            tag("]")
        ), |elm_type| {
            SeqType {
                elm_type,
            }
        }
    )(input)
}

#[test]
fn test_parse_seq() {
    assert_parse(
        parse_seq(Span::new("[Nil]")),
        SeqType {
            elm_type: BaseType::Nil
        }
    );
    assert_parse(
        parse_seq(Span::new("[.]")),
        SeqType {
            elm_type: BaseType::Any
        }
    );
    assert_parse(
        parse_seq(Span::new("[.Number]")),
        SeqType {
            elm_type: BaseType::Builtin(BuiltinType{
                name: "Number".to_string()
            })
        }
    );
    assert_parse(
        parse_seq(Span::new("[Number]")),
        SeqType {
            elm_type: BaseType::Ref(RefType{
                name: "Number".to_string()
            })
        }
    );

    ///// Spacing
    assert_parse(
        parse_seq(Span::new("[   Number \n \t ]")),
        SeqType {
            elm_type: BaseType::Ref(RefType{
                name: "Number".to_string()
            })
        }
    );

}
