
#[macro_use]
extern crate downcast_rs;

use std::fmt::Debug;

use downcast_rs::Downcast;

trait Key<E: Entry + 'static>: Debug + Downcast {
    fn same_as(&self, other: &dyn Key<E>) -> bool;
}

impl_downcast!(Key<E> where E: Entry);

trait Entry {
    //fn get_fields(&self) -> ;
}

trait Table<E: Entry> {
    type Key;
    fn insert(&mut self, entry: E) -> Box<dyn Key<E>>;
    fn lookup(&self, key: &dyn Key<E>) -> Option<&E>;
}

#[derive(Debug)]
struct Department {
    name: String,
    abreviation: String,
}

impl Entry for Department {

}

#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    department: Box<dyn Key<Department>>,
}

impl Entry for User {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct VecTableKey {
    id: usize,
}

impl<E: Entry + 'static> Key<E> for VecTableKey {
    fn same_as(&self, other: &dyn Key<E>) -> bool {
        if let Some(other_key) = other.downcast_ref::<VecTableKey>() {
            self.id == other_key.id
        } else {
            false
        }
    }
}

struct VecTable<E: Entry> {
    vector: Vec<E>,
}

impl<E: Entry> VecTable<E> {
    fn new() -> VecTable<E> {
        VecTable {
            vector: Vec::new()
        }
    }
}

impl<E: Entry + 'static> Table<E> for VecTable<E> {
    type Key = ();
    fn insert(&mut self, entry: E) -> Box<dyn Key<E>> {
        self.vector.push(entry);
        Box::new(VecTableKey {
            id: self.vector.len()-1
        })
    }

    fn lookup(&self, key: &dyn Key<E>) -> Option<&E> {
        if let Some(key) = key.downcast_ref::<VecTableKey>() {
            self.vector.get(key.id)
        } else {
            None
        }
    }
}

#[test]
fn test_vectable() {

    let mut department_table: VecTable<Department> = VecTable::new();

    let ece_department = Department {
        name: "Electrical and Computer Engineering".to_string(),
        abreviation: "ECE".to_string(),
    };

    let ece_key = department_table.insert(ece_department);

    let ece_department_after = department_table.lookup(&*ece_key).unwrap();

    assert_eq!(ece_department_after.name, "Electrical and Computer Engineering".to_string());
    assert_eq!(ece_department_after.abreviation, "ECE".to_string());
}

