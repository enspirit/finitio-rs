use snafu::{Whatever, whatever, ResultExt};

use crate::schema::{TypeInclude, seq::Seq};

impl TypeInclude<serde_json::Value> for Seq {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        match v {
            serde_json::Value::Array(a) => {
                let first_invalid = a.iter().position(|v| {
                    match self.elm_type.include(v) {
                        Ok(_) => false,
                        Err(_) => true,
                    }
                });
                match first_invalid {
                    None => Ok(()),
                    Some(index) => {
                        let value = a.get(index).unwrap();
                        self.elm_type.include(value)
                            .with_whatever_context(|_| format!("Seq contains invalid value at index {}", index))
                    }
                }
            },
            v => whatever!("Not an array: {}", v),
        }
    }
}

#[cfg(test)]
use crate::schema::{any::Any, nil::Nil, r#type::Type};
#[test]
fn test_include_seq() {
    use crate::common::FilePosition;

    let position = FilePosition { line: 2, column: 2};
    let any = Type::Any(Any { position: position.clone() });

    // Seq of any
    let seq = Seq {
        position: position.clone(),
        elm_type: Box::new(any)
    };

    let nil = serde_json::Value::Null {};
    assert_eq!(seq.include(&nil).is_ok(), false);

    let number = serde_json::json!(12);
    assert_eq!(seq.include(&number).is_ok(), false);

    let string = serde_json::json!("foo");
    assert_eq!(seq.include(&string).is_ok(), false);

    let obj = serde_json::json!({});
    assert_eq!(seq.include(&obj).is_ok(), false);

    // Valid empty array
    let arr = serde_json::json!([]);
    assert_eq!(seq.include(&arr).is_ok(), true);

    // Valid array of Any
    let arr = serde_json::json!([1, "", {}]);
    assert_eq!(seq.include(&arr).is_ok(), true);

    // Seq of null
    let nil = Type::Nil(Nil { position: position.clone() });
    let seq = Seq {
        position: position.clone(),
        elm_type: Box::new(nil)
    };

    // Valid array of Nil
    let arr = serde_json::json!([serde_json::Value::Null{}]);
    assert_eq!(seq.include(&arr).is_ok(), true);

    // Invalid array of Nil
    let arr = serde_json::json!([serde_json::Value::Null{}, 2]);
    assert_eq!(seq.include(&arr).is_ok(), false);
}
