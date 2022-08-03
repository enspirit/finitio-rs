#[cfg(test)]
use crate::fio::{
    common::assert_parse,
    base::{RefType, BuiltinType},
};

use crate::{fio::common::{parse_identifier, Span}, common::FilePosition};
use crate::fio::base::{parse_base_type, BaseType};
use crate::fio::seq::{parse_seq, SeqType};
use crate::fio::set::{parse_set, SetType};

use nom::{
    bytes::complete::tag,
    combinator::{map},
    sequence::{preceded, separated_pair},
    IResult, branch::alt,
};

use super::{common::ws};

#[derive(Debug, PartialEq)]
pub enum Type {
    BaseType(BaseType),
    SeqType(SeqType),
    SetType(SetType),
}

pub fn parse_type(input: Span) -> IResult<Span, Type> {
    alt((
        map(parse_base_type, Type::BaseType),
        map(parse_seq, Type::SeqType),
        map(parse_set, Type::SetType),
    ))
    (input)
}

#[test]
fn test_parse_type() {
    // Nil (with spaces)
    assert_parse(
        parse_type(Span::new(" Nil")),
        Type::BaseType(BaseType::Nil)
    );

    // Ref (with spaces)
    assert_parse(
        parse_type(Span::new(" Number")),
        Type::BaseType(BaseType::Ref(RefType {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 2 }
        }))
    );

    // Seq (with spaces)
    assert_parse(
        parse_type(Span::new("[ Number ]")),
        Type::SeqType(SeqType {
            position: FilePosition { line: 1, column: 1 },
            elm_type: BaseType::Ref(RefType {
                name: String::from("Number"),
                position: FilePosition { line: 1, column: 3 }
            })
        })
    );
}
