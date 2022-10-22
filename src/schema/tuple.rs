use std::collections::HashMap;

use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::heading::Heading;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Tuple {
    pub heading: Heading,
    pub position: FilePosition,
}

impl Tuple {
    pub(crate) fn from_fio(ftuple: &fio::TupleType) -> Self {
        let heading = Heading::from_fio(&ftuple.heading);
        Self {
            heading,
            position: ftuple.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        self.heading.resolve(type_map)
    }
}
