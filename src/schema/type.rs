use std::rc::Rc;
use std::{rc::Weak, cell::RefCell};

use crate::common::FilePosition;
use crate::fio;

use super::base::Base;
use super::seq::Seq;
use super::set::Set;
use super::{errors::ValidationError, typemap::TypeMap};

#[derive(Clone, Debug)]
pub enum Type {
  Nil,
  Any,
  Builtin(String),
  Ref(TypeRef)
}

#[derive(Clone, Debug)]
pub enum UserDefinedType {
  Base(Rc<RefCell<Base>>),
  Seq(Rc<RefCell<Seq>>),
  Set(Rc<RefCell<Set>>),
}

#[derive(Clone,Debug)]
pub enum TypeRef {
  Base(BaseRef),
  Seq(SeqRef),
  Set(SetRef),
  Unresolved { name: String, position: FilePosition },
}

#[derive(Clone,Debug)]
pub struct BaseRef {
  pub base_: Weak<RefCell<Base>>
}

#[derive(Clone,Debug)]
pub struct SeqRef {
  pub seq_: Weak<RefCell<Seq>>
}

#[derive(Clone,Debug)]
pub struct SetRef {
  pub set_: Weak<RefCell<Set>>
}

impl TypeRef {
  pub fn name(&self) -> String {
    match self {
      Self::Unresolved { name, position } => name.clone(),
      Self::Base(base) => base.base_.upgrade().unwrap().borrow().name.clone(),
      Self::Seq(seq) => seq.seq_.upgrade().unwrap().borrow().name.clone(),
      Self::Set(set) => set.set_.upgrade().unwrap().borrow().name.clone(),
    }
  }
  pub fn position(&self) -> FilePosition {
    match self {
      Self::Unresolved { name, position } => position.clone(),
      Self::Base(base) => base.base_.upgrade().unwrap().borrow().position.clone(),
      Self::Seq(seq) => seq.seq_.upgrade().unwrap().borrow().position.clone(),
      Self::Set(set) => set.set_.upgrade().unwrap().borrow().position.clone(),
    }
  }
  pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
    if let Self::Unresolved { name, position } = self {
      let ftype = type_map.get(name);
      *self = match ftype {
        Some(udtype) => {
          match udtype {
            UserDefinedType::Base(base_) => TypeRef::Base(BaseRef {
              base_: Rc::downgrade(&base_)
            }),
            UserDefinedType::Seq(seq_) => TypeRef::Seq(SeqRef {
              seq_: Rc::downgrade(&seq_)
            }),
            UserDefinedType::Set(set_) => TypeRef::Set(SetRef {
              set_: Rc::downgrade(&set_)
            }),
          }
        },
        None => {
          return Err(ValidationError::NoSuchType {
              name: name.clone(),
              position: self.position(),
          })
      }
      }
    }
    Ok(())
  }
}

impl Type {
  pub fn from_fio(
    ftype: &fio::BaseType
  ) -> Self {
    match ftype {
        fio::BaseType::Nil => Self::Nil,
        fio::BaseType::Any => Self::Any,
        fio::BaseType::Builtin(n) => Self::Builtin(n.name.clone()),
        fio::BaseType::Ref(n) => Self::Ref(TypeRef::Unresolved { name: n.name.clone(), position: n.position.clone() }),
    }
  }

  pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
    match self {
        Self::Nil
        | Self::Any
        | Self::Builtin(_) => Ok(()), // Should probably not consider all Builtin ok here?

        Self::Ref(tref) => tref.resolve(type_map),
    }
  }
}

impl UserDefinedType {
  pub fn name(&self) -> String {
    match self {
        UserDefinedType::Base(t) => t.borrow().name.clone(),
        UserDefinedType::Seq(t) => t.borrow().name.clone(),
        UserDefinedType::Set(t) => t.borrow().name.clone(),
    }
  }

  pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
    match self {
        UserDefinedType::Base(t) => t.borrow_mut().resolve(type_map),
        UserDefinedType::Seq(t) => t.borrow_mut().resolve(type_map),
        UserDefinedType::Set(t) => t.borrow_mut().resolve(type_map),
    }
  }

}

