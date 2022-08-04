use std::rc::Rc;
use std::{rc::Weak, cell::RefCell};

use crate::common::FilePosition;
use crate::fio;

use super::any::Any;
use super::builtin::Builtin;
use super::nil::Nil;
use super::r#ref::Ref;
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
  Ref(RefRef),
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
pub struct RefRef {
  pub ref_: Weak<RefCell<Ref>>
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
  pub fn position(&self) -> FilePosition {
    match self {
      Self::Any(r) => r.any_.upgrade().unwrap().borrow().position.clone(),
      Self::Nil(r) => r.nil_.upgrade().unwrap().borrow().position.clone(),
      Self::Builtin(r) => r.builtin_.upgrade().unwrap().borrow().position.clone(),
      Self::Ref(r) => r.ref_.upgrade().unwrap().borrow().position.clone(),
      Self::Seq(r) => r.seq_.upgrade().unwrap().borrow().position.clone(),
      Self::Set(r) => r.set_.upgrade().unwrap().borrow().position.clone(),
      Self::Unresolved { name, position } => position.clone(),
    }
  }
  pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
    if let Self::Unresolved { name, position } = self {
      let ftype = type_map.get(name);
      *self = match ftype {
        Some(udtype) => {
          match udtype {
            TypeDef::AnyType(any_) => TypeRef::Any(AnyRef {
              any_: Rc::downgrade(&any_.target)
            }),
            TypeDef::NilType(nil_) => TypeRef::Nil(NilRef {
              nil_: Rc::downgrade(&nil_.target)
            }),
            TypeDef::BuiltinType(builtin_) => TypeRef::Builtin(BuiltinRef {
              builtin_: Rc::downgrade(&builtin_.target)
            }),
            TypeDef::RefType(ref_) => TypeRef::Ref(RefRef {
              ref_: Rc::downgrade(&ref_.target)
            }),
            TypeDef::SeqType(seq_) => TypeRef::Seq(SeqRef {
              seq_: Rc::downgrade(&seq_.target)
            }),
            TypeDef::SetType(set_) => TypeRef::Set(SetRef {
              set_: Rc::downgrade(&set_.target)
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
        fio::Type::RefType(t) => Self::Ref(TypeRef::Unresolved { name: t.name.clone(), position: t.position.clone() }),
        fio::Type::SeqType(_) => todo!(),
        fio::Type::SetType(_) => todo!(),
    }
  }

  pub fn from_fio_ref(
    fref: &fio::RefType
  ) -> Self {
    Self::Ref(TypeRef::Unresolved {
      name: fref.name.clone(),
      position: fref.position.clone()
    })
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
