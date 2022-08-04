#[cfg(test)]
use crate::fio::{
    any, builtin, common::assert_parse, nil, r#ref, r#ref::RefType, seq::SeqType, set::SetType,
};

use crate::fio::r#type::{parse_type, Type};
use crate::{
    common::FilePosition,
    fio::common::{parse_identifier, Span},
};

use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

use super::common::ws;
#[derive(Debug, PartialEq)]
pub struct TypeDef {
    pub name: String,
    pub target: Type,
    pub position: FilePosition,
}

pub fn parse_typedef(input: Span) -> IResult<Span, TypeDef> {
    let parser = separated_pair(
        parse_identifier,
        preceded(ws, tag("=")),
        preceded(
            ws,
            parse_type
        ),
    );
    map(parser, |(name, right)| TypeDef {
        name: String::from(name),
        target: right,
        position: input.into(),
    })(input)
}

#[test]
fn test_parse_typedef() {
    // Aliasing nil
    assert_parse(
        parse_typedef(Span::new("Null = Nil")),
        TypeDef {
            name: String::from("Null"),
            target: Type::NilType(nil::NilType {
                position: FilePosition { line: 1, column: 8 },
            }),
            position: FilePosition { line: 1, column: 1 },
        },
    );

    // Aliasing any
    assert_parse(
        parse_typedef(Span::new("Any = .")),
        TypeDef {
            name: String::from("Any"),
            target: Type::AnyType(any::AnyType {
                position: FilePosition { line: 1, column: 7 },
            }),
            position: FilePosition { line: 1, column: 1 },
        },
    );

    // Aliasing builtin type
    assert_parse(
        parse_typedef(Span::new("Number = .Number")),
        TypeDef {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::BuiltinType(builtin::BuiltinType {
                name: String::from("Number"),
                position: FilePosition {
                    line: 1,
                    column: 10,
                },
            }),
        },
    );

    // // Aliasing ref type
    assert_parse(
        parse_typedef(Span::new("Integer = Number")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::RefType(r#ref::RefType {
                name: String::from("Number"),
                position: FilePosition {
                    line: 1,
                    column: 11,
                },
            }),
        },
    );

    // // A seq type
    assert_parse(
        parse_typedef(Span::new("Integer = [Number]")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::SeqType(SeqType {
                position: FilePosition {
                    line: 1,
                    column: 11,
                },
                elm_type: Box::new(Type::RefType(RefType {
                    name: String::from("Number"),
                    position: FilePosition {
                        line: 1,
                        column: 12,
                    },
                })),
            }),
        },
    );

    // // A set type
    assert_parse(
        parse_typedef(Span::new("Integer = {Number}")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::SetType(SetType {
                position: FilePosition {
                    line: 1,
                    column: 11,
                },
                elm_type: Box::new(Type::RefType(RefType {
                    name: String::from("Number"),
                    position: FilePosition {
                        line: 1,
                        column: 12,
                    },
                })),
            }),
        },
    );

    // ////// Spacing tests

    assert_parse(
        parse_typedef(Span::new("Integer=Number")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::RefType(r#ref::RefType {
                name: String::from("Number"),
                position: FilePosition { line: 1, column: 9 },
            }),
        },
    );
    assert_parse(
        parse_typedef(Span::new("Integer\t=   \nNumber")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::RefType(r#ref::RefType {
                name: String::from("Number"),
                position: FilePosition { line: 2, column: 1 },
            }),
        },
    );
}
