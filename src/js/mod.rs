mod schema;
mod r#type;
mod any;
mod nil;
mod r#ref;
mod builtin;
mod relation;
mod seq;
mod set;
mod union;
mod sub;
mod tuple;
mod r#struct;

pub use schema::generate_json;
