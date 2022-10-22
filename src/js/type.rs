use std::io::Error;

use crate::schema::{TypeInclude, Type};

impl TypeInclude<serde_json::Value> for Type {
  fn include(&self, v: &serde_json::Value) -> Result<bool, &'static str> {
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
}
