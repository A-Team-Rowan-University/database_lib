use std::fmt::Debug;

/**
 *  Key for an item in a table.
 *  Keys get implemented for each table type. This allows each table to define their own definition
 *  of a key, including what might be used in the actual table. For example, the VecTable defines
 *  a VecTableKey that has an usize for an index into a vector. The E type parameter is used to
 *  ensure that a key cannot be used for any entry other than the one it was created for.
 *
 *  The Downcast trait allows us to turn a Box<dyn Key> into the concrete type that it came from
 */
pub trait Key<E: Entry>: Debug + PartialEq { }

pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
}

/**
 *  Entry in a table. Things that implement this are stored in the database
 */
pub trait Entry: Clone {
    fn from_fields(values: &[Value]) -> Result<Self, String>;
    fn get_field_names() -> Vec<String>;
    fn get_fields(&self) -> Vec<Value>;
}

pub trait Table<E: Entry> {
    type Key: Key<E>;
    fn insert(&mut self, entry: E) -> Self::Key;
    fn lookup(&self, key: Self::Key) -> Option<E>;
}


