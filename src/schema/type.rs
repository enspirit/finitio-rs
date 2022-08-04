use std::rc::Rc;
use std::{cell::RefCell, rc::Weak};

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

#[derive(Clone, Debug)]
pub enum Type {
    Nil(FilePosition),
    Any(FilePosition),
    Builtin(String),
    Ref(TypeRef),
    Seq(Seq),
    Set(Set),
    Union(Union),
    Struct(Struct),
    Sub(Sub),
    Tuple(Tuple),
    Relation(Relation),
}

#[derive(Clone, Debug)]
pub enum TypeRef {
    Any(AnyRef),
    Nil(NilRef),
    Builtin(BuiltinRef),
    Ref(RefRef),
    Seq(SeqRef),
    Set(SetRef),
    Union(UnionRef),
    Struct(StructRef),
    Sub(SubRef),
    Tuple(TupleRef),
    Relation(RelationRef),
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
pub struct BuiltinRef {
    pub builtin_: Weak<RefCell<Builtin>>,
}

#[derive(Clone, Debug)]
pub struct RefRef {
    pub ref_: Weak<RefCell<Ref>>,
}

#[derive(Clone, Debug)]
pub struct SeqRef {
    pub seq_: Weak<RefCell<Seq>>,
}

#[derive(Clone, Debug)]
pub struct SetRef {
    pub set_: Weak<RefCell<Set>>,
}
#[derive(Clone, Debug)]
pub struct UnionRef {
    pub union_: Weak<RefCell<Union>>,
}

#[derive(Clone, Debug)]
pub struct StructRef {
    pub struct_: Weak<RefCell<Struct>>,
}

#[derive(Clone, Debug)]
pub struct SubRef {
    pub sub_: Weak<RefCell<Sub>>,
}

#[derive(Clone, Debug)]
pub struct TupleRef {
    pub tuple_: Weak<RefCell<Tuple>>,
}

#[derive(Clone, Debug)]
pub struct RelationRef {
    pub relation_: Weak<RefCell<Relation>>,
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
            Self::Union(r) => r.union_.upgrade().unwrap().borrow().position.clone(),
            Self::Struct(r) => r.struct_.upgrade().unwrap().borrow().position.clone(),
            Self::Sub(r) => r.sub_.upgrade().unwrap().borrow().position.clone(),
            Self::Tuple(r) => r.tuple_.upgrade().unwrap().borrow().position.clone(),
            Self::Relation(r) => r.relation_.upgrade().unwrap().borrow().position.clone(),
            Self::Unresolved { name: _name, position } => position.clone(),
        }
    }
    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        if let Self::Unresolved { name, position: _position } = self {
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

impl Type {
    pub fn from_fio(ftype: &fio::Type) -> Self {
        match ftype {
            fio::Type::NilType(t) => Self::Nil(t.position.clone()),
            fio::Type::AnyType(t) => Self::Any(t.position.clone()),
            fio::Type::BuiltinType(t) => Self::Builtin(t.name.clone()),
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

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
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
