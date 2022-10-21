use super::heading::{parse_heading, Attribute, Heading};
#[cfg(test)]
use super::{any, builtin, nil, r#ref};
#[cfg(test)]
use crate::fio::common::assert_parse;

use super::{NilType, RefType, SeqType, Type, UnionType};
use crate::common::FilePosition;
use crate::fio::common::Span;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{pair, preceded, terminated};
use nom::{bytes::complete::tag, combinator::map, sequence::delimited, IResult};
use serde::{Serialize, Deserialize};

use super::common::ws;
use super::r#type::{parse_type, parse_type_but_union};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TupleType {
    pub heading: Heading,
    pub position: FilePosition,
}

pub fn parse_tuple(input: Span) -> IResult<Span, TupleType> {
    map(parse_heading, |heading| TupleType {
        heading,
        position: input.into(),
    })(input)
}

#[test]
fn test_parse_tuple_simple() {
    assert_parse(
        parse_tuple(Span::new("{ name: String, age:? Number}")),
        TupleType {
            heading: Heading {
                attributes: vec![
                    Attribute {
                        name: "name".to_string(),
                        att_type: Type::RefType(RefType {
                            name: "String".to_string(),
                            position: FilePosition { line: 1, column: 9 },
                        }),
                        optional: false,
                        position: FilePosition { line: 1, column: 3 },
                    },
                    Attribute {
                        name: "age".to_string(),
                        att_type: Type::RefType(RefType {
                            name: "Number".to_string(),
                            position: FilePosition {
                                line: 1,
                                column: 23,
                            },
                        }),
                        optional: true,
                        position: FilePosition {
                            line: 1,
                            column: 17,
                        },
                    },
                ],
                position: FilePosition { line: 1, column: 1 },
            },
            position: FilePosition { line: 1, column: 1 },
        },
    );
}
