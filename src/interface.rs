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
pub trait ITryInto<T> {
    fn itry_into(self) -> Result<T, String>;
}


#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
	Boolean(bool),
}
pub enum QueryType{
	Lookup,
	Search,
	GetAll,
	PartialSearch,
	LimitSearch,
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
			"".to_string()
			//If you got here, how did you not pass a Value when you ARE a Value
		}
		
	}
}
impl ITryInto<i32> for Value{
	fn itry_into (self) -> Result<i32, String> {
		if let Value::Integer(temp) = self{
			Ok(temp)
		}else{
			Err("Converted value to wrong type".to_string())
}}}
impl ITryInto<f32> for Value{
	fn itry_into (self)-> Result<f32, String> {
		if let Value::Float(temp) = self{
			Ok(temp)
		}else{
			Err("Converted value to wrong type".to_string())
}}}
impl ITryInto<String> for Value{
	fn itry_into(self) -> Result<String, String> {
		if let Value::String(temp) = self{
			Ok(temp)
		}else{
			Err("Converted value to wrong type".to_string())
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
    fn insert(&self, entry: E) -> Self::Key;

    /// Find a key in the table. Returns Some(Entry) if the entry for the key is in the table, None
    /// otherwise.
    fn lookup(&self, key: Self::Key) -> Option<E>;

    /// Search for entries in the table with a field matching a value. Returns a vector of keys
    /// and entries for the results.
    fn search(&self, field_name: E::FieldNames, field_value: Value) -> Result<Vec<(Self::Key, E)>,String>;
	
	/// Update an entry at a given key with a new entry
	fn update(&self, key: Self::Key, entry: E)-> Result<(), String>;

    /// Removes the entry for the given key in the table. Returns an Ok(()) if successfull,
    /// but an Err(String) is the key could not be found, with an error message in the string.
    fn remove(&mut self, key: Self::Key) -> Result<(), String>;

    /// Check whether a given key is in the table. Returns true if the key is in the table,
    /// false otherwise
    fn contains(&self, key: Self::Key) -> bool;
	
	/// Generic Query Builder
	/// When complete, it will replce lookup and search
	/// If the query does not require a key, input DEFAULT_KEY
	/// This allows for easy addition of more query types
	fn query(&self, q: QueryType,  data:Vec<Value>, key: Self::Key) -> Result<Vec<(Self::Key, E)>,String>;
}
