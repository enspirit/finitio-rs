use std::cell::RefCell;
use std::rc::Rc;

use super::any::Any;
use super::builtin::Builtin;
use super::nil::Nil;
use super::r#ref::Ref;
use super::r#struct::Struct;
use super::relation::Relation;
use super::seq::Seq;
use super::set::Set;
use super::sub::Sub;
use super::tuple::Tuple;
use super::union::Union;
use super::{errors::ValidationError, typemap::TypeMap};

#[derive(Clone, Debug)]
pub enum TypeDef<'a> {
    AnyType(TypeDefStr<Any>),
    NilType(TypeDefStr<Nil>),
    BuiltinType(TypeDefStr<Builtin<'a>>),
    RefType(TypeDefStr<Ref<'a>>),
    SeqType(TypeDefStr<Seq<'a>>),
    SetType(TypeDefStr<Set<'a>>),
    UnionType(TypeDefStr<Union<'a>>),
    StructType(TypeDefStr<Struct<'a>>),
    SubType(TypeDefStr<Sub<'a>>),
    TupleType(TypeDefStr<Tuple<'a>>),
    RelationType(TypeDefStr<Relation<'a>>),
}

#[derive(Clone, Debug)]
pub struct TypeDefStr<T> {
    pub name: String,
    pub target: Rc<RefCell<T>>,
}

impl<'a> TypeDef<'a> {
    pub fn name(&self) -> String {
        match self {
            TypeDef::AnyType(t) => t.name.clone(),
            TypeDef::NilType(t) => t.name.clone(),
            TypeDef::BuiltinType(t) => t.name.clone(),
            TypeDef::RefType(t) => t.name.clone(),
            TypeDef::SeqType(t) => t.name.clone(),
            TypeDef::SetType(t) => t.name.clone(),
            TypeDef::UnionType(t) => t.name.clone(),
            TypeDef::StructType(t) => t.name.clone(),
            TypeDef::SubType(t) => t.name.clone(),
            TypeDef::TupleType(t) => t.name.clone(),
            TypeDef::RelationType(t) => t.name.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &'a TypeMap<'a>) -> Result<(), ValidationError> {
        match self {
            TypeDef::AnyType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::NilType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::BuiltinType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::RefType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::SeqType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::SetType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::UnionType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::StructType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::SubType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::TupleType(t) => t.target.borrow_mut().resolve(type_map),
            TypeDef::RelationType(t) => t.target.borrow_mut().resolve(type_map),
        }
    }
}
