use std::{borrow::Borrow, rc::Rc};

use crate::schema::{TypeInclude, r#ref::Ref, TypeRef};

impl TypeInclude<serde_json::Value> for Ref {
    fn include(&self, v: &serde_json::Value) -> Result<bool, std::io::Error> {
        self.target.include(v)
    }
}

impl TypeInclude<serde_json::Value> for TypeRef {
    fn include(&self, v: &serde_json::Value) -> Result<bool, std::io::Error> {
        match self {
            TypeRef::Any(t) => {
                t.any_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Nil(t) => {
                t.nil_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Builtin(t) => {
                t.builtin_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Ref(t) => {
                t.ref_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Seq(t) => {
                t.seq_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Set(t) => {
                t.set_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Union(t) => {
                t.union_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Struct(t) => {
                t.struct_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Sub(t) => {
                t.sub_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Tuple(t) => {
                t.tuple_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Relation(t) => {
                t.relation_.upgrade().unwrap().borrow_mut().include(v)
            },
            TypeRef::Unresolved { name, position } => todo!(),
        }
    }
}
