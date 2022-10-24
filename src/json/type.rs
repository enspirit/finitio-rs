use snafu::Whatever;

use crate::schema::{FinitioType, Type, TypeDef};

impl FinitioType<serde_json::Value> for Type {
  fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
      match self {
          Type::Any(t) => t.include(v),
          Type::Nil(t) => t.include(v),
          Type::Builtin(t) => t.include(v),
          Type::Ref(t) => t.include(v),
          Type::Seq(t) => t.include(v),
          Type::Set(t) => t.include(v),
          Type::Union(t) => t.include(v),
          Type::Struct(t) => t.include(v),
          Type::Sub(t) => t.include(v),
          Type::Tuple(t) => t.include(v),
          Type::Relation(t) => t.include(v),
      }
  }
  fn dress(&self, v: &serde_json::Value) -> Result<serde_json::Value, Whatever> {
      match self {
          Type::Any(t) => t.dress(v),
          Type::Nil(t) => t.dress(v),
          Type::Builtin(t) => t.dress(v),
          Type::Ref(t) => t.dress(v),
          Type::Seq(t) => t.dress(v),
          Type::Set(t) => t.dress(v),
          Type::Union(t) => t.dress(v),
          Type::Struct(t) => t.dress(v),
          Type::Sub(t) => t.dress(v),
          Type::Tuple(t) => t.dress(v),
          Type::Relation(t) => t.dress(v),
      }
  }
}

impl FinitioType<serde_json::Value> for TypeDef {
  fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
      match self {
        TypeDef::AnyType(t) => t.target.borrow().include(v),
        TypeDef::NilType(t) => t.target.borrow().include(v),
        TypeDef::BuiltinType(t) => t.target.borrow().include(v),
        TypeDef::RefType(t) => t.target.borrow().include(v),
        TypeDef::SeqType(t) => t.target.borrow().include(v),
        TypeDef::SetType(t) => t.target.borrow().include(v),
        TypeDef::UnionType(t) => t.target.borrow().include(v),
        TypeDef::StructType(t) => t.target.borrow().include(v),
        TypeDef::SubType(t) => t.target.borrow().include(v),
        TypeDef::TupleType(t) => t.target.borrow().include(v),
        TypeDef::RelationType(t) => t.target.borrow().include(v),
    }
  }
  fn dress(&self, v: &serde_json::Value) -> Result<serde_json::Value, Whatever> {
      match self {
        TypeDef::AnyType(t) => t.target.borrow().dress(v),
        TypeDef::NilType(t) => t.target.borrow().dress(v),
        TypeDef::BuiltinType(t) => t.target.borrow().dress(v),
        TypeDef::RefType(t) => t.target.borrow().dress(v),
        TypeDef::SeqType(t) => t.target.borrow().dress(v),
        TypeDef::SetType(t) => t.target.borrow().dress(v),
        TypeDef::UnionType(t) => t.target.borrow().dress(v),
        TypeDef::StructType(t) => t.target.borrow().dress(v),
        TypeDef::SubType(t) => t.target.borrow().dress(v),
        TypeDef::TupleType(t) => t.target.borrow().dress(v),
        TypeDef::RelationType(t) => t.target.borrow().dress(v),
    }
  }
}
