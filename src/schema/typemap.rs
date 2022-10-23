use std::{collections::{HashMap}, fmt};

use super::typedef::TypeDef;

pub struct TypeMap<'a> {
    map: HashMap<String, TypeDef<'a>>,
}

impl<'a> TypeMap<'a> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, ftype: &'a TypeDef<'a>) {
        self.map.insert(ftype.name().clone(), ftype.clone());
    }

    pub fn concat(&mut self, others: &'a Vec<TypeDef<'a>>) {
        for typedef in others.iter() {
            self.map.insert(typedef.name().clone(), typedef.clone());
        }
    }

    pub fn get(&self, name: &String) -> Option<&'a TypeDef> {
        self.map.get(name)
    }
}

impl fmt::Display for TypeMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names: Vec<String> = self.map.keys().map(|s| s.clone()).collect();
        write!(f, "TypeMap({})", names.join(", "))
    }
}
