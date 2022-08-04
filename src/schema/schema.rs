use crate::{fio, common::FilePosition};
use super::{errors::ValidationError, TypeDef, typemap::TypeMap, Type, builtin::Builtin, r#ref::Ref};
use std::{collections::{btree_map::Entry as BTreeMapEntry, BTreeMap}, rc::Rc, cell::RefCell};
use super::any::Any;
use super::nil::Nil;
use super::seq::Seq;
use super::set::Set;

#[derive(Default, Debug)]
pub struct Schema {
  pub types: BTreeMap<String, TypeDef>
}

impl Schema {
  pub fn from_fio(
    fschema: fio::Schema
  ) -> Result<Self, ValidationError> {
    let mut ns = Self::default();
    let mut type_map = TypeMap::new();

    let mut names: BTreeMap<String, FilePosition> = BTreeMap::new();
    for typedef in fschema.type_defs.iter() {
      // Check for name clash, keep track of typedef position for nice error messages
      match names.entry(typedef.name.to_owned()) {
        BTreeMapEntry::Occupied(entry) => {
          return Err(ValidationError::DuplicateIdentifier {
            position: entry.get().clone(),
            identifier: typedef.name.to_owned()
          })
        },
        BTreeMapEntry::Vacant(entry) => {
          entry.insert(typedef.position.clone());
        }
      }
      //

      match &typedef.target {
        fio::Type::NilType(t) => {
          ns.add_type(
            TypeDef::Nil(Rc::new(RefCell::new(Nil::from_fio(
              typedef.name.clone(),
              t
            )))),
            &mut type_map
          )
        },
        fio::Type::AnyType(t) => {
          ns.add_type(
            TypeDef::Any(Rc::new(RefCell::new(Any::from_fio(
              typedef.name.clone(),
              t
            )))),
            &mut type_map
          )
        },
        fio::Type::BuiltinType(t) => {
          ns.add_type(
            TypeDef::Builtin(Rc::new(RefCell::new(Builtin::from_fio(
              typedef.name.clone(),
              t
            )))),
            &mut type_map
          )
        },
        fio::Type::RefType(t) => {
          ns.add_type(
            TypeDef::Ref(Rc::new(RefCell::new(Ref::from_fio(
              typedef.name.clone(),
              t
            )))),
            &mut type_map
          )
        },
        fio::Type::SeqType(t) => {
          ns.add_type(
            TypeDef::Seq(Rc::new(RefCell::new(Seq::from_fio(
              typedef.name.clone(),
              t
            )))),
            &mut type_map
          )
        },
        fio::Type::SetType(t) => {
          ns.add_type(
            TypeDef::Set(Rc::new(RefCell::new(Set::from_fio(
              typedef.name.clone(),
              t
            )))),
            &mut type_map
          )
        },
      }
    }
    ns.resolve(&type_map)?;
    Ok(ns)
  }

  fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
    for ud_type in self.types.values_mut() {
        ud_type.resolve(type_map)?;
    }
    Ok(())
  }

  fn add_type(&mut self, type_: TypeDef, type_map: &mut TypeMap) {
    type_map.insert(&type_);
    self.types.insert(type_.name().to_owned(), type_);
}
}
