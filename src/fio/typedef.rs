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

#[derive(Debug, PartialEq)]
pub struct TypeDef {
    pub name: String,
    pub target: Type,
    pub position: FilePosition
}

pub fn parse_right(input: Span) -> IResult<Span, Type> {
    alt((
        map(parse_base_type, Type::BaseType),
        map(parse_seq, Type::SeqType),
        map(parse_set, Type::SetType),
    ))
    (input)
}

pub fn parse_typedef(input: Span) -> IResult<Span, TypeDef> {
    let parser = separated_pair(
        parse_identifier,
        preceded(ws, tag("=")),
        preceded(ws, parse_right)
    );
    map(parser, |(name, right)| TypeDef {
        name: String::from(name),
        target: right,
        position: input.into()
    })(input)
}

#[test]
fn test_parse_right() {
    // Nil (with spaces)
    assert_parse(
        parse_right(Span::new(" Nil")),
        Type::BaseType(BaseType::Nil)
    );

    // Ref (with spaces)
    assert_parse(
        parse_right(Span::new(" Number")),
        Type::BaseType(BaseType::Ref(RefType {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 2 }
        }))
    );

    // Seq (with spaces)
    assert_parse(
        parse_right(Span::new("[ Number ]")),
        Type::SeqType(SeqType {
            elm_type: BaseType::Ref(RefType {
                name: String::from("Number"),
                position: FilePosition { line: 1, column: 3 }
            })
        })
    );
}

#[test]
fn test_parse_typedef() {

    // Aliasing nil
    assert_parse(
        parse_typedef(Span::new("Null = Nil")),
        TypeDef {
            name: String::from("Null"),
            target: Type::BaseType(BaseType::Nil),
            position: FilePosition { line: 1, column: 1 }
        }
    );

    // Aliasing builtin type
    assert_parse(
        parse_typedef(Span::new("Number = .Number")),
        TypeDef {
            name: String::from("Number"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::BaseType(BaseType::Builtin(BuiltinType {
                name: String::from("Number"),
                position: FilePosition { line: 1, column: 10 }
            }))
        }
    );

    // Aliasing ref type
    assert_parse(
        parse_typedef(Span::new("Integer = Number")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::BaseType(BaseType::Ref(RefType {
                name: String::from("Number"),
                position: FilePosition { line: 1, column: 11 }
            }))
        }
    );

    // A seq type
    assert_parse(
        parse_typedef(Span::new("Integer = [Number]")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::SeqType(SeqType {
                elm_type: BaseType::Ref(RefType {
                    name: String::from("Number"),
                    position: FilePosition { line: 1, column: 12 }
                })
            })
        }
    );

    // A set type
    assert_parse(
        parse_typedef(Span::new("Integer = {Number}")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::SetType(SetType {
                elm_type: BaseType::Ref(RefType {
                    name: String::from("Number"),
                    position: FilePosition { line: 1, column: 12 }
                })
            })
        }
    );

    ////// Spacing tests

    assert_parse(
        parse_typedef(Span::new("Integer=Number")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::BaseType(BaseType::Ref(RefType {
                name: String::from("Number"),
                position: FilePosition { line: 1, column: 9 }
            }))
        }
    );
    assert_parse(
        parse_typedef(Span::new("Integer\t=   \nNumber")),
        TypeDef {
            name: String::from("Integer"),
            position: FilePosition { line: 1, column: 1 },
            target: Type::BaseType(BaseType::Ref(RefType {
                name: String::from("Number"),
                position: FilePosition { line: 2, column: 1 }
            }))
        }
    );
}
