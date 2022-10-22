use snafu::{Whatever, whatever};
use snafu::prelude::*;
use crate::schema::{TypeInclude, builtin::Builtin};

impl TypeInclude<serde_json::Value> for Builtin {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        match self.target.as_str() {
            "Number" => {
                match v {
                    serde_json::Value::Number(_) => Ok(()),
                    v => whatever!("Not a valid builtin Number: {}", v)
                }
            },
            "String" => {
                match v {
                    serde_json::Value::String(_) => Ok(()),
                    v => whatever!("Not a valid builtin String: {}", v)
                }
            },
            "Boolean" => {
                match v {
                    serde_json::Value::Bool(_) => Ok(()),
                    v => whatever!("Not a valid builtin Boolean: {}", v)
                }
            },
            &_ => whatever!("Unsupported builtin type name: {}", self.target)
        }
    }
}


#[cfg(test)]
use crate::schema::{any::Any, nil::Nil, r#type::Type};
#[test]
fn test_include_builtin() {
    use crate::common::FilePosition;

    let position = FilePosition { line: 2, column: 2};

    let builtin_str = Builtin {
        position: position.clone(),
        target: String::from("String")
    };

    let nil = serde_json::Value::Null {};
    assert_eq!(builtin_str.include(&nil).is_ok(), false);

    let number = serde_json::json!(12);
    assert_eq!(builtin_str.include(&number).is_ok(), false);

    let string = serde_json::json!("foo");
    assert_eq!(builtin_str.include(&string).is_ok(), true);

    let obj = serde_json::json!({});
    assert_eq!(builtin_str.include(&obj).is_ok(), false);

    let arr = serde_json::json!([]);
    assert_eq!(builtin_str.include(&arr).is_ok(), false);
}
