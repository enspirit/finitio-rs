use std::collections::HashMap;

use super::r#type::UserDefinedType;

pub struct TypeMap {
    map: HashMap<String, UserDefinedType>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, ftype: &UserDefinedType) {
        self.map.insert(ftype.name().clone(), ftype.clone());
    }

    pub fn get(&self, name: &String) -> Option<&UserDefinedType> {
        self.map.get(name)
    }
}
