#[cfg(test)]
use crate::fio::common::assert_parse;

use crate::fio::common::{parse_identifier, Span};
use crate::fio::base::{parse_base_type, BaseType, RefType};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    error::context,
    multi::{many0, separated_list0},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use super::base::BuiltinType;
use super::common::ws;

#[derive(Debug, PartialEq)]
pub enum Type {
    BaseType(BaseType),
}

#[derive(Debug, PartialEq)]
pub struct TypeDef {
    pub name: String,
    pub target: Type
}

pub fn parse_right(input: Span) -> IResult<Span, Type> {
    map(preceded(ws, parse_base_type), Type::BaseType)(input)
}

pub fn parse_typedef(input: Span) -> IResult<Span, TypeDef> {
    let parser = separated_pair(
        parse_identifier,
        preceded(ws, tag("=")),
        parse_right
    );
    map(parser, |(name, right)| TypeDef {
        name: String::from(name),
        target: right
    })(input)
}

#[test]
fn test_parse_right() {
    assert_parse(
        parse_right(Span::new(" Nil")),
        Type::BaseType(BaseType::Nil)
    );
    assert_parse(
        parse_right(Span::new(" Number")),
        Type::BaseType(BaseType::Ref(RefType {
            name: String::from("Number")
        }))
    );
    assert_parse(
        parse_right(Span::new(" .Number")),
        Type::BaseType(BaseType::Builtin(BuiltinType {
            name: String::from("Number")
        }))
    );
}

#[test]
fn test_parse_typedef() {

    // Aliasing nil
    assert_parse(
        parse_typedef(Span::new("Null = Nil")),
        TypeDef {
            name: String::from("Null"),
            target: Type::BaseType(BaseType::Nil)
        }
    );

    // Aliasing builtin type
    assert_parse(
        parse_typedef(Span::new("Number = .Number")),
        TypeDef {
            name: String::from("Number"),
            target: Type::BaseType(BaseType::Builtin(BuiltinType {
                name: String::from("Number")
            }))
        }
    );

    // Aliasing ref type
    assert_parse(
        parse_typedef(Span::new("Integer = Number")),
        TypeDef {
            name: String::from("Integer"),
            target: Type::BaseType(BaseType::Ref(RefType {
                name: String::from("Number")
            }))
        }
    );

    ////// Spacing tests

    assert_parse(
        parse_typedef(Span::new("Integer=Number")),
        TypeDef {
            name: String::from("Integer"),
            target: Type::BaseType(BaseType::Ref(RefType {
                name: String::from("Number")
            }))
        }
    );
    assert_parse(
        parse_typedef(Span::new("Integer\t=   \nNumber")),
        TypeDef {
            name: String::from("Integer"),
            target: Type::BaseType(BaseType::Ref(RefType {
                name: String::from("Number")
            }))
        }
    );
}
