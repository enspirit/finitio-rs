
use super::heading::{parse_heading, Heading};
use crate::common::FilePosition;
use crate::fio::common::Span;
use nom::{combinator::map, IResult};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TupleType<'a> {
    pub heading: Heading<'a>,
    pub position: FilePosition,
}

pub fn parse_tuple(input: Span) -> IResult<Span, TupleType> {
    map(parse_heading, |heading| TupleType {
        heading,
        position: input.into(),
    })(input)
}

#[cfg(test)]
use super::{RefType, Type, heading::{Attribute}};
#[cfg(test)]
use crate::fio::common::assert_parse;

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
