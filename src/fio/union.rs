#[cfg(test)]
use super::{any, builtin, nil, r#ref};
#[cfg(test)]
use crate::fio::common::assert_parse;

use super::{NilType, RefType, SeqType, Type};
use crate::common::FilePosition;
use crate::fio::common::Span;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{pair, preceded, terminated};
use nom::{bytes::complete::tag, combinator::map, sequence::delimited, IResult};
use serde::{Serialize, Deserialize};

use super::common::ws;
use super::r#type::{parse_type, parse_type_but_union};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UnionType {
    pub candidates: Vec<Type>,
    pub position: FilePosition,
}

pub fn parse_union(input: Span) -> IResult<Span, UnionType> {
    let first_elm = terminated(parse_type_but_union, preceded(ws, tag("|")));
    let alt_elms = separated_list0(
        delimited(ws, tag("|"), ws),
        preceded(ws, parse_type_but_union),
    );
    map(pair(first_elm, alt_elms), |(first, alt)| {
        let mut candidates: Vec<Type> = vec![first];
        candidates.extend(alt);
        UnionType {
            candidates: candidates,
            position: input.into(),
        }
    })(input)
}

#[test]
fn test_parse_union_simple() {
    assert_parse(
        parse_union(Span::new("Nil|Number")),
        UnionType {
            candidates: vec![
                Type::NilType(NilType {
                    position: FilePosition { line: 1, column: 1 },
                }),
                Type::RefType(RefType {
                    name: "Number".to_string(),
                    position: FilePosition { line: 1, column: 5 },
                }),
            ],
            position: FilePosition { line: 1, column: 1 },
        },
    );
}

#[test]
fn test_parse_union_spacing() {
    ///// Spacing
    assert_parse(
        parse_union(Span::new(" Nil |\n Number")),
        UnionType {
            candidates: vec![
                Type::NilType(NilType {
                    position: FilePosition { line: 1, column: 2 },
                }),
                Type::RefType(RefType {
                    name: "Number".to_string(),
                    position: FilePosition { line: 2, column: 2 },
                }),
            ],
            position: FilePosition { line: 1, column: 1 },
        },
    );
}

#[test]
fn test_parse_union_complex() {
    ///// With complex type
    assert_parse(
        parse_union(Span::new(" Nil |\n [Number]")),
        UnionType {
            candidates: vec![
                Type::NilType(NilType {
                    position: FilePosition { line: 1, column: 2 },
                }),
                Type::SeqType(SeqType {
                    position: FilePosition { line: 2, column: 2 },
                    elm_type: Box::new(Type::RefType(RefType {
                        name: String::from("Number"),
                        position: FilePosition { line: 2, column: 3 },
                    })),
                }),
            ],
            position: FilePosition { line: 1, column: 1 },
        },
    );
}
