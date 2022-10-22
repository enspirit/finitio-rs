use crate::schema::{TypeInclude, nil::Nil};

impl TypeInclude<serde_json::Value> for Nil {
    fn include(&self, v: &serde_json::Value) -> Result<bool, std::io::Error> {
        match v {
            serde_json::Value::Null => Ok(true),
            _ => Ok(false)
        }
    }
}
