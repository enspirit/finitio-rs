use snafu::Whatever;

use crate::schema::{TypeInclude, r#ref::Ref, TypeRef};

impl TypeInclude<serde_json::Value> for Ref<'_> {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        self.target.include(v)
    }
}

impl TypeInclude<serde_json::Value> for TypeRef<'_> {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        match self {
            TypeRef::Any(t) => {
                t.any_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Nil(t) => {
                t.nil_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Builtin(t) => {
                t.builtin_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Ref(t) => {
                t.ref_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Seq(t) => {
                t.seq_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Set(t) => {
                t.set_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Union(t) => {
                t.union_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Struct(t) => {
                t.struct_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Sub(t) => {
                t.sub_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Tuple(t) => {
                t.tuple_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Relation(t) => {
                t.relation_.upgrade().unwrap().borrow().include(v)
            },
            TypeRef::Unresolved { name: _, position: _ } => todo!(),
        }
    }
}


#[cfg(test)]
use crate::schema::{builtin::Builtin, r#type::Type};
#[test]
fn test_include_builtin() {
    use crate::common::FilePosition;

    let position = FilePosition { line: 2, column: 2};

    let str = Ref {
        position: position.clone(),
        target: Type::Builtin(Builtin {
            position: position.clone(),
            target: "String"
        })
    };

    let nil = serde_json::Value::Null {};
    assert_eq!(str.include(&nil).is_ok(), false);

    let number = serde_json::json!(12);
    assert_eq!(str.include(&number).is_ok(), false);

    let string = serde_json::json!("foo");
    assert_eq!(str.include(&string).is_ok(), true);

    let obj = serde_json::json!({});
    assert_eq!(str.include(&obj).is_ok(), false);

    let arr = serde_json::json!([]);
    assert_eq!(str.include(&arr).is_ok(), false);
}
