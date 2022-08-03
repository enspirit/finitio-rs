use std::rc::Rc;
use std::{rc::Weak, cell::RefCell};

use crate::common::FilePosition;
use crate::fio;

use super::base::Base;
use super::seq::Seq;
use super::set::Set;
use super::{errors::ValidationError, typemap::TypeMap};

#[derive(Clone, Debug)]
pub enum TypeDef {
  Base(Rc<RefCell<Base>>),
  Seq(Rc<RefCell<Seq>>),
  Set(Rc<RefCell<Set>>),
}

impl TypeDef {
  pub fn name(&self) -> String {
    match self {
        TypeDef::Base(t) => t.borrow().name.clone(),
        TypeDef::Seq(t) => t.borrow().name.clone(),
        TypeDef::Set(t) => t.borrow().name.clone(),
    }
  }

  pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
    match self {
        TypeDef::Base(t) => t.borrow_mut().resolve(type_map),
        TypeDef::Seq(t) => t.borrow_mut().resolve(type_map),
        TypeDef::Set(t) => t.borrow_mut().resolve(type_map),
    }
  }

}

