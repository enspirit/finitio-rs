#[cfg(test)]
use super::{any, builtin, nil, r#ref};
#[cfg(test)]
use crate::fio::common::assert_parse;

use super::{Type, NilType, RefType, SeqType, UnionType};
use crate::common::FilePosition;
use crate::fio::common::Span;
use nom::multi::{separated_list1, separated_list0};
use nom::sequence::{preceded, terminated, pair};
use nom::{bytes::complete::tag, combinator::map, sequence::delimited, IResult};

use super::common::ws;
use super::r#type::{parse_type, parse_type_but_union};

#[derive(Debug, PartialEq)]
pub struct StructType {
    pub elements: Vec<Type>,
    pub position: FilePosition,
}

pub fn parse_struct(input: Span) -> IResult<Span, StructType> {
    let elms = separated_list1(
        delimited(ws, tag(","), ws),
        delimited(ws, parse_type, ws)
    );
    let parser = delimited(tag("<"), elms, tag(">"));
    map(
        parser,
        |elements| {
            StructType {
                elements,
                position: input.into(),
            }
        },
    )(input)
}

#[test]
fn test_parse_struct_simple() {
    assert_parse(
        parse_struct(Span::new("<Nil>")),
        StructType {
            elements: vec![
                Type::NilType(NilType {
                    position: FilePosition { line: 1, column: 2 },
                }),
            ],
            position: FilePosition { line: 1, column: 1 },
        },
    );
}

#[test]
fn test_parse_struct_duo() {
    assert_parse(
        parse_struct(Span::new("<Nil, Number>")),
        StructType {
            elements: vec![
                Type::NilType(NilType {
                    position: FilePosition { line: 1, column: 2 },
                }),
                Type::RefType(RefType {
                    name: "Number".to_string(),
                    position: FilePosition { line: 1, column: 7 },
                })
            ],
            position: FilePosition { line: 1, column: 1 },
        },
    );
}

#[test]
fn test_parse_struct_spacing() {
    ///// Spacing
    assert_parse(
        parse_struct(Span::new("< Nil ,\n Number >")),
        StructType {
            elements: vec![
                Type::NilType(NilType {
                    position: FilePosition { line: 1, column: 3 },
                }),
                Type::RefType(RefType {
                    name: "Number".to_string(),
                    position: FilePosition { line: 2, column: 2 },
                })
            ],
            position: FilePosition { line: 1, column: 1 },
        },
    );
}

#[test]
fn test_parse_struct_complex() {
    ///// With complex type
    assert_parse(
        parse_struct(Span::new("< Nil | Number, String>")),
        StructType {
            elements: vec![
                Type::UnionType(UnionType {
                    candidates: vec![
                        Type::NilType(NilType {
                            position: FilePosition { line: 1, column: 3 },
                        }),
                        Type::RefType(RefType {
                            name: "Number".to_string(),
                            position: FilePosition { line: 1, column: 9 },
                        })
                    ],
                    position: FilePosition { line: 1, column: 3 },
                }),
                Type::RefType(RefType {
                    name: "String".to_string(),
                    position: FilePosition { line: 1, column: 17 },
                })
            ],
            position: FilePosition { line: 1, column: 1 },
        },
    );

}