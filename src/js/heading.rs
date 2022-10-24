use snafu::{Whatever, whatever};
use crate::schema::{FinitioType, heading::{Heading}};

impl FinitioType<serde_json::Value> for Heading {
    fn include(&self, v: &serde_json::Value) -> Result<(), Whatever> {
        match v {
            serde_json::Value::Object(obj) => {
                // check for extra props
                let extras: Vec<String> = obj
                    .keys()
                    .filter(|prop| !self.attributes.contains_key(*prop))
                    .map(|s| String::from(s))
                    .collect();

                if extras.len() > 0 {
                    whatever!("The objet has extra properties: {}", extras.join(","))
                }

                // check for missing props
                let missing: Vec<String> = self.attributes
                    .values()
                    .filter(|prop| {
                        !prop.optional && !obj.contains_key(&prop.name)
                    })
                    .map(|a| {
                        a.name.clone()
                    })
                    .collect();

                if missing.len() > 0 {
                    whatever!("The objet is missing properties: {}", missing.join(","))
                }

                // validate all properties
                let mut errors = obj.iter().fold(Vec::new(), |mut errors, (name, value)| {
                    let att = self.attributes.get(name).unwrap();
                    let is_valid = att.att_type.include(value);
                    match is_valid {
                        Ok(_) => (),
                        Err(_) => errors.push((att, value)),
                    }
                    errors
                });

                if errors.is_empty() {
                    Ok(())
                } else {
                    let (attr, value) = errors.pop().unwrap();
                    let err = attr.att_type.include(value);
                    whatever!(err, "Invalid value for attribute: {}", attr.name);
                    Ok(())
                }
            },
            v => whatever!("Value not compatible with heading: {}", v)
        }
    }
}
