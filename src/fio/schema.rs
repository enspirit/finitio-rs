#[cfg(test)]
use crate::{
    common::FilePosition,
    fio::{any, builtin, r#ref, Type},
};

use nom::{
    branch::alt,
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};
use serde::{Serialize, Deserialize};

use crate::fio::common::Span;
use crate::fio::errors::ParseError;

use super::{
    common::{ws, ws1, peol_comment},
    import::{parse_import, Import},
    typedef::{parse_typedef, TypeDef},
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub imports: Vec<Import>,
    pub type_defs: Vec<TypeDef>,
}

pub enum SchemaPart {
    Import(Import),
    TypeDef(TypeDef),
    Comment(String)
}

pub fn parse_schema(input: &str) -> Result<Schema, ParseError> {
    let span = Span::new(input);
    let result = parse_schema_content(span);
    match result {
        Ok((span, parts)) if span.fragment() == &"" => {
            let mut imports: Vec<Import> = Vec::new();
            let mut type_defs: Vec<TypeDef> = Vec::new();

            for part in parts {
                match part {
                    SchemaPart::Import(part) => imports.push(part),
                    SchemaPart::TypeDef(part) => type_defs.push(part),
                    SchemaPart::Comment(_) => {},
                }
            }
            Ok(Schema { imports, type_defs })
        }
        Ok((garbage, _)) => Err(ParseError::TrailingGarbage(garbage)),
        Err(error) => Err(ParseError::Nom(error)),
    }
}

fn parse_schema_part(input: Span) -> IResult<Span, SchemaPart> {
    alt((
        map(parse_import, SchemaPart::Import),
        map(parse_typedef, SchemaPart::TypeDef),
        map(peol_comment, SchemaPart::Comment)
    ))(input)
}

pub fn parse_schema_content(input: Span) -> IResult<Span, Vec<SchemaPart>> {
    preceded(ws, terminated(separated_list0(ws1, parse_schema_part), ws))(input)
}

#[test]
fn test_parse_schema() {
    let content = "
@import finitio/data

Number = .Number
Any = .
Integer = Number
  ";
    assert_eq!(
        parse_schema(content),
        Ok(Schema {
            imports: vec![Import {
                filename: "finitio/data".to_string(),
                position: FilePosition { line: 2, column: 9 },
            }],
            type_defs: vec![
                TypeDef {
                    name: String::from("Number"),
                    position: FilePosition { line: 4, column: 1 },
                    target: Type::BuiltinType(builtin::BuiltinType {
                        name: String::from("Number"),
                        position: FilePosition {
                            line: 4,
                            column: 10
                        }
                    })
                },
                TypeDef {
                    name: String::from("Any"),
                    target: Type::AnyType(any::AnyType {
                        position: FilePosition { line: 5, column: 7 }
                    }),
                    position: FilePosition { line: 5, column: 1 }
                },
                TypeDef {
                    name: String::from("Integer"),
                    position: FilePosition { line: 6, column: 1 },
                    target: Type::RefType(r#ref::RefType {
                        name: String::from("Number"),
                        position: FilePosition {
                            line: 6,
                            column: 11
                        }
                    })
                },
            ]
        })
    )
}
