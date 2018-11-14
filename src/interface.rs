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
pub trait Key<E: Entry>: Debug + PartialEq {}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
}
impl ToString for Value{
	fn to_string(&self) -> String{
		if let Value::Integer(temp) = self{
			temp.to_owned().to_string()
		}else if let Value::Float(temp) = self{
			temp.to_owned().to_string()
		}else if let Value::String(temp) = self{
			temp.to_owned().to_string()
		} else {
			panic!("Could not convert Value");
		}
		
	}
}
impl Into<i32> for Value{
	fn into (self) -> i32 {
		if let Value::Integer(temp) = self{
			temp
		}else{
			panic!("Converted value to wrong type");
}}}
impl Into<f32> for Value{
	fn into (self)-> f32 {
		if let Value::Float(temp) = self{
			temp
		}else{
			panic!("Converted value to wrong type");
}}}
impl Into<String> for Value{
	fn into(self) -> String {
		if let Value::String(temp) = self{
			temp
		}else{
			panic!("Converted value to wrong type");
}}}



pub trait FieldName: PartialEq + Copy + Clone + Debug + FromStr + ToString {}

/**
 *  Entry in a table. Things that implement this are stored in the database
*/
pub trait Entry: Clone {
    type FieldNames: FieldName;

    fn from_fields(values: &[Value]) -> Result<Self, String>;
    fn get_field_names() -> Vec<Self::FieldNames>;
    fn get_fields(&self) -> Vec<Value>;
    fn get_field(&self, field_name: Self::FieldNames) -> Option<Value>;
}

/**
 * A table in a database that can store entries.
*/
pub trait Table<E: Entry> {

    /// The Key type for this database.
    /// Must implement the Key trait
    type Key: Key<E>;

    /// Insert an entry into the table. Returns a key for the entry in the table.
    fn insert(&mut self, entry: E) -> Self::Key;

    /// Find a key in the table. Returns Some(Entry) if the entry for the key is in the table, None
    /// otherwise.
    fn lookup(&self, key: Self::Key) -> Option<E>;

    /// Search for entries in the table with a field matching a value. Returns a vector of keys
    /// and entries for the results.
    fn search(&self, field_name: E::FieldNames, field_value: Value) -> Vec<(Self::Key, E)>;

    /// Removes the entry for the given key in the table. Returns an Ok(()) if successfull,
    /// but an Err(String) is the key could not be found, with an error message in the string.
    fn remove(&mut self, key: Self::Key) -> Result<(), String>;

    /// Check whether a given key is in the table. Returns true if the key is in the table,
    /// false otherwise
    fn contains(&self, key: Self::Key) -> bool;
}
