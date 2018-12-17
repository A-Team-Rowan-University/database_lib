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

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Integer(temp) => temp.to_string(),
            Value::Float(temp) => temp.to_string(),
            Value::String(temp) => temp.clone(),
            Value::Boolean(temp) => temp.to_string(),
        }
    }
}

impl ITryInto<i32> for Value {
    fn itry_into(self) -> Result<i32, String> {
        match self {
            Value::Integer(temp) => Ok(temp),
            _ => Err("Converted value to wrong type".to_string()),
        }
    }
}

impl ITryInto<f32> for Value {
    fn itry_into(self) -> Result<f32, String> {
        match self {
            Value::Float(temp) => Ok(temp),
            _ => Err("Converted value to wrong type".to_string()),
        }
    }
}

impl ITryInto<String> for Value {
    fn itry_into(self) -> Result<String, String> {
        match self {
            Value::String(temp) => Ok(temp),
            _ => Err("Converted value to wrong type".to_string()),
        }
    }
}

impl ITryInto<bool> for Value {
    fn itry_into(self) -> Result<bool, String> {
        match self {
            Value::Boolean(temp) => Ok(temp),
            _ => Err("Converted value to wrong type".to_string()),
        }
    }
}

mod value_tests {

    use interface::ITryInto;
    use interface::Value;

    #[test]
    fn value_string_to_string() {
        assert_eq!(
            Value::String("hello".to_string()).to_string(),
            "hello".to_string()
        );
    }

    #[test]
    fn value_integer_to_string() {
        assert_eq!(Value::Integer(42).to_string(), "42".to_string());
    }

    #[test]
    fn value_float_to_string() {
        assert_eq!(Value::Float(42.612).to_string(), "42.612".to_string());
    }

    #[test]
    fn value_bool_to_string() {
        assert_eq!(Value::Boolean(true).to_string(), "true".to_string());
    }

    #[test]
    fn value_into_i32() {
        assert_eq!(Value::Integer(42).itry_into(), Ok(42));
    }

    #[test]
    fn value_into_f32() {
        assert_eq!(Value::Float(42.0).itry_into(), Ok(42.0));
    }

    #[test]
    fn value_into_string() {
        assert_eq!(
            Value::String("hello".to_string()).itry_into(),
            Ok("hello".to_string())
        );
    }

    #[test]
    fn value_into_bool() {
        assert_eq!(Value::Boolean(true).itry_into(), Ok(true));
    }
}

//Enum for query
//Shows what type of query along with the data needed for it
//The data does not include the key becuase it needs Self

//Advanced search (multiple fields, full or partial)
pub enum QueryType<E: Entry> {
    //Doesn't require input, but it does need a key in the query call
    Lookup,
    //FieldName for Field being searched, Value for what's being searched
    //Field to sort by, Direction to sort in, page number
    Search(E::FieldNames, Value, u16, E::FieldNames, SortDirection, u16),
    //Limit, Field to sort by, Direction to sort in, page number
    //Field to sort by, Direction to sort in, page number
    GetAll(u16, E::FieldNames, SortDirection, u16),
    //FieldName for Field being searched, Value for what's being searched
    // Field to sort by, Direction to sort in, page number
    PartialSearch(E::FieldNames, Value, u16, E::FieldNames, SortDirection, u16),
    //FieldNames for Fields being searched, Values for what's being searched (in the same order)
    // Field to sort by, Direction to sort in, page number
    MultiSearch(
        Vec<E::FieldNames>,
        Vec<Value>,
        u16,
        E::FieldNames,
        SortDirection,
        u16,
    ),
}
//This enum is to determine direction in QueryType
pub enum SortDirection {
    Asc,  //Ascending
    Desc, //Descending
}

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
    fn search(
        &self,
        field_name: E::FieldNames,
        field_value: Value,
    ) -> Result<Vec<(Self::Key, E)>, String>;

    /// Update an entry at a given key with a new entry
    fn update(&self, key: Self::Key, entry: E) -> Result<(), String>;

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
    fn query(&self, q: QueryType<E>, key: Option<Self::Key>)
        -> Result<Vec<(Self::Key, E)>, String>;
}
