
use std::marker::PhantomData;

use interface::Entry;
use interface::Key;
use interface::Table;

/**
 *  A key for a VecTable
 *
 *  It stores an index into the vector for the entry this key goes with
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VecTableKey {
    // The index into the vector for this entry
    id: usize,
}

impl<E: Entry> Key<E> for VecTableKey { }

/**
 *  A table implemented as a vector. Inserting will add to the end of the vector, and keys are the
 *  index. Removing will probably do nothing, so the vec will keep expanding but never shrink.
 *  Intended for testing and example purposes only.
*/
pub struct VecTable<E: Entry> {
    vector: Vec<E>,
}

impl<E: Entry> VecTable<E> {

    /**
     *  Gives a new `VecTable` with an empty vector
     */
    pub fn new() -> VecTable<E> {
        VecTable {
            vector: Vec::new()
        }
    }
}

impl<E: Entry> Table<E> for VecTable<E> {

    type Key = VecTableKey;

    fn insert(&mut self, entry: E) -> Self::Key {
        self.vector.push(entry);
        VecTableKey {
            id: self.vector.len()-1
        }
    }

    fn lookup(&self, key: Self::Key) -> Option<E> {
        self.vector.get(key.id).map(|e| e.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestTableKey {
    // The index into the vector for this entry
    id: usize,
}

impl<E: Entry> Key<E> for TestTableKey { }

pub struct TestTable<E: Entry> {
    // Needed since we do not actually use the E type parameter
    entry_type: PhantomData<E>
}

impl<E: Entry> Table<E> for TestTable<E> {

    type Key = TestTableKey;

    fn insert(&mut self, entry: E) -> Self::Key {
        unimplemented!()
    }

    fn lookup(&self, key: Self::Key) -> Option<E> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use tests::Department;
    use interface::Table;
    use vec_table::VecTableKey;
    use vec_table::VecTable;

    #[test]
    fn test_vectable_key_partial_eq() {
        let key_1 = VecTableKey { id: 1 };
        let key_2 = VecTableKey { id: 1 };

        assert!(key_1 == key_2);
    }

    #[test]
    fn test_vectable() {

        let mut department_table: VecTable<Department> = VecTable::new();

        let ece_department = Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        };

        let ece_key = department_table.insert(ece_department);

        let ece_department_after = department_table.lookup(ece_key).unwrap();

        assert_eq!(ece_department_after.name, "Electrical and Computer Engineering".to_string());
        assert_eq!(ece_department_after.abreviation, "ECE".to_string());
    }
}
