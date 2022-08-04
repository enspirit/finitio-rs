
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;
use super::TypeRef;

#[derive(Clone,Debug)]
pub struct Ref {
    pub name: String,
    pub target: Type,
    pub position: FilePosition,
}

impl Ref {
    pub(crate) fn from_fio(
        name: String,
        fref: &fio::RefType
    ) -> Self {
        let target = Type::from_fio_ref(fref);
        Self { name: name, target, position: fref.position.clone() }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        self.target.resolve(type_map)?;
        Ok(())
    }
}
