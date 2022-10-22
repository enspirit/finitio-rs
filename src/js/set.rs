use snafu::{Whatever, whatever, ResultExt};
use crate::schema::{TypeInclude, set::Set};

impl TypeInclude<serde_json::Value> for Set {
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
                            .with_whatever_context(|_| format!("Set contains invalid value at index {}", index))
                    }
                }
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
    assert_eq!(set.include(&nil).is_ok(), false);

    let number = serde_json::json!(12);
    assert_eq!(set.include(&number).is_ok(), false);

    let string = serde_json::json!("foo");
    assert_eq!(set.include(&string).is_ok(), false);

    let obj = serde_json::json!({});
    assert_eq!(set.include(&obj).is_ok(), false);

    // Valid empty array
    let arr = serde_json::json!([]);
    assert_eq!(set.include(&arr).is_ok(), true);

    // Valid set of .String
    let arr = serde_json::json!(["foo", "bar"]);
    assert_eq!(set.include(&arr).is_ok(), true);

    // Inalid set of .String (duplicates)
    let arr = serde_json::json!(["foo", "foo"]);
    assert_eq!(set.include(&arr).is_ok(), false);

}
