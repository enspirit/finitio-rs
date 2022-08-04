use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::typemap::TypeMap;

#[derive(Clone,Debug)]
pub struct Builtin {
    pub name: String,
    pub target: String,
    pub position: FilePosition,
}

impl Builtin {
    pub(crate) fn from_fio(
        name: String,
        fbuiltin: &fio::BuiltinType
    ) -> Self {
        Self {
            name: name,
            target: fbuiltin.name.clone(),
            position: fbuiltin.position.clone()
        }
    }

    pub(crate) fn resolve(&mut self, _type_map: &TypeMap) -> Result<(), ValidationError> {
        // TODO: user should be able to provide a list of builtins that are supported
        Ok(())
    }
}
