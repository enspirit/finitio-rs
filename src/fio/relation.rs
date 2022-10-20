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
pub struct RelationType {
    pub heading: Heading,
    pub position: FilePosition,
}

pub fn parse_relation(input: Span) -> IResult<Span, RelationType> {
    map(delimited(tag("{"), parse_heading, tag("}")), |heading| {
        RelationType {
            heading,
            position: input.into(),
        }
    })(input)
}

#[test]
fn test_parse_relation_simple() {
    assert_parse(
        parse_relation(Span::new("{{ name: String, age:? Number }}")),
        RelationType {
            heading: Heading {
                attributes: vec![
                    Attribute {
                        name: "name".to_string(),
                        att_type: Type::RefType(RefType {
                            name: "String".to_string(),
                            position: FilePosition {
                                line: 1,
                                column: 10,
                            },
                        }),
                        optional: false,
                        position: FilePosition { line: 1, column: 4 },
                    },
                    Attribute {
                        name: "age".to_string(),
                        att_type: Type::RefType(RefType {
                            name: "Number".to_string(),
                            position: FilePosition {
                                line: 1,
                                column: 24,
                            },
                        }),
                        optional: true,
                        position: FilePosition {
                            line: 1,
                            column: 18,
                        },
                    },
                ],
                position: FilePosition { line: 1, column: 2 },
            },
            position: FilePosition { line: 1, column: 1 },
        },
    );
}