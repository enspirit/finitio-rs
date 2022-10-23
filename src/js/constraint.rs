use std::fmt;
use snafu::{Whatever, ResultExt, whatever};
use evalexpr::*;
use crate::{schema::{TypeInclude, sub::{Sub}, Constraint, constraint::ConstraintExecute}, js::builtin};
use serde_json::Value;

fn from_serde_to_evalexpr_value(v: &serde_json::Value) -> Result<evalexpr::Value, Whatever> {
  match v {
      Value::Null => Ok(evalexpr::Value::Empty),
      Value::Bool(b) => Ok(evalexpr::Value::Boolean(*b)),
      Value::Number(n) => Ok(evalexpr::Value::Float(n.as_f64().unwrap())),
      Value::String(s) => Ok(evalexpr::Value::String(s.clone())),
      Value::Array(a) =>todo!(),
      Value::Object(_) => todo!(),
  }
}

impl ConstraintExecute<serde_json::Value> for &Constraint {
  fn execute(&self, v: &serde_json::Value) -> Result<bool, Whatever> {

      let node = match self.expr_node.as_ref() {
        None => whatever!("Constraint needs to be compiled before using execute()"),
        Some(node) => node
      };

      let value = from_serde_to_evalexpr_value(v)
        .with_whatever_context(|_| format!("Unable to convert value from serde_json::Value to evalexpr::Value"))?;
      let mut context = HashMapContext::new();
      context.set_value(self.param.clone(), value)
        .with_whatever_context(|_| format!("Unable to construct execution context for constraint: {}", self))?;

      let res = node.eval_boolean_with_context(&context)
        .with_whatever_context(|_| format!("Constraint `{}`, execution failed", self.expr))?;

      Ok(res)
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
