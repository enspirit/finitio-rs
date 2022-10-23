use crate::common::FilePosition;
use crate::fio::{self};

use super::errors::ValidationError;
use super::typemap::TypeMap;
use super::heading::Heading;
#[derive(Clone, Debug)]
pub struct Relation<'a> {
    pub heading: Heading<'a>,
    pub position: FilePosition,
}

impl<'a> Relation<'a> {
    pub(crate) fn from_fio(ftuple: &fio::RelationType<'a>) -> Self {
        let heading = Heading::from_fio(&ftuple.heading);
        Self {
            heading,
            position: ftuple.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap<'a>) -> Result<(), ValidationError> {
        self.heading.resolve(type_map)
    }
}
