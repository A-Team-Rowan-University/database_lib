//!
//! This module has some test implementations of the traits provided in this crate.
//! This is used by tests in the crate. Otherwise, there would be nothing to use in tests.
//!

use interface::Entry;
use interface::Key;


/**
 *  Test entry with no keys for anything
 */
#[derive(Debug)]
pub struct Department {
    pub name: String,
    pub abreviation: String,
}

impl Entry for Department {

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

}


