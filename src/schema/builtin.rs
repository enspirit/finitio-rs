use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Builtin<'a> {
    pub target: &'a str,
    pub position: FilePosition,
}

impl<'a> Builtin<'a> {
    pub(crate) fn from_fio(fbuiltin: &'a fio::BuiltinType) -> Self {
        Self {
            target: &fbuiltin.name[..],
            position: fbuiltin.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, _type_map: &TypeMap) -> Result<(), ValidationError> {
        // TODO: user should be able to provide a list of builtins that are supported
        Ok(())
    }
}
