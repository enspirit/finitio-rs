use snafu::Whatever;

use crate::schema::{FinitioType, any::Any};

impl<T> FinitioType<T> for Any {
    fn include(&self, _: &T) -> Result<(), Whatever> {
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_include_any() {
    use crate::common::FilePosition;

    let any = Any { position: FilePosition { line: 2, column: 2} };

    let nil = serde_json::Value::Null {};
    assert_eq!(any.include(&nil).is_ok(), true);

    let number = serde_json::json!(12);
    assert_eq!(any.include(&number).is_ok(), true);

    let string = serde_json::json!("foo");
    assert_eq!(any.include(&string).is_ok(), true);

    let obj = serde_json::json!({});
    assert_eq!(any.include(&obj).is_ok(), true);
}
