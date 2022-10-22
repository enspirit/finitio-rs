use crate::schema::{TypeInclude, tuple::Tuple};

impl TypeInclude<serde_json::Value> for Tuple {
    fn include(&self, v: &serde_json::Value) -> Result<bool, std::io::Error> {
        todo!()
    }
}
