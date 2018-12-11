use std::fmt::Debug;
use std::str::FromStr;

/**
 *  Key for an item in a table.
 *  Keys get implemented for each table type. This allows each table to define their own definition
 *  of a key, including what might be used in the actual table. For example, the VecTable defines
 *  a VecTableKey that has an usize for an index into a vector. The E type parameter is used to
 *  ensure that a key cannot be used for any entry other than the one it was created for.
 *
 *  The Downcast trait allows us to turn a Box<dyn Key> into the concrete type that it came from
*/
pub trait Key<E: Entry<T>, T: Table<E>>: ToString + Debug + PartialEq { }

pub trait ITryInto<T> {
    fn itry_into(self) -> Result<T, String>;
}

#[derive(Debug, Clone)]
pub enum Value<E: Entry<T>, T: Table<E>> {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Key(T::Key),
}

impl<E: Entry<T>, T: Table<E>> PartialEq for Value<E, T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Integer(i) => if let Value::Integer(j) = other { i == j } else { false },
            Value::Float(i) => if let Value::Float(j) = other { i == j } else { false },
            Value::String(i) => if let Value::String(j) = other { i == j } else { false },
            Value::Boolean(i) => if let Value::Boolean(j) = other { i == j } else { false },
            Value::Key(i) => if let Value::Key(j) = other { i == j } else { false },
        }
    }
}

impl<E: Entry<T>, T:Table<E>> ToString for Value<E, T> {
    fn to_string(&self) -> String {
        match self {
            Value::Integer(temp) => temp.to_string(),
            Value::Float(temp) => temp.to_string(),
            Value::String(temp) => temp.clone(),
            Value::Boolean(temp) => temp.to_string(),
            Value::Key(temp) => temp.to_string(),
        }
    }
}

impl<E: Entry<T>, T:Table<E>> ITryInto<i32> for Value<E, T> {
    fn itry_into(self) -> Result<i32, String> {
        match self {
            Value::Integer(temp) => Ok(temp),
            _ => Err("Converted value to wrong type".to_string()),
        }
    }
}

impl<E: Entry<T>, T:Table<E>> ITryInto<f32> for Value<E, T> {
    fn itry_into(self) -> Result<f32, String> {
        match self {
            Value::Float(temp) => Ok(temp),
            _ => Err("Converted value to wrong type".to_string()),
        }
    }
}

impl<E: Entry<T>, T:Table<E>> ITryInto<String> for Value<E, T> {
    fn itry_into(self) -> Result<String, String> {
        match self {
            Value::String(temp) => Ok(temp),
            _ => Err("Converted value to wrong type".to_string()),
        }
    }
}

impl<E: Entry<T>, T:Table<E>> ITryInto<bool> for Value<E, T> {
    fn itry_into(self) -> Result<bool, String> {
        match self {
            Value::Boolean(temp) => Ok(temp),
            _ => Err("Converted value to wrong type".to_string()),
        }
    }
}

pub trait FieldName: PartialEq + Copy + Clone + Debug + FromStr + ToString {}

/**
 *  Entry in a table. Things that implement this are stored in the database
*/
pub trait Entry<T: Table<Self>>: Clone {
    type FieldNames: FieldName;

    fn from_fields(values: &[Value<Self, T>]) -> Result<Self, String>;
    fn get_field_names() -> Vec<Self::FieldNames>;
    fn get_fields(&self) -> Vec<Value<Self, T>>;
    fn get_field(&self, field_name: Self::FieldNames) -> Value<Self, T>;
}

/**
 * A table in a database that can store entries.
*/
pub trait Table<E: Entry<Self>> where Self: Sized {
    /// The Key type for this database.
    /// Must implement the Key trait
    type Key: Key<E, Self>;

    /// Insert an entry into the table. Returns a key for the entry in the table.
    fn insert(&mut self, entry: E) -> Self::Key;

    /// Find a key in the table. Returns Some(Entry) if the entry for the key is in the table, None
    /// otherwise.
    fn lookup(&self, key: Self::Key) -> Option<E>;

    /// Search for entries in the table with a field matching a value. Returns a vector of keys
    /// and entries for the results.
    fn search(
        &self,
        field_name: E::FieldNames,
        field_value: Value<E, Self>,
    ) -> Result<Vec<(Self::Key, E)>, String>;

    /// Update an entry at a given key with a new entry
    fn update(&self, key: Self::Key, entry: E) -> Result<(), String>;

    /// Removes the entry for the given key in the table. Returns an Ok(()) if successfull,
    /// but an Err(String) is the key could not be found, with an error message in the string.
    fn remove(&mut self, key: Self::Key) -> Result<(), String>;

    /// Check whether a given key is in the table. Returns true if the key is in the table,
    /// false otherwise
    fn contains(&self, key: Self::Key) -> bool;
}
