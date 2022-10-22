use snafu::Whatever;

use crate::schema::{TypeInclude, r#struct::Struct};

impl TypeInclude<serde_json::Value> for Struct {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        todo!()
    }
}
