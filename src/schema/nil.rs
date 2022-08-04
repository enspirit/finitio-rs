
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
pub struct Nil {
    pub name: String,
    pub position: FilePosition,
}

impl Nil {
    pub(crate) fn from_fio(
        name: String,
        fnil: &fio::NilType
    ) -> Self {
        Self { name: name, position: fnil.position.clone() }
    }

    pub(crate) fn resolve(&mut self, _type_map: &TypeMap) -> Result<(), ValidationError> {
        Ok(())
    }
}