use snafu::OptionExt;

use super::any::Any;
use super::nil::Nil;
use super::r#struct::Struct;
use super::relation::Relation;
use super::seq::Seq;
use super::set::Set;
use super::sub::Sub;
use super::tuple::Tuple;
use super::union::Union;
use super::{
    builtin::Builtin, errors::ValidationError, r#ref::Ref, typedef::TypeDefStr, typemap::TypeMap,
    TypeDef,
};
use crate::{common::FilePosition, fio};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{
    cell::RefCell,
    collections::{btree_map::Entry as BTreeMapEntry, BTreeMap},
    rc::Rc,
};

#[derive(Default, Debug)]
pub struct Schema {
    pub types: BTreeMap<String, TypeDef>,
}

impl Schema {

    pub fn from_fios<'a>(fschemas: HashMap<PathBuf, fio::Schema>) -> Result<HashMap<PathBuf, Self>, ValidationError> {
        let mut schemas: HashMap<PathBuf, Schema> = HashMap::new();

        for (path, fschema) in fschemas.iter() {
            let mut typedefs = fschema.type_defs.to_vec();
            let mut names: BTreeMap<String, FilePosition> = BTreeMap::new();
            let mut ns = Self::default();
            let mut type_map = TypeMap::new();

            for import in fschema.imports.iter() {
                let base_dir = path.parent()
                    .expect("base_dir could not be determined from source");

                let import_path = base_dir.join(&import.filename);
                let schema = fschemas.get(&import_path)
                    .expect("Import refers to a schema that hasn't been loaded/parsed");

                typedefs.append(&mut schema.type_defs.to_vec());
            }

            for typedef in typedefs.iter() {
                // Check for name clash, keep track of typedef position for nice error messages
                match names.entry(typedef.name.to_owned()) {
                    BTreeMapEntry::Occupied(entry) => {
                        return Err(ValidationError::DuplicateIdentifier {
                            first_seen: entry.get().clone(),
                            position: typedef.position.clone(),
                            identifier: typedef.name.to_owned(),
                        })
                    }
                    BTreeMapEntry::Vacant(entry) => {
                        entry.insert(typedef.position.clone());
                    }
                }
                //

                match &typedef.target {
                    fio::Type::NilType(t) => ns.add_type(
                        TypeDef::NilType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Nil::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::AnyType(t) => ns.add_type(
                        TypeDef::AnyType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Any::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::BuiltinType(t) => ns.add_type(
                        TypeDef::BuiltinType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Builtin::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::RefType(t) => ns.add_type(
                        TypeDef::RefType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Ref::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::SeqType(t) => ns.add_type(
                        TypeDef::SeqType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Seq::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::SetType(t) => ns.add_type(
                        TypeDef::SetType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Set::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::UnionType(t) => ns.add_type(
                        TypeDef::UnionType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Union::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::StructType(t) => ns.add_type(
                        TypeDef::StructType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Struct::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::SubType(t) => ns.add_type(
                        TypeDef::SubType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Sub::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::TupleType(t) => ns.add_type(
                        TypeDef::TupleType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Tuple::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                    fio::Type::RelationType(t) => ns.add_type(
                        TypeDef::RelationType(TypeDefStr {
                            name: typedef.name.clone(),
                            target: Rc::new(RefCell::new(Relation::from_fio(t))),
                        }),
                        &mut type_map,
                    ),
                }
            }
            ns.resolve(&type_map)?;
            schemas.insert(path.clone(), ns);
        }
        Ok(schemas)
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
