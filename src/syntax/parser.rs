extern crate pest;
use pest::iterators::Pairs;
use ::pest::{Parser, iterators::Pair};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};

use super::FilePosition;

#[derive(Parser)]
#[grammar = "syntax/finitio.pest"]
pub struct FinitioParser;

#[derive(Clone, Debug)]
pub struct Metadata {
    position: FilePosition,
    content: String
}

#[derive(Clone, Debug)]
pub struct Attribute {
    position: FilePosition,
    metadata: Option<Metadata>,
    name: String,
    optional: bool,
    attr_type: Box<FinitioType>
}

#[derive(Clone, Debug)]
pub struct Heading {
    position: FilePosition,
    attributes: Vec<Attribute>,
    extra_type: Option<Box<FinitioType>>
}

#[derive(Clone, Debug)]
pub struct TypeDef {
    position: FilePosition,
    metadata: Option<Metadata>,
    name: String,
    r#type: FinitioType
}

#[derive(Clone, Debug)]
pub struct Import {
    position: FilePosition,
    uri: String,
}

#[derive(Clone, Debug)]
pub struct Schema {
    path: String,
    imports: Vec<Import>,
    types: Vec<TypeDef>
}


#[derive(Clone, Debug)]
pub enum FinitioType {
    Nil {
        position: FilePosition
    },
    Any {
        position: FilePosition
    },
    Builtin {
        position: FilePosition,
        name: String
    },
    Ref {
        position: FilePosition,
        name: String
    },
    Seq {
        position: FilePosition,
        elm_type: Box<FinitioType>
    },
    Set {
        position: FilePosition,
        elm_type: Box<FinitioType>
    },
    Struct {
        position: FilePosition,
        elements: Vec<FinitioType>
    },
    Union {
        position: FilePosition,
        types: Vec<FinitioType>
    },
    Sub {
        position: FilePosition,
        base: Box<FinitioType>
    },
    Tuple {
        position: FilePosition,
        heading: Heading
    },
    Rel {
        position: FilePosition,
        heading: Heading
    }
}


fn to_attribute(pair: Pair<Rule>) -> Attribute {
    let mut name: Option<String> = None;
    let mut metadata: Option<Metadata> = None;
    let mut attr_type: Option<FinitioType> = None;
    let mut optional = false;

    let position = FilePosition {
        start: pair.as_span().start_pos().line_col(),
        end: pair.as_span().end_pos().line_col(),
    };

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::metadata => {
                metadata = Some(Metadata{
                    position: FilePosition {
                        start: pair.as_span().start_pos().line_col(),
                        end: pair.as_span().end_pos().line_col(),
                    },
                    content: pair.as_str().to_string()
                })
            },
            Rule::attribute_def => {
                optional = pair.as_str().to_string().eq(":?");
            },
            Rule::ident => {
                name = Some(pair.as_str().to_string())
            },
            Rule::fio_type|Rule::uniontype => {
                attr_type = Some(to_type(pair));
            },
            _ => {
                panic!("Should not process {:?} as this stage", pair.as_rule());
            }
        }
    }
    Attribute {
        position,
        metadata,
        optional,
        name: name.expect("Should have had an ident pair"),
        attr_type: Box::new(attr_type.expect("Should have had a type"))
    }
}

fn to_heading(pair: Pair<Rule>) -> Heading {
    let mut attributes: Vec<Attribute> = vec![];
    let mut extra_type: Option<Box<FinitioType>> = None;

    let position = FilePosition {
        start: pair.as_span().start_pos().line_col(),
        end: pair.as_span().end_pos().line_col(),
    };

    for part in pair.into_inner() {
        let position = FilePosition {
            start: part.as_span().start_pos().line_col(),
            end: part.as_span().end_pos().line_col(),
        };

        match part.as_rule() {
            Rule::attribute => {
                attributes.push(to_attribute(part));
            },
            Rule::heading_extra => {
                extra_type = match part.into_inner().next() {
                    Some(t) => Some(Box::new(to_type(t))),
                    None => Some(Box::new(FinitioType::Any{
                        position,
                    }))
                };
            }
            _ => {
                panic!("Should not process {:?} as this stage", part.as_rule());
            }
        }
    }

    Heading {
        position,
        attributes,
        extra_type
    }
}

fn to_type(pair: Pair<Rule>) -> FinitioType {
    let position = FilePosition {
        start: pair.as_span().start_pos().line_col(),
        end: pair.as_span().end_pos().line_col(),
    };

    match pair.as_rule() {
        Rule::fio_type => {
            to_type(pair.into_inner().next().unwrap())
        },
        Rule::anytype => {
            FinitioType::Any {
                position,
            }
        },
        Rule::niltype => {
            FinitioType::Nil {
                position,
            }
        },
        Rule::uniontype => {
            FinitioType::Union {
                position,
                types: pair.into_inner().map(|e| {
                    to_type(e)
                }).collect()
            }
        },
        Rule::builtintype => {
            FinitioType::Builtin {
                position,
                // getting rid of the dot in front of ident
                name: pair.as_str()[1..].to_string()
            }
        },
        Rule::reftype => {
            FinitioType::Ref {
                position,
                name: pair.as_str().to_string()
            }
        },
        Rule::seqtype => {
            FinitioType::Seq {
                position,
                elm_type: Box::new(to_type(pair.into_inner().next().unwrap()))
            }
        },
        Rule::structtype => {
            FinitioType::Struct {
                position,
                elements: pair.into_inner().map(|e| {
                    to_type(e)
                }).collect()
            }
        },
        Rule::settype => {
            FinitioType::Set {
                position,
                elm_type: Box::new(to_type(pair.into_inner().next().unwrap()))
            }
        },
        Rule::subtype => {
            FinitioType::Sub {
                position,
                base: Box::new(to_type(pair.into_inner().next().unwrap()))
            }
        },
        Rule::tupletype => {
            FinitioType::Tuple {
                position,
                heading: to_heading(pair.into_inner().next().unwrap())
            }
        },
        Rule::reltype => {
            FinitioType::Rel {
                position,
                heading: to_heading(pair.into_inner().next().unwrap())
            }
        },
        _ => {
            panic!("Should not process {:?} as this stage", pair.as_rule());
        }
    }
}

fn to_typedef(pairs: Pairs<Rule>, position: FilePosition) -> TypeDef {
    let mut metadata: Option<Metadata> = None;
    let mut name: Option<String> = None;
    let mut r#type: Option<FinitioType> = None;

    for pair in pairs {
        // dbg!("{:?}", pair.as_span());
        match pair.as_rule() {
            Rule::ident => {
                name = Some(pair.as_str().to_string())
            },
            Rule::fio_type => {
                let t = to_type(pair.into_inner().next().unwrap());
                r#type = Some(t)
            },
            Rule::uniontype => {
                let t = to_type(pair);
                r#type = Some(t)
            },
            Rule::metadata => {
                metadata = Some(Metadata {
                    position: FilePosition {
                        start: pair.as_span().start_pos().line_col(),
                        end: pair.as_span().end_pos().line_col(),
                    },
                    content: pair.as_str().to_string()
                }
            );
            },
            _ => {
                panic!("Should not process {:?} as this stage", pair.as_rule());
            }
        }
    }

    TypeDef {
        position,
        metadata,
        name: name.unwrap(),
        r#type: r#type.unwrap(),
    }
}

fn parse_schema(path: &Path) -> Schema {
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let pairs = FinitioParser::parse(Rule::fio, &unparsed_file)
        .expect("Failed to parse"); // get and unwrap the `file` rule; never fails

    let mut typedefs: Vec<TypeDef> = vec!();
    let mut imports: Vec<Import> = vec!();

    for pair in pairs {
        match pair.as_rule() {
            Rule::fio_imports => {
                let paths: Vec<Import> = pair.into_inner().map(|p| {
                    Import {
                        position: FilePosition {
                            start: p.as_span().start_pos().line_col(),
                            end: p.as_span().end_pos().line_col(),
                        },
                        uri: p.into_inner().as_str().to_string()
                    }
                }).collect();
                imports.extend(paths.iter().cloned());
            },
            Rule::fio_typedefs => {
                let pairs = pair.into_inner();
                for pair in pairs {
                    let position = FilePosition {
                        start: pair.as_span().start_pos().line_col(),
                        end: pair.as_span().end_pos().line_col(),
                    };

                    typedefs.push(to_typedef(pair.into_inner(), position));
                }
            }
            _ => {},
        }
    }

    Schema {
        path: path.to_str().unwrap().to_string(),
        imports,
        types: typedefs
    }
}

#[derive(Clone, Debug)]
pub struct System {
    schemas: HashMap<String, Schema>
}

pub fn resolve_import_uri(path: &Path, importer: &Path, root: &Path) -> PathBuf {
    if path.is_relative() {
        return importer.join(path).canonicalize().unwrap();
    }
    return root.join(path).canonicalize().unwrap();
}

pub fn parse_system(path: &str) -> System {
    let entry_path = Path::new(path).canonicalize().unwrap();
    let entry_point = parse_schema(&entry_path);

    let context = Path::new(path).parent().expect("Could not find dirname from entry point");
    let mut schemas: HashMap<String, Schema> = HashMap::new();
    schemas.insert(entry_path.to_str().unwrap().to_string(), entry_point.clone());

    let mut to_parse: Vec<(String, &Path)> = entry_point.imports.into_iter().map(|i| {
        (i.uri, context)
    }).collect();

    while !to_parse.is_empty() {
        let (import, importer) = to_parse.pop().unwrap();

        let already_parsed = schemas.contains_key(&import);
        if already_parsed {
            break;
        }

        let importee = Path::new(import.as_str());
        let import_path = resolve_import_uri(&importee, importer, context);

        let schema = parse_schema(&import_path);
        schemas.insert(import, schema.clone());

        let extra_imports: Vec<(String, &Path)> = schema.imports.into_iter().map(|i| {
            (i.uri, context)
        }).collect();

        to_parse.extend(extra_imports);
    }

    System {
        schemas
    }
}

pub fn resolve_system(system: System) {

}

pub fn main() {
    let system = parse_system("examples/index.fio");
    println!("{:?}", system);
}
