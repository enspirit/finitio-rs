use std::fs::File;

#[cfg(test)]
use crate::fio::{
    common::assert_parse,
};

use crate::{fio::common::{parse_identifier, Span}, common::FilePosition};
use crate::fio::seq::{parse_seq, SeqType};
use crate::fio::set::{parse_set, SetType};

use nom::{
    bytes::complete::tag,
    combinator::{map},
    sequence::{preceded, separated_pair},
    IResult, branch::alt,
};

use super::{
    common::ws,
    nil::{parse_nil, NilType},
    any::{parse_any, AnyType},
    builtin::parse_builtin, builtin::BuiltinType,
    r#ref::{parse_ref, RefType},
};

#[derive(Debug, PartialEq)]
pub enum Type {
    AnyType(AnyType),
    NilType(NilType),
    BuiltinType(BuiltinType),
    RefType(RefType),
    SeqType(SeqType),
    SetType(SetType),
}

pub fn parse_type(input: Span) -> IResult<Span, Type> {
    alt((
        map(preceded(ws, parse_builtin), Type::BuiltinType),
        map(preceded(ws, parse_any), Type::AnyType),
        map(preceded(ws, parse_nil), Type::NilType),
        map(preceded(ws, parse_ref), Type::RefType),
        map(preceded(ws, parse_seq), Type::SeqType),
        map(preceded(ws, parse_set), Type::SetType),
    ))
    (input)
}

#[test]
fn test_parse_type() {
    // Nil (with spaces)
    assert_parse(
        parse_type(Span::new(" Nil")),
        Type::NilType(NilType{
            position: FilePosition { line: 1, column: 2 }
        })
    );

    // // Ref (with spaces)
    assert_parse(
        parse_type(Span::new(" Number")),
        Type::RefType(RefType {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 2 }
        })
    );

    // // Seq (with spaces)
    assert_parse(
        parse_type(Span::new(" [ Number ]")),
        Type::SeqType(SeqType {
            position: FilePosition { line: 1, column: 2 },
            elm_type: Box::new(Type::RefType(RefType {
                name: String::from("Number"),
                position: FilePosition { line: 1, column: 4 }
            }))
        })
    );
}
