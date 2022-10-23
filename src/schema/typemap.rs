use std::{collections::{HashMap, BTreeMap}, fmt};

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

    pub fn concat(&mut self, others: &Vec<TypeDef>) {
        for typedef in others.iter() {
            self.map.insert(typedef.name().clone(), typedef.clone());
        }
    }

    pub fn get(&self, name: &String) -> Option<&TypeDef> {
        self.map.get(name)
    }
}

impl fmt::Display for TypeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names: Vec<String> = self.map.keys().map(|s| s.clone()).collect();
        write!(f, "TypeMap({})", names.join(", "))
    }
}
