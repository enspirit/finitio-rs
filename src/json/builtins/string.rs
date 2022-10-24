use serde_json::json;
use snafu::{Whatever, whatever};

pub struct JsString;
pub struct JsNumber;
pub struct JsBoolean;

pub enum JsTypes {
  JsString(JsString),
  JsNumber(JsNumber),
  JsBoolean(JsBoolean),
}

pub trait JsDress {
  fn dress(value: &serde_json::Value) -> Result<serde_json::Value, Whatever>;
}

impl JsDress for JsString {
    fn dress(value: &serde_json::Value) -> Result<serde_json::Value, Whatever> {
      match value {
        resolver::Value::String(s) => Ok(value.clone()),
        _ => whatever!("oops")
    }
  }
}

impl JsDress for JsNumber {
    fn dress(value: &serde_json::Value) -> Result<serde_json::Value, Whatever> {
      match value {
        resolver::Value::Number(n) => Ok(value.clone()),
        _ => whatever!("oops")
    }
  }
}

impl JsDress for JsBoolean {
    fn dress(value: &serde_json::Value) -> Result<serde_json::Value, Whatever> {
      match value {
        resolver::Value::Bool(b) => Ok(value.clone()),
        _ => whatever!("oops")
    }
  }
}

impl JsDress for JsTypes {
    fn dress(value: &serde_json::Value) -> Result<serde_json::Value, Whatever> {
        match value {
            resolver::Value::Null => todo!(),
            resolver::Value::Bool(b) => JsBoolean::dress(value),
            resolver::Value::Number(_) => JsNumber::dress(value),
            resolver::Value::String(_) => JsString::dress(value),
            resolver::Value::Array(_) => todo!(),
            resolver::Value::Object(_) => todo!(),
        }
    }
}

#[cfg(test)]
#[test]
fn test_dress() {
  // Strings
  let value = json!("foo");
  let got = JsTypes::dress(&value).unwrap();
  assert_eq!("foo", got.as_str().unwrap());

  // Numbers
  let value = json!(42);
  let got = JsTypes::dress(&value).unwrap();
  assert_eq!(42, got.as_i64().unwrap());

  // Booleans
  let value = json!(true);
  let got = JsTypes::dress(&value).unwrap();
  assert_eq!(true, got.as_bool().unwrap());
  let value = json!(false);
  let got = JsTypes::dress(&value).unwrap();
  assert_eq!(false, got.as_bool().unwrap());
}
