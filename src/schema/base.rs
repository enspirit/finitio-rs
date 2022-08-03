
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;
use super::TypeRef;

#[derive(Clone, Debug)]
pub struct Base {
    pub name: String,
    pub elm_type: Type,
    pub position: FilePosition,
}

impl Base {
    pub(crate) fn from_fio(
        name: String,
        fbase: &fio::BaseType
    ) -> Self {
        let elm_type = Type::from_fio(&fbase);
        let position = FilePosition { line: 1, column: 1 }; // FIX ME
        Self { name: name, elm_type: elm_type, position }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        self.elm_type.resolve(type_map)?;
        Ok(())
    }
}
