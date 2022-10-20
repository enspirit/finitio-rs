use serde::{Deserialize, Serialize};

use crate::{schema};

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeDef {
    any(),
    builtin {
        jsType: String
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    name: String,
    r#type: TypeDef
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Schema {
    pub types: Vec<Type>
}

impl Schema {

    pub fn from_schema(adt: &schema::Schema) {
        let mut schema = Self {
            types: Vec::new()
        };
        for (k, typedef) in &adt.types {
            match (typedef) {
                schema::TypeDef::AnyType(t) => {
                    schema.types.push(Type {
                        name: k.to_owned(),
                        r#type: TypeDef::any()
                    });
                },
                schema::TypeDef::BuiltinType(t) => {
                    schema.types.push(Type {
                        name: k.to_owned(),
                        r#type: TypeDef::builtin {
                            jsType: t.target.borrow_mut().target.clone()
                        }
                    });
                },
                _ => {}
            }
        }

        let j = serde_json::to_string(&schema).expect("oops");
        println!("{}", j);
    }

}
