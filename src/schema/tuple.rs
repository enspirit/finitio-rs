use std::collections::HashMap;

use crate::common::FilePosition;
use crate::fio;

use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Tuple {
    pub attributes: HashMap<String, Attribute>,
    pub position: FilePosition,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub att_type: Type,
    pub optional: bool
}

impl Tuple {
    pub(crate) fn from_fio(ftuple: &fio::TupleType) -> Self {
        let attributes = ftuple
            .heading
            .attributes
            .iter()
            .fold(HashMap::new(), |mut acc, att| {
                let att_type = Type::from_fio(&att.att_type);
                let attribute = Attribute {
                    name: att.name.to_string(),
                    att_type,
                    optional: att.optional
                };
                acc.entry(att.name.to_string()).or_insert(attribute);
                acc
            });
        Self {
            attributes,
            position: ftuple.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        for (_, att) in self.attributes.iter_mut() {
            att.att_type.resolve(type_map)?
        }
        Ok(())
    }
}
