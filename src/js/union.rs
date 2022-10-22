use std::{cell::RefCell, rc::Rc};

use crate::schema::{TypeInclude, union::Union};

impl TypeInclude<serde_json::Value> for Union {
    fn include(&self, v: &serde_json::Value) -> Result<bool, &'static str> {
        let found = self.candidates.iter().find(|x| {
            match x.include(v) {
                Ok(_) => true,
                Err(_) => false,
            }
        });
        match found {
            Some(t) => Ok(true),
            None => Err("Could not find any candidate including value"),
        }
    }
}

#[cfg(test)]
use crate::schema::{any::Any, nil::Nil, r#ref::Ref, builtin::Builtin, r#type::Type, r#type::TypeRef, r#type::BuiltinRef};
#[test]
fn test_include_union() {
    use crate::common::FilePosition;

    let position = FilePosition { line: 2, column: 2};

    let any_t = Type::Any(Any {
        position: position.clone()
    });
    let builtin_str = Type::Builtin(Builtin {
        position: position.clone(),
        target: String::from("String")
    });
    let builtin_num = Type::Builtin(Builtin {
        position: position.clone(),
        target: String::from("Number")
    });

    // Union of .Number or .String
    let union_t = Type::Union(Union {
        position: position.clone(),
        candidates: vec![builtin_str, builtin_num]
    });

    // invalid
    let nil = serde_json::Value::Null {};
    assert_eq!(union_t.include(&nil).is_ok(), false, "null is not a valid .Number|.String");

    let obj = serde_json::json!({});
    assert_eq!(union_t.include(&obj).is_ok(), false, "{{}} is not a valid .Number|.String");

    let arr = serde_json::json!([]);
    assert_eq!(union_t.include(&arr).is_ok(), false, "[] is not a valid .Number|.String");

    // Valid
    let number = serde_json::json!(12);
    assert_eq!(union_t.include(&number).is_ok(), true, "12 is a valid .Number|.String");

    let string = serde_json::json!("foo");
    assert_eq!(union_t.include(&string).is_ok(), true, "'string' is a valid .Number|.String");

}
