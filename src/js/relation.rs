use std::collections::{HashMap};
use serde_hashkey::{to_key};
use snafu::{Whatever, whatever, ResultExt};

use crate::schema::{TypeInclude, relation::{Relation}};

impl TypeInclude<serde_json::Value> for Relation {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        match v {
            serde_json::Value::Array(arr) => {
                let mut values = HashMap::new();

                for (pos, row) in arr.iter().enumerate() {
                    let key = to_key(row).unwrap();

                    self.heading.include(row)
                        .with_whatever_context(|_| format!("Set contains invalid value at index {}", pos))?;

                    match values.insert(key.clone(), row) {
                        None => {},
                        Some(_) => {
                            whatever!("Relation contains duplicated tuple: {}", row)
                        }
                    }
                }
                Ok(())
            },
            v => whatever!("Invalid value for Relation: {}", v)
        }
    }
}

#[cfg(test)]
use crate::schema::{any::Any, heading::{Heading, Attribute}, builtin::Builtin, r#type::Type};

#[test]
fn test_include_tuple() {
    use crate::common::FilePosition;

    let position = FilePosition { line: 2, column: 2};

    let any_t = Type::Any(Any {
        position: position.clone()
    });
    let builtin_str = Type::Builtin(Builtin {
        position: position.clone(),
        target: String::from("String")
    });
    let builtin_num = Type::Builtin(Builtin {
        position: position.clone(),
        target: String::from("Number")
    });

    // Tuple { name: .String, age: .Number, extra: Any }
    let attributes = HashMap::from([
        ("name".to_string(), Attribute {
            name: String::from("name"),
            att_type: builtin_str,
            optional: false
        }),
        ("age".to_string(), Attribute {
            name: String::from("age"),
            att_type: builtin_num,
            optional: false
        }),
        ("extra".to_string(), Attribute {
            name: String::from("extra"),
            att_type: any_t,
            optional: true
        }),
    ]);
    let relation = Type::Relation(Relation {
        heading: Heading {
            attributes,
            position: position.clone()
        },
        position: position.clone()
    });

    // invalid: row is missing properties
    let missing_props = serde_json::json!([{}]);
    assert_eq!(relation.include(&missing_props).is_ok(), false, "obj with missing props is not valid");

    // invalid: extra properties
    let extra_props = serde_json::json!([
        { "name": "Foo", "age": 22, "extra": "Bar", "invalid": "This is not covered by our tuple" }
    ]);
    assert_eq!(relation.include(&extra_props).is_ok(), false, "obj with extra props is not valid");

    // invalid: wrong types
    let extra_props = serde_json::json!([
        { "name": "Foo", "age": "22", "extra": "Bar" }
    ]);
    assert_eq!(relation.include(&extra_props).is_ok(), false, "obj with invalid types is not valid");

    // Valid
    let valid = serde_json::json!([
        { "name": "Foo", "age": 22, "extra": "foo" },
        { "name": "Bar", "age": 42, "extra": "bar" }
    ]);
    assert_eq!(relation.include(&valid).is_ok(), true, "obj with valid props&types is valid");

    // Valid (optional prop missing)
    let valid = serde_json::json!([
        { "name": "Foo", "age": 22 },
        { "name": "Bar", "age": 22, "extra": "bar" }
    ]);
    assert_eq!(relation.include(&valid).is_ok(), true, "obj with missing optional props is valid");

}
