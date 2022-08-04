use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Seq {
    pub elm_type: Box<Type>,
    pub position: FilePosition,
}

impl Seq {
    pub(crate) fn from_fio(fseq: &fio::SeqType) -> Self {
        let elm_type = Type::from_fio(&fseq.elm_type);
        Self {
            elm_type: Box::new(elm_type),
            position: fseq.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        self.elm_type.resolve(type_map)?;
        Ok(())
    }
}
