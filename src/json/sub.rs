use snafu::{Whatever, ResultExt, whatever};
use crate::{schema::{FinitioType, sub::{Sub}, constraint::ConstraintExecute}};

impl FinitioType<serde_json::Value> for Sub {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        self.base_type.include(v)
            .with_whatever_context(|_| format!("Value rejected by base type: {}", v))?;

        for constraint in self.constraints.iter() {
            let valid = constraint
                .execute(v)
                .with_whatever_context(|_| format!("Unable to execute constraint: {}", constraint))?;

            if !valid {
                whatever!("Value rejected by constraint {}: {}", constraint, v);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
use crate::schema::{builtin::Builtin, r#type::Type, Constraint};

#[test]
fn test_include_sub() {
    use crate::common::FilePosition;

    let position = FilePosition { line: 2, column: 2};

    let builtin_num = Type::Builtin(Builtin {
        position: position.clone(),
        target: String::from("Number")
    });

    // PosInt = .Number(i | i > 0)
    let mut constraint = Constraint::new("i".to_string(), "i > 0".to_string(), position.clone());
    constraint.compile().unwrap();

    let sub = Type::Sub(Sub {
        base_type: Box::new(builtin_num),
        constraints: vec![constraint],
        position: position.clone()
    });

    // invalid: wrong base type
    let invalid_type = serde_json::json!("foo");
    assert_eq!(sub.include(&invalid_type).is_ok(), false, "invalid base type");

    // valid: positive integer
    let valid_pos_int = serde_json::json!(10);
    assert_eq!(sub.include(&valid_pos_int).is_ok(), true, "valid pos int");

    // valid: negative integer
    let valid_pos_int = serde_json::json!(-10);
    assert_eq!(sub.include(&valid_pos_int).is_ok(), false, "invalid negative int");
}
