use crate::common::FilePosition;
use crate::fio::{self};

use super::errors::ValidationError;
use super::typemap::TypeMap;
use super::heading::Heading;
#[derive(Clone, Debug)]
pub struct Relation {
    pub heading: Heading,
    pub position: FilePosition,
}

impl Relation {
    pub(crate) fn from_fio(ftuple: &fio::RelationType) -> Self {
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
