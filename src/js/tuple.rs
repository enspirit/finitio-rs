use crate::schema::{TypeInclude, tuple::{Tuple, Attribute}};

impl TypeInclude<serde_json::Value> for Tuple {
    fn include(&self, v: &serde_json::Value) -> Result<bool, &'static str> {
        match v {
            serde_json::Value::Object(obj) => {
                // check for extra props
                let extras = obj.keys().filter(|prop| !self.attributes.contains_key(*prop));
                if extras.peekable().peek().is_some() {
                    return Err("The tuple has extra properties")
                }

                // check for missing props
                let missing = self.attributes.values().filter(|prop| {
                    !prop.optional && !obj.contains_key(&prop.name)
                });
                if missing.peekable().peek().is_some() {
                    return Err("The tuple is missing properties")
                }

                // validate all properties
                let invalid = obj.iter().filter(|(name, value)| {
                    let att = self.attributes.get(*name).unwrap();
                    let is_valid = att.att_type.include(value);
                    match is_valid {
                        Ok(_) => false,
                        Err(_) => true,
                    }
                });

                if invalid.peekable().peek().is_some() {
                    Err("Some properties have invalid values")
                } else {
                    Ok(true)
                }
            },
            _ => Err("Invalid source type")
        }
    }
}

#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use crate::schema::{any::Any, nil::Nil, r#ref::Ref, builtin::Builtin, r#type::Type, r#type::TypeRef, r#type::BuiltinRef};

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
        attributes,
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
