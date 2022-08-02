#[cfg(test)]
use crate::{common::FilePosition, fio::{typedef::Type, base::BuiltinType}};

use nom::{
    branch::alt,
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};

use crate::fio::common::Span;
use crate::fio::errors::ParseError;

use super::{
    common::{ws, ws1},
    import::{parse_import, Import}, typedef::{parse_typedef, TypeDef},
};

#[derive(Debug, PartialEq)]
pub struct Schema {
    pub imports: Vec<Import>,
    pub type_defs: Vec<TypeDef>,
}

pub enum SchemaPart {
    Import(Import),
    TypeDef(TypeDef),
}

pub fn parse_schema(input: &str) -> Result<Schema, ParseError> {
    let span = Span::new(input);
    let result = parse_schema_content(span);
    match result {
        Ok((span, parts)) if span.fragment() == &"" => {
            let mut Imports: Vec<Import> = Vec::new();
            let mut TypeDefs: Vec<TypeDef> = Vec::new();

            for part in parts {
                match part {
                    SchemaPart::Import(part) => Imports.push(part),
                    SchemaPart::TypeDef(part) => TypeDefs.push(part),
                }
            }
            Ok(Schema {
                imports: Imports,
                type_defs: TypeDefs
            })
        }
        Ok((garbage, _)) => Err(ParseError::TrailingGarbage(garbage)),
        Err(error) => Err(ParseError::Nom(error)),
    }
}

fn parse_schema_part(input: Span) -> IResult<Span, SchemaPart> {
    alt((
        map(parse_import, SchemaPart::Import),
        map(parse_typedef, SchemaPart::TypeDef),
    ))(input)
}

pub fn parse_schema_content(input: Span) -> IResult<Span, Vec<SchemaPart>> {
    preceded(ws, terminated(separated_list0(ws1, parse_schema_part), ws))(input)
}

#[test]
fn test_parse_schema() {
    use crate::fio::base::{BaseType};
    let content = "
      @import finitio/data

      Number = .Number
  ";
    assert_eq!(
        parse_schema(content),
        Ok(Schema {
            imports: vec![Import {
                filename: "finitio/data".to_string(),
                position: FilePosition {
                    line: 2,
                    column: 15
                },
            }],
            type_defs: vec![TypeDef {
                name: String::from("Number"),
                target: Type::BaseType(BaseType::Builtin(BuiltinType {
                    name: String::from("Number")
                }))
            }]
        })
    )
}
