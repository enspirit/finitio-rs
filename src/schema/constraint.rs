use evalexpr::{build_operator_tree, Node};
use snafu::{Whatever, ResultExt, whatever};

use crate::common::FilePosition;

#[derive(Clone, Debug)]
pub struct Constraint {
    pub param: String,
    pub expr: String,
    pub position: FilePosition,
    pub expr_node: Option<Node>
}

impl Constraint {

  pub fn new(param: String, expr: String, position: FilePosition) -> Self {
    Self {
        param: param,
        expr: expr,
        position: position,
        expr_node: None
    }
  }

  pub fn compile(&mut self) -> Result<(), Whatever> {
    if self.expr_node.is_some() {
      whatever!("Constraint has already been compiled");
    }
    let expr = build_operator_tree(&self.expr)
      .with_whatever_context(|_| format!("Invalid constraint expression: {}", self.expr))?;

    self.expr_node = Some(expr);
    Ok(())
  }

}

pub trait ConstraintExecute<T> {
  fn execute(&self, _: &T) -> Result<bool, Whatever>;
}

