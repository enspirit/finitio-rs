#[cfg(test)]
use crate::{common::FilePosition, fio::common::assert_parse};

use crate::fio::seq::{parse_seq, SeqType};
use crate::fio::set::{parse_set, SetType};

use nom::{branch::alt, combinator::map, sequence::preceded, IResult};

use crate::fio::Span;

use super::r#struct::{parse_struct, StructType};
use super::relation::{RelationType, parse_relation};
use super::sub::parse_sub;
use super::tuple::{parse_tuple, TupleType};
use super::union::{parse_union, UnionType};
use super::SubType;
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
    UnionType(UnionType),
    StructType(StructType),
    SubType(SubType),
    TupleType(TupleType),
    RelationType(RelationType),
}

pub fn parse_type(input: Span) -> IResult<Span, Type> {
    alt((
        map(preceded(ws, parse_sub), Type::SubType),
        map(preceded(ws, parse_struct), Type::StructType),
        map(preceded(ws, parse_union), Type::UnionType),
        map(preceded(ws, parse_builtin), Type::BuiltinType),
        map(preceded(ws, parse_any), Type::AnyType),
        map(preceded(ws, parse_nil), Type::NilType),
        map(preceded(ws, parse_ref), Type::RefType),
        map(preceded(ws, parse_seq), Type::SeqType),
        map(preceded(ws, parse_set), Type::SetType),
        map(preceded(ws, parse_tuple), Type::TupleType),
    ))(input)
}

// Don't know how to do that without this duplication.
// The problem is if parse_union uses parse_type which tries to parse a union (stackoverflow)
pub fn parse_type_but_union(input: Span) -> IResult<Span, Type> {
    alt((
        map(preceded(ws, parse_sub), Type::SubType),
        map(preceded(ws, parse_struct), Type::StructType),
        map(preceded(ws, parse_builtin), Type::BuiltinType),
        map(preceded(ws, parse_any), Type::AnyType),
        map(preceded(ws, parse_nil), Type::NilType),
        map(preceded(ws, parse_ref), Type::RefType),
        map(preceded(ws, parse_seq), Type::SeqType),
        map(preceded(ws, parse_set), Type::SetType),
    ))(input)
}

// Don't know how to do that without this duplication.
// The problem is if parse_sub uses parse_type which tries to parse a sub (stackoverflow)
pub fn parse_subtypeable(input: Span) -> IResult<Span, Type> {
    alt((
        map(preceded(ws, parse_struct), Type::StructType),
        map(preceded(ws, parse_builtin), Type::BuiltinType),
        map(preceded(ws, parse_any), Type::AnyType),
        map(preceded(ws, parse_ref), Type::RefType),
        map(preceded(ws, parse_seq), Type::SeqType),
        map(preceded(ws, parse_set), Type::SetType),
        map(preceded(ws, parse_tuple), Type::TupleType),
        map(preceded(ws, parse_relation), Type::RelationType),
    ))(input)
}

#[test]
fn test_parse_type_nil() {
    // Nil (with spaces)
    assert_parse(
        parse_type(Span::new(" Nil")),
        Type::NilType(NilType {
            position: FilePosition { line: 1, column: 2 },
        }),
    );
}

#[test]
fn test_parse_type_ref() {
    // // Ref (with spaces)
    assert_parse(
        parse_type(Span::new(" Number")),
        Type::RefType(RefType {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 2 },
        }),
    );
}

#[test]
fn test_parse_type_seq() {
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

#[test]
fn test_parse_type_struct() {
    // // Seq (with spaces)
    assert_parse(
        parse_type(Span::new(" < Nil >")),
        Type::StructType(StructType {
            elements: vec![Type::NilType(NilType {
                position: FilePosition { line: 1, column: 4 },
            })],
            position: FilePosition { line: 1, column: 2 },
        }),
    );
}

// #[test]
// fn test_parse_type_sub() {
//     // Sub (with spaces)
//     assert_parse(
//         parse_type(Span::new(" Number(i | i > 0)")),
//         Type::SubType(SubType {
//             base: Box::new(Type::RefType(RefType {
//                 name: "Number".to_string(),
//                 position: FilePosition { line: 1, column: 2 }
//             })),
//             constraints: vec![

//             ],
//             position: FilePosition { line: 1, column: 2 },
//         },),
//     );

// }
