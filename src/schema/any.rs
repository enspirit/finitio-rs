use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Any {
    pub position: FilePosition,
}

impl Any {
    pub(crate) fn from_fio(fany: &fio::AnyType) -> Self {
        Self {
            position: fany.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, _type_map: &TypeMap) -> Result<(), ValidationError> {
        Ok(())
    }
}
