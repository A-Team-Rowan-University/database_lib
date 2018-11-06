//!
//! This module has some test implementations of the traits provided in this crate.
//! This is used by tests in the crate. Otherwise, there would be nothing to use in tests.
//!

use interface::Entry;
use interface::Key;
extern crate mysql as my;

/**
 *  Test entry with no keys for anything
 */
#[derive(Debug,Clone)]
pub struct Department {
    pub name: String,
    pub abreviation: String,
}

impl Entry for Department {
	fn to_vec_string(&self)->Vec<String>{unimplemented!()}
	fn from_mysql(_data: &Vec<my::Value>)->Department{unimplemented!()}
}

/**
 *  Test entry with a key
 */
#[derive(Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub department: Box<dyn Key<Department>>,
}

impl Entry for User {
	fn to_vec_string(&self)->Vec<String>{unimplemented!()}
	fn from_mysql(_data: &Vec<my::Value>)->User{unimplemented!()}
}


