use std::collections::HashMap;

use super::typedef::TypeDef;

pub struct TypeMap {
    map: HashMap<String, TypeDef>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, ftype: &TypeDef) {
        self.map.insert(ftype.name().clone(), ftype.clone());
    }

    pub fn get(&self, name: &String) -> Option<&TypeDef> {
        self.map.get(name)
    }
}
