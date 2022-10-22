use crate::schema::{TypeInclude, tuple::{Tuple}, heading::{Heading, Attribute}};

impl TypeInclude<serde_json::Value> for Heading {
    fn include(&self, v: &serde_json::Value) -> Result<bool, &'static str> {
        match v {
            serde_json::Value::Object(obj) => {
                // check for extra props
                let extras = obj.keys().filter(|prop| !self.attributes.contains_key(*prop));
                if extras.peekable().peek().is_some() {
                    return Err("The tuple has extra properties")
                }

                // check for missing props
                let missing = self.attributes.values().filter(|prop| {
                    !prop.optional && !obj.contains_key(&prop.name)
                });
                if missing.peekable().peek().is_some() {
                    return Err("The tuple is missing properties")
                }

                // validate all properties
                let invalid = obj.iter().filter(|(name, value)| {
                    let att = self.attributes.get(*name).unwrap();
                    let is_valid = att.att_type.include(value);
                    match is_valid {
                        Ok(_) => false,
                        Err(_) => true,
                    }
                });

                if invalid.peekable().peek().is_some() {
                    Err("Some properties have invalid values")
                } else {
                    Ok(true)
                }
            },
            _ => Err("Invalid source type")
        }
    }
}
