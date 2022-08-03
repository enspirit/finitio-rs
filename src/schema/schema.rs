use crate::{fio, common::FilePosition};
use super::{errors::ValidationError, UserDefinedType, typemap::TypeMap, Type, base::Base};
use std::{collections::{btree_map::Entry as BTreeMapEntry, BTreeMap}, rc::Rc, cell::RefCell};
use super::seq::Seq;
use super::set::Set;

#[derive(Default, Debug)]
pub struct Schema {
  pub types: BTreeMap<String, UserDefinedType>
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
        fio::Type::BaseType(t) => {
          ns.add_type(
            UserDefinedType::Base(Rc::new(RefCell::new(Base::from_fio(
              typedef.name.clone(),
              t
            )))),
            &mut type_map
          )
        },
        fio::Type::SeqType(t) => {
          ns.add_type(
            UserDefinedType::Seq(Rc::new(RefCell::new(Seq::from_fio(
              typedef.name.clone(),
              t
            )))),
            &mut type_map
          )
        },
        fio::Type::SetType(t) => {
          ns.add_type(
            UserDefinedType::Set(Rc::new(RefCell::new(Set::from_fio(
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

  fn add_type(&mut self, type_: UserDefinedType, type_map: &mut TypeMap) {
    type_map.insert(&type_);
    self.types.insert(type_.name().to_owned(), type_);
}
}
