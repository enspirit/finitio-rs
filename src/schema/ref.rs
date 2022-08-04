use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Ref {
    pub target: Type,
    pub position: FilePosition,
}

impl Ref {
    pub(crate) fn from_fio(fref: &fio::RefType) -> Self {
        let target = Type::from_fio_ref(fref);
        Self {
            target,
            position: fref.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        self.target.resolve(type_map)?;
        Ok(())
    }
}
