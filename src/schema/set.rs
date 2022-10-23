use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Set<'a> {
    pub elm_type: Box<Type<'a>>,
    pub position: FilePosition,
}

impl<'a> Set<'a> {
    pub(crate) fn from_fio(fset: &'a fio::SetType) -> Self {
        let elm_type = Type::from_fio(&fset.elm_type);
        Self {
            elm_type: Box::new(elm_type),
            position: fset.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &'a TypeMap<'a>) -> Result<(), ValidationError> {
        self.elm_type.resolve(type_map)?;
        Ok(())
    }
}
