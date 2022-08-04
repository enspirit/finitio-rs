use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Nil {
    pub position: FilePosition,
}

impl Nil {
    pub(crate) fn from_fio(fnil: &fio::NilType) -> Self {
        Self {
            position: fnil.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, _type_map: &TypeMap) -> Result<(), ValidationError> {
        Ok(())
    }
}
