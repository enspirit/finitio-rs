use std::rc::Rc;
use std::{cell::RefCell, rc::Weak};

use snafu::Whatever;

use crate::common::FilePosition;
use crate::fio;

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
use super::typedef::TypeDef;
use super::union::Union;
use super::{errors::ValidationError, typemap::TypeMap};

pub trait TypeInclude<T> {
    fn include(&self, _: &T) -> Result<(), Whatever>;
}

#[derive(Clone, Debug)]
pub enum Type<'a> {
    Nil(Nil),
    Any(Any),
    Builtin(Builtin<'a>),
    Ref(TypeRef<'a>),
    Seq(Seq<'a>),
    Set(Set<'a>),
    Union(Union<'a>),
    Struct(Struct<'a>),
    Sub(Sub<'a>),
    Tuple(Tuple<'a>),
    Relation(Relation<'a>),
}

#[derive(Clone, Debug)]
pub enum TypeRef<'a> {
    Any(AnyRef),
    Nil(NilRef),
    Builtin(BuiltinRef<'a>),
    Ref(RefRef<'a>),
    Seq(SeqRef<'a>),
    Set(SetRef<'a>),
    Union(UnionRef<'a>),
    Struct(StructRef<'a>),
    Sub(SubRef<'a>),
    Tuple(TupleRef<'a>),
    Relation(RelationRef<'a>),
    Unresolved {
        name: String,
        position: FilePosition,
    },
}

#[derive(Clone, Debug)]
pub struct AnyRef {
    pub any_: Weak<RefCell<Any>>,
}

#[derive(Clone, Debug)]
pub struct NilRef {
    pub nil_: Weak<RefCell<Nil>>,
}

#[derive(Clone, Debug)]
pub struct BuiltinRef<'a> {
    pub builtin_: Weak<RefCell<Builtin<'a>>>,
}

#[derive(Clone, Debug)]
pub struct RefRef<'a> {
    pub ref_: Weak<RefCell<Ref<'a>>>,
}

#[derive(Clone, Debug)]
pub struct SeqRef<'a> {
    pub seq_: Weak<RefCell<Seq<'a>>>,
}

#[derive(Clone, Debug)]
pub struct SetRef<'a> {
    pub set_: Weak<RefCell<Set<'a>>>,
}
#[derive(Clone, Debug)]
pub struct UnionRef<'a> {
    pub union_: Weak<RefCell<Union<'a>>>,
}

#[derive(Clone, Debug)]
pub struct StructRef<'a> {
    pub struct_: Weak<RefCell<Struct<'a>>>,
}

#[derive(Clone, Debug)]
pub struct SubRef<'a> {
    pub sub_: Weak<RefCell<Sub<'a>>>,
}

#[derive(Clone, Debug)]
pub struct TupleRef<'a> {
    pub tuple_: Weak<RefCell<Tuple<'a>>>,
}

#[derive(Clone, Debug)]
pub struct RelationRef<'a> {
    pub relation_: Weak<RefCell<Relation<'a>>>,
}

impl<'a> TypeRef<'a> {
    pub fn position(&self) -> FilePosition {
        match self {
            Self::Any(r) => r.any_.upgrade().unwrap().borrow().position.clone(),
            Self::Nil(r) => r.nil_.upgrade().unwrap().borrow().position.clone(),
            Self::Builtin(r) => r.builtin_.upgrade().unwrap().borrow().position.clone(),
            Self::Ref(r) => r.ref_.upgrade().unwrap().borrow().position.clone(),
            Self::Seq(r) => r.seq_.upgrade().unwrap().borrow().position.clone(),
            Self::Set(r) => r.set_.upgrade().unwrap().borrow().position.clone(),
            Self::Union(r) => r.union_.upgrade().unwrap().borrow().position.clone(),
            Self::Struct(r) => r.struct_.upgrade().unwrap().borrow().position.clone(),
            Self::Sub(r) => r.sub_.upgrade().unwrap().borrow().position.clone(),
            Self::Tuple(r) => r.tuple_.upgrade().unwrap().borrow().position.clone(),
            Self::Relation(r) => r.relation_.upgrade().unwrap().borrow().position.clone(),
            Self::Unresolved {
                name: _name,
                position,
            } => position.clone(),
        }
    }
    pub(crate) fn resolve(&mut self, type_map: &'a TypeMap<'a>) -> Result<(), ValidationError> {
        if let Self::Unresolved {
            name,
            position: _position,
        } = self
        {
            let ftype = type_map.get(name);
            *self = match ftype {
                Some(udtype) => match udtype {
                    TypeDef::AnyType(any_) => TypeRef::Any(AnyRef {
                        any_: Rc::downgrade(&any_.target),
                    }),
                    TypeDef::NilType(nil_) => TypeRef::Nil(NilRef {
                        nil_: Rc::downgrade(&nil_.target),
                    }),
                    TypeDef::BuiltinType(builtin_) => TypeRef::Builtin(BuiltinRef {
                        builtin_: Rc::downgrade(&builtin_.target),
                    }),
                    TypeDef::RefType(ref_) => TypeRef::Ref(RefRef {
                        ref_: Rc::downgrade(&ref_.target),
                    }),
                    TypeDef::SeqType(seq_) => TypeRef::Seq(SeqRef {
                        seq_: Rc::downgrade(&seq_.target),
                    }),
                    TypeDef::SetType(set_) => TypeRef::Set(SetRef {
                        set_: Rc::downgrade(&set_.target),
                    }),
                    TypeDef::UnionType(union_) => TypeRef::Union(UnionRef {
                        union_: Rc::downgrade(&union_.target),
                    }),
                    TypeDef::StructType(struct_) => TypeRef::Struct(StructRef {
                        struct_: Rc::downgrade(&struct_.target),
                    }),
                    TypeDef::SubType(sub_) => TypeRef::Sub(SubRef {
                        sub_: Rc::downgrade(&sub_.target),
                    }),
                    TypeDef::TupleType(tuple_) => TypeRef::Tuple(TupleRef {
                        tuple_: Rc::downgrade(&tuple_.target),
                    }),
                    TypeDef::RelationType(relation_) => TypeRef::Relation(RelationRef {
                        relation_: Rc::downgrade(&relation_.target),
                    }),
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

impl<'a> Type<'a> {
    pub fn from_fio(ftype: &'a fio::Type) -> Self {
        match ftype {
            fio::Type::NilType(t) => Self::Nil(Nil::from_fio(t)),
            fio::Type::AnyType(t) => Self::Any(Any::from_fio(t)),
            fio::Type::BuiltinType(t) => Self::Builtin(Builtin::from_fio(t)),
            fio::Type::RefType(t) => Self::Ref(TypeRef::Unresolved {
                name: t.name.clone(),
                position: t.position.clone(),
            }),
            fio::Type::SeqType(t) => Self::Seq(Seq::from_fio(t)),
            fio::Type::SetType(t) => Self::Set(Set::from_fio(t)),
            fio::Type::UnionType(t) => Self::Union(Union::from_fio(t)),
            fio::Type::StructType(t) => Self::Struct(Struct::from_fio(t)),
            fio::Type::SubType(t) => Self::Sub(Sub::from_fio(t)),
            fio::Type::TupleType(t) => Self::Tuple(Tuple::from_fio(t)),
            fio::Type::RelationType(t) => Self::Relation(Relation::from_fio(t)),
        }
    }

    pub fn from_fio_ref(fref: &fio::RefType) -> Self {
        Self::Ref(TypeRef::Unresolved {
            name: fref.name.clone(),
            position: fref.position.clone(),
        })
    }

    pub(crate) fn resolve(&mut self, type_map: &'a TypeMap<'a>) -> Result<(), ValidationError> {
        match self {
            Self::Nil(_) | Self::Any(_) | Self::Builtin(_) => Ok(()), // Should probably not consider all Builtin ok here?

            Self::Ref(tref) => tref.resolve(type_map),
            Self::Seq(sref) => sref.resolve(type_map),
            Self::Set(sref) => sref.resolve(type_map),
            Self::Union(uref) => uref.resolve(type_map),
            Self::Struct(sref) => sref.resolve(type_map),
            Self::Sub(sref) => sref.resolve(type_map),
            Self::Tuple(tref) => tref.resolve(type_map),
            Self::Relation(rref) => rref.resolve(type_map),
        }
    }
}
