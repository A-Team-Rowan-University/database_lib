use std::fmt::Debug;
use downcast_rs::Downcast;
extern crate mysql as my;

/**
 *  Key for an item in a table.
 *  Keys get implemented for each table type. This allows each table to define their own definition
 *  of a key, including what might be used in the actual table. For example, the VecTable defines
 *  a VecTableKey that has an usize for an index into a vector. The E type parameter is used to
 *  ensure that a key cannot be used for any entry other than the one it was created for.
 *
 *  The Downcast trait allows us to turn a Box<dyn Key> into the concrete type that it came from
 */
pub trait Key<E: Entry + 'static>: Debug + Downcast {
    fn same_as(&self, other: Box<dyn Key<E>>) -> bool;
}

impl_downcast!(Key<E> where E: Entry);

/**
 * Entry in a table. Things that implement this are stored in the database
 */
pub trait Entry {
    //fn get_fields(&self) -> ;
	fn from_mysql(data:&Vec<my::Value>) -> Self;
	fn to_vec_string(&self) -> Vec<String>;
}

pub trait Table<E: Entry> {
    type Key;
    fn insert(&mut self, entry: E) -> Box<dyn Key<E>>;
    fn lookup(&self, key: Box<dyn Key<E>>) -> Option<E>;
}

