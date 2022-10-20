use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::{fio};

pub fn generate_json(adt: &fio::Schema) -> Result<()> {
    let j = serde_json::to_string(&adt)?;
    println!("{}", j);
    Ok(())
}

