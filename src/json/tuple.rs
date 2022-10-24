use snafu::{Whatever, whatever, ResultExt};

use crate::schema::{FinitioType, tuple::{Tuple}};

impl FinitioType<serde_json::Value> for Tuple {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        match v {
            serde_json::Value::Object(_obj) => {
                self.heading.include(v)
                    .with_whatever_context(|_| format!("Invalid tuple: {}", v))
            },
            v => whatever!("Invalid source type for Tuple: {}", v)
        }
    }
    fn dress(&self, value: &serde_json::Value) -> Result<serde_json::Value, Whatever> {
        self.include(value)?;
        Ok(value.clone())
    }
}

#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use crate::schema::{any::Any, builtin::Builtin, r#type::Type, heading::{Heading, Attribute}};

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
    let tuple = Type::Tuple(Tuple {
        heading: Heading {
            attributes,
            position: position.clone()
        },
        position: position.clone()
    });

    // invalid: missing properties
    let missing_props = serde_json::json!({});
    assert_eq!(tuple.include(&missing_props).is_ok(), false, "obj with missing props is not valid");

    // invalid: extra properties
    let extra_props = serde_json::json!({ "name": "Foo", "age": 22, "extra": "Bar", "invalid": "This is not covered by our tuple" });
    assert_eq!(tuple.include(&extra_props).is_ok(), false, "obj with extra props is not valid");

    // invalid: wrong types
    let extra_props = serde_json::json!({ "name": "Foo", "age": "22", "extra": "Bar" });
    assert_eq!(tuple.include(&extra_props).is_ok(), false, "obj with invalid types is not valid");

    // Valid
    let valid = serde_json::json!({ "name": "Foo", "age": 22, "extra": "Bar" });
    assert_eq!(tuple.include(&valid).is_ok(), true, "obj with valid props&types is valid");

    // Valid (optional prop missing)
    let valid = serde_json::json!({ "name": "Foo", "age": 22 });
    assert_eq!(tuple.include(&valid).is_ok(), true, "obj with missing optional props is valid");

}
