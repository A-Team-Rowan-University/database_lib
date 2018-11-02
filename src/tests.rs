//!
//! This module has some test implementations of the traits provided in this crate.
//! This is used by tests in the crate. Otherwise, there would be nothing to use in tests.
//!

use interface::Entry;
use interface::Value;


/**
 *  Test entry with no keys for anything
 */
#[derive(Debug, Clone)]
pub struct Department {
    pub name: String,
    pub abreviation: String,
}

impl Entry for Department {

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref name) = values[0] {
            if let Value::String(ref abreviation) = values[1] {
                Ok(Department {
                    name: name.clone(),
                    abreviation: abreviation.clone(),
                })
            } else {
                Err("Incorrect type for abreviation. Should be String".to_string())
            }
        } else {
            Err("Incorrect type for name. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<String> {
        vec!["Name".to_string(), "Abreviation".to_string()]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.name.clone()), Value::String(self.abreviation.clone())]
    }
}

/**
 *  Test entry with a key
 */
#[derive(Debug, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
}

impl Entry for User {

    fn from_fields(values: &[Value]) -> Result<Self, String> {
        if let Value::String(ref first_name) = values[0] {
            if let Value::String(ref last_name) = values[1] {
                Ok(User {
                    first_name: first_name.clone(),
                    last_name: last_name.clone(),
                })
            } else {
                Err("Incorrect type for abreviation. Should be String".to_string())
            }
        } else {
            Err("Incorrect type for name. Should be String".to_string())
        }
    }

    fn get_field_names() -> Vec<String> {
        vec!["First Name".to_string(), "Last Name".to_string()]
    }

    fn get_fields(&self) -> Vec<Value> {
        vec![Value::String(self.first_name.clone()), Value::String(self.last_name.clone())]
    }
}


