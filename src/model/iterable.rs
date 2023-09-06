use chrono::{DateTime, Utc};
use uuid::Uuid;

// TODO: Add more types if needed
#[derive(Debug)]
pub enum IterableType {
    Uuid(Uuid),
    DateTime(DateTime<Utc>),
    Bool(bool),
    String(String),
}

pub trait Iterable {
    // returns an array of the fields of the struct inside an HashMap
    fn get_fields(&self) -> (Vec<String>, Vec<IterableType>);
}
