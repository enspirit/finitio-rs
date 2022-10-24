use snafu::{Whatever, whatever, ResultExt};

use crate::schema::{FinitioType, r#struct::Struct};

impl FinitioType<serde_json::Value> for Struct {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        match v {
            serde_json::Value::Array(arr) => {
                if arr.len() < self.elements.len() {
                    whatever!("Not enough values in struct, expected length {} found: {}", self.elements.len(), arr.len())
                }
                if arr.len() > self.elements.len() {
                    whatever!("Too many values in struct, expected length {} found: {}", self.elements.len(), arr.len())
                }

                for (pos, val) in arr.iter().enumerate() {
                    let val_type = self.elements.get(pos).unwrap();
                    match val_type.include(val) {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(e).with_whatever_context(|_| format!("Invalid struct value: {}", v))
                        }
                    }
                }

                Ok(())
            },
            _ => whatever!("Invalid value for type Struct: {}", v)
        }
    }
}

#[cfg(test)]
use crate::schema::{any::Any, builtin::Builtin, r#type::Type};

#[test]
fn test_include_struct() {
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
    let str = Type::Struct(Struct {
        position: position.clone(),
        elements: vec![builtin_str, builtin_num, any_t]
    });

    // invalid: missing values
    let missing_values = serde_json::json!([]);
    assert_eq!(str.include(&missing_values).is_ok(), false, "array with missing values is not valid");
    let missing_values = serde_json::json!(["foo"]);
    assert_eq!(str.include(&missing_values).is_ok(), false, "array with missing values is not valid");

    // invalid: extra values
    let extra_values = serde_json::json!(["foo", 42, {}, 22]);
    assert_eq!(str.include(&extra_values).is_ok(), false, "array with extra values is not valid");

    // invalid: wrong types
    let wrong_types = serde_json::json!([{}, 42, "foo"]);
    assert_eq!(str.include(&wrong_types).is_ok(), false, "array with invalid types is not valid");

    // Valid
    let valid = serde_json::json!(["foo", 42, {}]);
    assert_eq!(str.include(&valid).is_ok(), true, "array with valid values is valid");

}
