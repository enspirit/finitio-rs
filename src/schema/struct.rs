use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Struct {
    pub elements: Vec<Type>,
    pub position: FilePosition,
}

impl Struct {
    pub(crate) fn from_fio(fstruct: &fio::StructType) -> Self {
        let elements = fstruct.elements
            .iter()
            .map(|ftype| Type::from_fio(ftype))
            .collect();

        Self {
            elements: elements,
            position: fstruct.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        for mut c in self.elements.iter_mut() {
            c.resolve(type_map)?;
        }
        Ok(())
    }
}
