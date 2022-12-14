use crate::common::FilePosition;
use crate::fio;

use super::constraint::Constraint;
use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Sub {
    pub base_type: Box<Type>,
    pub constraints: Vec<Constraint>,
    pub position: FilePosition,
}

impl Sub {
    pub(crate) fn from_fio(fseq: &fio::SubType) -> Self {
        let base_type = Type::from_fio(&fseq.base);
        let constraints: Vec<Constraint> = fseq.constraints.iter().map(|c| {
            let mut c = Constraint::new(c.param.clone(), c.expr.clone(), c.position.clone());
            c.compile().unwrap();
            c
        }).collect();
        Self {
            base_type: Box::new(base_type),
            constraints,
            position: fseq.position.clone(),
        }
    }

    pub(crate) fn resolve(&mut self, type_map: &TypeMap) -> Result<(), ValidationError> {
        self.base_type.resolve(type_map)?;
        Ok(())
    }
}
