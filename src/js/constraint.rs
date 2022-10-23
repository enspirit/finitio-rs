use std::fmt;
use snafu::{Whatever, ResultExt, whatever};
use crate::{schema::{Constraint, constraint::ConstraintExecute}};
use serde_json::Value;

impl ConstraintExecute<serde_json::Value> for &Constraint {
  fn execute(&self, v: &serde_json::Value) -> Result<bool, Whatever> {

      let node = match self.expr_node.as_ref() {
        None => whatever!("Constraint needs to be compiled before using execute()"),
        Some(node) => node
      };

      let res = node.clone()
        .value(self.param.clone(), v)
        .exec()
        .with_whatever_context(|_| format!("Unable to evaluate constraint: {}", self))?;

      let bool = match res {
        Value::Bool(bool) => bool,
        v => whatever!("Invalid return value for constraint `{}`, expected boolean got: {}", self, v)
      };

      Ok(bool)
  }
}

impl fmt::Display for Constraint {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.expr_node {
      None => write!(f, "NotCompiled({}, {})", self.param, self.expr),
      Some(_) => write!(f, "Compiled({}, {})", self.param, self.expr)
    }
  }
}
