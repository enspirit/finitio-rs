use nom::{
    bytes::complete::{tag},
    character::complete::{char, newline},
    combinator::{map, opt},
    error::context,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, branch::alt,
};
use serde::{Serialize, Deserialize};

use crate::common::FilePosition;
use crate::fio::common::{parse_identifier, ws, Span};
use crate::fio::r#type::{parse_type, Type};

#[cfg(test)]
use crate::fio::common::assert_parse;
#[cfg(test)]

use super::RefType;
use super::{common::ws_no_nl, AnyType};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Heading {
    pub attributes: Vec<Attribute>,
    pub position: FilePosition,
    pub allow_extra: Option<Box<AllowExtra>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AllowExtra {
    pub extra_type: Type,
    pub position: FilePosition,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub name: String,
    pub att_type: Type,
    pub optional: bool,
    pub position: FilePosition,
}

fn parse_separator(input: Span) -> IResult<Span, char> {
    alt((
        preceded(ws_no_nl, preceded(opt(char(',')), newline)),
        preceded(ws_no_nl, char(',')),
    ))(input)
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

pub fn parse_attributes(input: Span) -> IResult<Span, (Vec<Attribute>, Option<AllowExtra>)> {
    context(
        "fields",
        delimited(
            char('{'),
            alt((
                pair(
                    separated_list1(parse_separator, preceded(ws, parse_attribute)),
                    preceded(parse_separator, preceded(ws, parse_extra))
                ),
                pair(
                    separated_list0(parse_separator, preceded(ws, parse_attribute)),
                    preceded(ws, parse_extra)
                ),
            )),
            preceded(ws, char('}')),
        ),
    )(input)
}

pub fn parse_heading(input: Span) -> IResult<Span, Heading> {
    map(parse_attributes, |(attributes, extra)| {
        let extra = match extra {
            Some(extra) => Some(Box::new(extra)),
            None => None,
        };
        Heading {
            attributes,
            position: input.into(),
            allow_extra: extra
        }
    })(input)
}

fn parse_extra(input: Span) -> IResult<Span, Option<AllowExtra>> {
    map(
        opt(preceded(
            tag("..."),
            delimited(
                ws,
                opt(preceded(char(':'), parse_type)),
                ws,
            )
        )),
        |extra| {
            match extra {
                Some(specification) => match specification {
                    None => Some(AllowExtra {
                        extra_type: Type::AnyType(AnyType { position: input.into() }),
                        position: input.into()
                    }),
                    Some(ftype) => Some(AllowExtra {
                        extra_type: ftype,
                        position: input.into()
                    }),
                },
                None => None,
            }
        }
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
        assert_parse(
            parse_attributes(Span::new(content)),
            (vec![], None)
        )
    }
}

#[test]
fn test_parse_attributes_simple() {
    assert_parse(
        parse_attributes(Span::new("{name: String}")),
        (
            vec![Attribute {
                name: "name".to_string(),
                att_type: Type::RefType(RefType {
                    name: "String".to_string(),
                    position: FilePosition { line: 1, column: 8 },
                }),
                optional: false,
                position: FilePosition { line: 1, column: 2 },
            }],
            None
        ),
    );
}

#[test]
fn test_parse_attributes_duo() {
    assert_parse(
        parse_attributes(Span::new("{name: String, age: Number}")),
        (
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
            None
        )
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
        (
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
            None
        )
    );
}

#[test]
fn test_parse_optional_comma_when_using_newline() {
    let heading = "{
      name :  String
      age  :? Number
    }";
    assert_parse(
        parse_attributes(Span::new(heading)),
        (
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
            None
        )
    );
}

#[test]
fn test_parse_allow_extra() {
    let extra = "";
    assert_parse(
        parse_extra(Span::new(extra)),
        None
    );

    let extra = "...";
    assert_parse(
        parse_extra(Span::new(extra)),
        Some(AllowExtra {
            position: FilePosition { line: 1, column: 1 },
            extra_type: Type::AnyType(AnyType {
                position: FilePosition { line: 1, column: 1 }
            })
        })
    );

    let extra = "...: String";
    assert_parse(
        parse_extra(Span::new(extra)),
        Some(AllowExtra {
            extra_type: Type::RefType(RefType {
                name: "String".to_string(),
                position: FilePosition {
                    line: 1,
                    column: 6,
                },
            }),
            position: FilePosition {
                line: 1,
                column: 1
            },
        })
    );
}

#[test]
fn test_parse_empty_heading() {
    let extra = "{}";
    assert_parse(
        parse_heading(Span::new(extra)),
        Heading {
            attributes: vec![],
            position: FilePosition {
                line: 1,
                column: 1,
            },
            allow_extra: None
        }
    );
}

#[test]
fn test_parse_heading_only_extra() {
    let extra = "{ ... }";
    assert_parse(
        parse_heading(Span::new(extra)),
        Heading {
            attributes: vec![],
            position: FilePosition {
                line: 1,
                column: 1,
            },
            allow_extra: Some(Box::new(AllowExtra {
                position: FilePosition { line: 1, column: 3 },
                extra_type: Type::AnyType(AnyType {
                    position: FilePosition { line: 1, column: 3 }
                })
            })),
        }
    );
}

#[test]
fn test_parse_empty_heading_with_typed_extra() {
    let extra = "{ ...: String }";
    assert_parse(
        parse_heading(Span::new(extra)),
        Heading {
            attributes: vec![],
            position: FilePosition {
                line: 1,
                column: 1,
            },
            allow_extra: Some(Box::new(AllowExtra {
                position: FilePosition { line: 1, column: 3 },
                extra_type: Type::RefType(RefType {
                    name: "String".to_string(),
                    position: FilePosition { line: 1, column: 8 }
                })
            })),
        }
    );
}

#[test]
fn test_parse_non_empty_heading() {
    let extra = "{ name: String }";
    assert_parse(
        parse_heading(Span::new(extra)),
        Heading {
            attributes: vec![
                Attribute {
                    name: "name".to_string(),
                    position: FilePosition { line: 1, column: 3 },
                    att_type: Type::RefType(RefType {
                        name: "String".to_string(),
                        position: FilePosition { line: 1, column: 9 },
                    }),
                    optional: false,
                },
            ],
            position: FilePosition {
                line: 1,
                column: 1,
            },
            allow_extra: None
        }
    );
}

#[test]
fn test_parse_heading_with_any_extra() {
    let extra = "{\n  name: String,\n  ... }";
    assert_parse(
        parse_heading(Span::new(extra)),
        Heading {
            attributes: vec![
                Attribute {
                    name: "name".to_string(),
                    position: FilePosition { line: 2, column: 3 },
                    att_type: Type::RefType(RefType {
                        name: "String".to_string(),
                        position: FilePosition { line: 2, column: 9 },
                    }),
                    optional: false,
                },
            ],
            position: FilePosition {
                line: 1,
                column: 1,
            },
            allow_extra: Some(Box::new(AllowExtra {
                position: FilePosition { line: 3, column: 3 },
                extra_type: Type::AnyType(AnyType {
                    position: FilePosition { line: 3, column: 3 }
                })
            })),
        }
    );
}

#[test]
fn test_parse_heading_with_string_extra() {
    let extra = "{\n  name: String,\n  ...: String }";
    assert_parse(
        parse_heading(Span::new(extra)),
        Heading {
            attributes: vec![
                Attribute {
                    name: "name".to_string(),
                    position: FilePosition { line: 2, column: 3 },
                    att_type: Type::RefType(RefType {
                        name: "String".to_string(),
                        position: FilePosition { line: 2, column: 9 },
                    }),
                    optional: false,
                },
            ],
            position: FilePosition {
                line: 1,
                column: 1,
            },
            allow_extra: Some(Box::new(AllowExtra {
                position: FilePosition { line: 3, column: 3 },
                extra_type: Type::RefType(RefType {
                    name: "String".to_string(),
                    position: FilePosition { line: 3, column: 8 },
                }),
            })),
        }
    );
}

#[test]
fn test_parse_heading_inline_with_extra() {
    let extra = "{ name: String, ... }";
    assert_parse(
        parse_heading(Span::new(extra)),
        Heading {
            attributes: vec![
                Attribute {
                    name: "name".to_string(),
                    position: FilePosition { line: 1, column: 3 },
                    att_type: Type::RefType(RefType {
                        name: "String".to_string(),
                        position: FilePosition { line: 1, column: 9 },
                    }),
                    optional: false,
                },
            ],
            position: FilePosition {
                line: 1,
                column: 1,
            },
            allow_extra: Some(Box::new(AllowExtra {
                position: FilePosition { line: 1, column: 17 },
                extra_type: Type::AnyType(AnyType {
                    position: FilePosition { line: 1, column: 17 }
                })
            })),
        }
    );
}

#[test]
fn test_parse_heading_inline_with_string_extra() {
    let extra = "{ name: String, ...: String }";
    assert_parse(
        parse_heading(Span::new(extra)),
        Heading {
            attributes: vec![
                Attribute {
                    name: "name".to_string(),
                    position: FilePosition { line: 1, column: 3 },
                    att_type: Type::RefType(RefType {
                        name: "String".to_string(),
                        position: FilePosition { line: 1, column: 9 },
                    }),
                    optional: false,
                },
            ],
            position: FilePosition {
                line: 1,
                column: 1,
            },
            allow_extra: Some(Box::new(AllowExtra {
                position: FilePosition { line: 1, column: 17 },
                extra_type: Type::RefType(RefType {
                    name: "String".to_string(),
                    position: FilePosition { line: 1, column: 22 },
                }),
            })),
        }
    );
}
