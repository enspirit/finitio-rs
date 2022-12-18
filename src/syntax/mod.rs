pub mod parser;

#[derive(Clone, Debug)]
pub struct FilePosition {
  pub start: (usize, usize),
  pub end: (usize, usize),
}
