use crate::schema::{TypeInclude, union::Union};

impl TypeInclude<serde_json::Value> for Union {
    fn include(&self, v: &serde_json::Value) -> Result<bool, std::io::Error> {
        let found = self.candidates.iter().find(|x| {
            match x.include(v) {
                Ok(_) => true,
                Err(_) => false,
            }
        });
        match found {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
