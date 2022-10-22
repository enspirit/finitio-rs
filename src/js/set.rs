use snafu::Whatever;

use crate::schema::{TypeInclude, set::Set};

impl TypeInclude<serde_json::Value> for Set {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        todo!()
    }
}
