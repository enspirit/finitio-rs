use std::rc::Rc;
use std::{cell::RefCell};

use super::any::Any;
use super::nil::Nil;
use super::builtin::Builtin;
use super::r#ref::Ref;
use super::seq::Seq;
use super::set::Set;
use super::{errors::ValidationError, typemap::TypeMap};

#[derive(Clone, Debug)]
pub enum TypeDef {
  Any(Rc<RefCell<Any>>),
  Nil(Rc<RefCell<Nil>>),
  Builtin(Rc<RefCell<Builtin>>),
  Ref(Rc<RefCell<Ref>>),
  Seq(Rc<RefCell<Seq>>),
  Set(Rc<RefCell<Set>>),
}

impl TypeDef {
  pub fn name(&self) -> String {
    match self {
        TypeDef::Any(t) => t.borrow().name.clone(),
        TypeDef::Nil(t) => t.borrow().name.clone(),
        TypeDef::Builtin(t) => t.borrow().name.clone(),
        TypeDef::Ref(t) => t.borrow().name.clone(),
        TypeDef::Seq(t) => t.borrow().name.clone(),
        TypeDef::Set(t) => t.borrow().name.clone(),
    }
  }

  pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
    match self {
        TypeDef::Any(t) => t.borrow_mut().resolve(type_map),
        TypeDef::Nil(t) => t.borrow_mut().resolve(type_map),
        TypeDef::Builtin(t) => t.borrow_mut().resolve(type_map),
        TypeDef::Ref(t) => t.borrow_mut().resolve(type_map),
        TypeDef::Seq(t) => t.borrow_mut().resolve(type_map),
        TypeDef::Set(t) => t.borrow_mut().resolve(type_map),
    }
  }

}

