use snafu::Whatever;

use crate::schema::{TypeInclude, sub::Sub};

impl TypeInclude<serde_json::Value> for Sub {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        todo!()
    }
}
