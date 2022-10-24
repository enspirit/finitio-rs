use snafu::{Whatever, whatever};

use crate::schema::{FinitioType, nil::Nil};

impl FinitioType<serde_json::Value> for Nil {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        match v {
            serde_json::Value::Null => Ok(()),
            v => whatever!("Invalid Nil: `{}`", v)
        }
    }
}

#[cfg(test)]
#[test]
fn test_include_nil() {
    use crate::common::FilePosition;

    let any = Nil { position: FilePosition { line: 2, column: 2} };

    let nil = serde_json::Value::Null {};
    assert_eq!(any.include(&nil).is_ok(), true);

    let number = serde_json::json!(12);
    assert_eq!(any.include(&number).is_ok(), false);

    let string = serde_json::json!("foo");
    assert_eq!(any.include(&string).is_ok(), false);

    let obj = serde_json::json!({});
    assert_eq!(any.include(&obj).is_ok(), false);
}
