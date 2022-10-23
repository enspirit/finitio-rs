use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::heading::Heading;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Tuple<'a> {
    pub heading: Heading<'a>,
    pub position: FilePosition,
}

impl<'a> Tuple<'a> {
    pub(crate) fn from_fio(ftuple: &fio::TupleType<'a>) -> Self {
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
