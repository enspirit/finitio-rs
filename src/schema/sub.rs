use crate::common::FilePosition;
use crate::fio;

use super::constraint::Constraint;
use super::errors::ValidationError;
use super::r#type::Type;
use super::typemap::TypeMap;

#[derive(Clone, Debug)]
pub struct Sub<'a> {
    pub base_type: Box<Type<'a>>,
    pub constraints: Vec<Constraint>,
    pub position: FilePosition,
}

impl<'a> Sub<'a> {
    pub(crate) fn from_fio(fseq: &'a fio::SubType) -> Self {
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

    pub(crate) fn resolve(&mut self, type_map: &'a TypeMap<'a>) -> Result<(), ValidationError> {
        self.base_type.resolve(type_map)?;
        Ok(())
    }
}
