use crate::schema::{TypeInclude, relation::Relation};

impl TypeInclude<serde_json::Value> for Relation {
    fn include(&self, v: &serde_json::Value) -> Result<bool, &'static str> {
        todo!()
    }
}
