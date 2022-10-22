use std::collections::{HashSet, HashMap};
use serde_hashkey::{from_key, to_key, Error, Key};
use snafu::{Whatever, whatever, ResultExt};
use crate::schema::{TypeInclude, set::Set};

impl TypeInclude<serde_json::Value> for Set {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        match v {
            serde_json::Value::Array(a) => {
                let mut values = HashMap::new();

                for (pos, value) in a.iter().enumerate() {
                    let key = to_key(value).unwrap();

                    self.elm_type.include(value)
                        .with_whatever_context(|_| format!("Set contains invalid value at index {}", pos))?;

                    match values.insert(key.clone(), value) {
                        None => {},
                        Some(val) => {
                            whatever!("Set contains duplicated value: {}", value)
                        }
                    }
                }
                Ok(())
            },
            v => whatever!("Not an array: {}", v),
        }
    }
}

#[cfg(test)]
use crate::schema::{any::Any, r#type::Type, builtin::Builtin};
#[test]
fn test_include_set() {
    use crate::common::FilePosition;

    let position = FilePosition { line: 2, column: 2};
    let any = Type::Any(Any { position: position.clone() });
    let builtin_str = Type::Builtin(Builtin {
        position: position.clone(),
        target: String::from("String")
    });

    // Set of .String
    let set = Set {
        position: position.clone(),
        elm_type: Box::new(builtin_str)
    };

    let nil = serde_json::Value::Null {};
    assert_eq!(set.include(&nil).is_ok(), false, "Nil is not valid for Set");

    let number = serde_json::json!(12);
    assert_eq!(set.include(&number).is_ok(), false, "Number is not valid for Set");

    let string = serde_json::json!("foo");
    assert_eq!(set.include(&string).is_ok(), false, "String is not valid for Set");

    let obj = serde_json::json!({});
    assert_eq!(set.include(&obj).is_ok(), false, "Object is not valid for Set");

    // Valid empty array
    let arr = serde_json::json!([]);
    assert_eq!(set.include(&arr).is_ok(), true, "Empty array is valid for Set");

    // Valid set of .String
    let arr = serde_json::json!(["foo", "bar"]);
    assert_eq!(set.include(&arr).is_ok(), true, "Valid set is valid for Set");

    // Inalid set of .String (duplicates)
    let arr = serde_json::json!(["foo", "foo"]);
    assert_eq!(set.include(&arr).is_ok(), false, "Array with duplicates is not valid for Set");

}
