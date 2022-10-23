use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Union<'a> {
    pub candidates: Vec<Type<'a>>,
    pub position: FilePosition,
}

impl<'a> Union<'a> {
    pub(crate) fn from_fio(funion: &'a fio::UnionType) -> Self {
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

    pub(crate) fn resolve(&mut self, type_map: &'a TypeMap<'a>) -> Result<(), ValidationError> {
        for c in self.candidates.iter_mut() {
            c.resolve(type_map)?;
        }
        Ok(())
    }
}
