use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Union {
    pub candidates: Vec<Type>,
    pub position: FilePosition,
}

impl Union {
    pub(crate) fn from_fio(funion: &fio::UnionType) -> Self {
        let candidates = funion
            .candidates
            .iter()
            .map(|ftype| Type::from_fio(ftype))
            .collect();

        Self {
            candidates: candidates,
            position: funion.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        for mut c in self.candidates.iter_mut() {
            c.resolve(type_map)?;
        }
        Ok(())
    }
}
