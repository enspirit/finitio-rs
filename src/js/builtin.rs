use crate::schema::{TypeInclude, builtin::Builtin};

impl TypeInclude<serde_json::Value> for Builtin {
    fn include(&self, v: &serde_json::Value) -> Result<bool, std::io::Error> {
        todo!()
    }
}
