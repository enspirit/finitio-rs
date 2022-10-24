use std::{path::PathBuf, collections::HashMap};
use serde_json::Result;

use crate::{fio};

pub fn generate_json(adt: &HashMap<PathBuf, fio::Schema>) -> Result<()> {
    let j = serde_json::to_string(&adt)?;
    println!("{}", j);
    Ok(())
}

