use std::error::Error;
use std::fmt;

use crate::common::FilePosition;

#[derive(Debug)]
pub enum ValidationError {
    DuplicateIdentifier {
        position: FilePosition,
        identifier: String,
    },
    NoSuchType {
        position: FilePosition,
        name: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // FIXME Replace this by a proper implementation of Display
        write!(f, "{:?}", self)
    }
}

impl Error for ValidationError {}
