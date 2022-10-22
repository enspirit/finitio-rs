use crate::schema::{TypeInclude, any::Any};

impl<T> TypeInclude<T> for Any {
    fn include(&self, _: &T) -> Result<bool, std::io::Error> {
        Ok(true)
    }
}
