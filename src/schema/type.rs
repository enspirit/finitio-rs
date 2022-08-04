use std::rc::Rc;
use std::{rc::Weak, cell::RefCell};

use crate::common::FilePosition;
use crate::fio;

use super::any::Any;
use super::builtin::Builtin;
use super::nil::Nil;
use super::seq::Seq;
use super::set::Set;
use super::typedef::TypeDef;
use super::{errors::ValidationError, typemap::TypeMap};

#[derive(Clone, Debug)]
pub enum Type {
  Nil(FilePosition),
  Any(FilePosition),
  Builtin(String),
  Ref(TypeRef)
}

#[derive(Clone,Debug)]
pub enum TypeRef {
  Any(AnyRef),
  Nil(NilRef),
  Builtin(BuiltinRef),
  Seq(SeqRef),
  Set(SetRef),
  Unresolved { name: String, position: FilePosition },
}

#[derive(Clone,Debug)]
pub struct AnyRef {
  pub any_: Weak<RefCell<Any>>
}

#[derive(Clone,Debug)]
pub struct NilRef {
  pub nil_: Weak<RefCell<Nil>>
}

#[derive(Clone,Debug)]
pub struct BuiltinRef {
  pub builtin_: Weak<RefCell<Builtin>>
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
      Self::Any(t) => t.any_.upgrade().unwrap().borrow().name.clone(),
      Self::Nil(t) => t.nil_.upgrade().unwrap().borrow().name.clone(),
      Self::Builtin(t) => t.builtin_.upgrade().unwrap().borrow().name.clone(),
      Self::Seq(seq) => seq.seq_.upgrade().unwrap().borrow().name.clone(),
      Self::Set(set) => set.set_.upgrade().unwrap().borrow().name.clone(),
      Self::Unresolved { name, position } => name.clone(),
    }
  }
  pub fn position(&self) -> FilePosition {
    match self {
      Self::Any(any) => any.any_.upgrade().unwrap().borrow().position.clone(),
      Self::Nil(nil) => nil.nil_.upgrade().unwrap().borrow().position.clone(),
      Self::Builtin(nil) => nil.builtin_.upgrade().unwrap().borrow().position.clone(),
      Self::Seq(seq) => seq.seq_.upgrade().unwrap().borrow().position.clone(),
      Self::Set(set) => set.set_.upgrade().unwrap().borrow().position.clone(),
      Self::Unresolved { name, position } => position.clone(),
    }
  }
  pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
    if let Self::Unresolved { name, position } = self {
      let ftype = type_map.get(name);
      *self = match ftype {
        Some(udtype) => {
          match udtype {
            TypeDef::Any(any_) => TypeRef::Any(AnyRef {
              any_: Rc::downgrade(&any_)
            }),
            TypeDef::Nil(nil_) => TypeRef::Nil(NilRef {
              nil_: Rc::downgrade(&nil_)
            }),
            TypeDef::Builtin(builtin_) => TypeRef::Builtin(BuiltinRef {
              builtin_: Rc::downgrade(&builtin_)
            }),
            TypeDef::Seq(seq_) => TypeRef::Seq(SeqRef {
              seq_: Rc::downgrade(&seq_)
            }),
            TypeDef::Set(set_) => TypeRef::Set(SetRef {
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
    ftype: &fio::Type
  ) -> Self {
    match ftype {
        fio::Type::NilType(t) => Self::Nil(t.position.clone()),
        fio::Type::AnyType(t) => Self::Any(t.position.clone()),
        fio::Type::BuiltinType(t) => Self::Builtin(t.name.clone()),
        fio::Type::RefType(_) => todo!(),
        fio::Type::SeqType(_) => todo!(),
        fio::Type::SetType(_) => todo!(),
        // fio::BaseType::Nil => Self::Nil,
        // fio::BaseType::Any => Self::Any,
        // fio::BaseType::Builtin(n) => Self::Builtin(n.name.clone()),
        // fio::BaseType::Ref(n) => Self::Ref(TypeRef::Unresolved { name: n.name.clone(), position: n.position.clone() }),
    }
  }

  pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
    match self {
        Self::Nil(_)
        | Self::Any(_)
        | Self::Builtin(_) => Ok(()), // Should probably not consider all Builtin ok here?

        Self::Ref(tref) => tref.resolve(type_map),
    }
  }
}
