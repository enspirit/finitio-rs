use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Struct<'a> {
    pub elements: Vec<Type<'a>>,
    pub position: FilePosition,
}

impl<'a> Struct<'a> {
    pub(crate) fn from_fio(fstruct: &'a fio::StructType) -> Self {
        let elements = fstruct
            .elements
            .iter()
            .map(|ftype| Type::from_fio(ftype))
            .collect();

        Self {
            elements: elements,
            position: fstruct.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &'a TypeMap<'a>) -> Result<(), ValidationError> {
        for c in self.elements.iter_mut() {
            c.resolve(type_map)?;
        }
        Ok(())
    }
}
