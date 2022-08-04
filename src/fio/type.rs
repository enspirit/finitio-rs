#[cfg(test)]
use crate::{common::FilePosition, fio::common::assert_parse};

use crate::fio::seq::{parse_seq, SeqType};
use crate::fio::set::{parse_set, SetType};

use nom::{branch::alt, combinator::map, sequence::preceded, IResult};

use crate::fio::Span;

use super::{
    any::{parse_any, AnyType},
    builtin::parse_builtin,
    builtin::BuiltinType,
    common::ws,
    nil::{parse_nil, NilType},
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
    ))(input)
}

#[test]
fn test_parse_type() {
    // Nil (with spaces)
    assert_parse(
        parse_type(Span::new(" Nil")),
        Type::NilType(NilType {
            position: FilePosition { line: 1, column: 2 },
        }),
    );

    // // Ref (with spaces)
    assert_parse(
        parse_type(Span::new(" Number")),
        Type::RefType(RefType {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 2 },
        }),
    );

    // // Seq (with spaces)
    assert_parse(
        parse_type(Span::new(" [ Number ]")),
        Type::SeqType(SeqType {
            position: FilePosition { line: 1, column: 2 },
            elm_type: Box::new(Type::RefType(RefType {
                name: String::from("Number"),
                position: FilePosition { line: 1, column: 4 },
            })),
        }),
    );
}
