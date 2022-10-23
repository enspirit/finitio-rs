use nom::{
    character::complete::char,
    combinator::{map, opt},
    error::context,
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
use serde::{Serialize, Deserialize};

use crate::common::FilePosition;
use crate::fio::common::{parse_identifier, ws, Span};
use crate::fio::r#type::{parse_type, Type};

#[cfg(test)]
use crate::fio::common::assert_parse;
#[cfg(test)]

use super::RefType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Heading {
    pub attributes: Vec<Attribute>,
    pub position: FilePosition,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub att_type: Type,
    pub optional: bool,
    pub position: FilePosition,
}

fn parse_separator(input: Span) -> IResult<Span, char> {
    preceded(ws, char(','))(input)
}

fn parse_attribute(input: Span) -> IResult<Span, Attribute> {
    map(
        separated_pair(
            parse_identifier,
            preceded(ws, char(':')),
            pair(opt(terminated(char('?'), ws)), parse_type),
        ),
        |(name, (optional, att_type))| Attribute {
            name,
            att_type,
            optional: optional != None,
            position: input.into(),
        },
    )(input)
}

pub fn parse_heading(input: Span) -> IResult<Span, Heading> {
    map(parse_attributes, |attributes| Heading {
        attributes,
        position: input.into(),
    })(input)
}

fn parse_attributes(input: Span) -> IResult<Span, Vec<Attribute>> {
    context(
        "fields",
        delimited(
            char('{'),
            separated_list0(parse_separator, preceded(ws, parse_attribute)),
            preceded(ws, char('}')),
        ),
    )(input)
}

#[test]
fn test_parse_attribute() {
    assert_parse(
        parse_attribute(Span::new("name:String")),
        Attribute {
            name: "name".to_string(),
            position: FilePosition { line: 1, column: 1 },
            att_type: Type::RefType(RefType {
                name: "String".to_string(),
                position: FilePosition { line: 1, column: 6 },
            }),
            optional: false,
        },
    );
    assert_parse(
        parse_attribute(Span::new("name: String")),
        Attribute {
            name: "name".to_string(),
            position: FilePosition { line: 1, column: 1 },
            att_type: Type::RefType(RefType {
                name: "String".to_string(),
                position: FilePosition { line: 1, column: 7 },
            }),
            optional: false,
        },
    );
    assert_parse(
        parse_attribute(Span::new("name : String")),
        Attribute {
            name: "name".to_string(),
            position: FilePosition { line: 1, column: 1 },
            att_type: Type::RefType(RefType {
                name: "String".to_string(),
                position: FilePosition { line: 1, column: 8 },
            }),
            optional: false,
        },
    );
}

#[test]
fn test_parse_attribute_optional() {
    assert_parse(
        parse_attribute(Span::new("name :? String")),
        Attribute {
            name: "name".to_string(),
            position: FilePosition { line: 1, column: 1 },
            att_type: Type::RefType(RefType {
                name: "String".to_string(),
                position: FilePosition { line: 1, column: 9 },
            }),
            optional: true,
        },
    );
}

#[test]
fn test_parse_attributes_0() {
    let contents = ["{}", "{ }"];
    for content in contents.iter() {
        assert_parse(parse_attributes(Span::new(content)), vec![])
    }
}

#[test]
fn test_parse_attributes_simple() {
    assert_parse(
        parse_attributes(Span::new("{name: String}")),
        vec![Attribute {
            name: "name".to_string(),
            att_type: Type::RefType(RefType {
                name: "String".to_string(),
                position: FilePosition { line: 1, column: 8 },
            }),
            optional: false,
            position: FilePosition { line: 1, column: 2 },
        }],
    );
}

#[test]
fn test_parse_attributes_duo() {
    assert_parse(
        parse_attributes(Span::new("{name: String, age: Number}")),
        vec![
            Attribute {
                name: "name".to_string(),
                att_type: Type::RefType(RefType {
                    name: "String".to_string(),
                    position: FilePosition { line: 1, column: 8 },
                }),
                optional: false,
                position: FilePosition { line: 1, column: 2 },
            },
            Attribute {
                name: "age".to_string(),
                att_type: Type::RefType(RefType {
                    name: "Number".to_string(),
                    position: FilePosition {
                        line: 1,
                        column: 21,
                    },
                }),
                optional: false,
                position: FilePosition {
                    line: 1,
                    column: 16,
                },
            },
        ],
    );
}

#[test]
fn test_parse_attributes_spacing() {
    let heading = "{
      name :  String,
      age  :? Number
    }";
    assert_parse(
        parse_attributes(Span::new(heading)),
        vec![
            Attribute {
                name: "name".to_string(),
                att_type: Type::RefType(RefType {
                    name: "String".to_string(),
                    position: FilePosition {
                        line: 2,
                        column: 15,
                    },
                }),
                optional: false,
                position: FilePosition { line: 2, column: 7 },
            },
            Attribute {
                name: "age".to_string(),
                att_type: Type::RefType(RefType {
                    name: "Number".to_string(),
                    position: FilePosition {
                        line: 3,
                        column: 15,
                    },
                }),
                optional: true,
                position: FilePosition { line: 3, column: 7 },
            },
        ],
    );
}
