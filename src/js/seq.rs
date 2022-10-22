use crate::schema::{TypeInclude, seq::Seq};

impl TypeInclude<serde_json::Value> for Seq {
    fn include(&self, v: &serde_json::Value) -> Result<bool, std::io::Error> {
        todo!()
    }
}
