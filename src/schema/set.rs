
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
pub struct Set {
    pub elm_type: Box<Type>,
    pub position: FilePosition,
}

impl Set {
    pub(crate) fn from_fio(
        fset: &fio::SetType
    ) -> Self {
        let elm_type = Type::from_fio(&fset.elm_type);
        Self {
            elm_type: Box::new(elm_type),
            position: fset.position.clone()
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        self.elm_type.resolve(type_map)?;
        Ok(())
    }
}
