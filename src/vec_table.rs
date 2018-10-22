use interface::Entry;
use interface::Key;
use interface::Table;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VecTableKey {
    id: usize,
}

impl<E: Entry + 'static> Key<E> for VecTableKey {
    fn same_as(&self, other: Box<dyn Key<E>>) -> bool {
        if let Some(other_key) = other.downcast_ref::<VecTableKey>() {
            self.id == other_key.id
        } else {
            false
        }
    }
}

pub struct VecTable<E: Entry> {
    vector: Vec<E>,
}

impl<E: Entry> VecTable<E> {
    pub fn new() -> VecTable<E> {
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

    fn lookup(&self, key: Box<dyn Key<E>>) -> Option<&E> {
        if let Some(key) = key.downcast_ref::<VecTableKey>() {
            self.vector.get(key.id)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use tests::Department;
    use interface::Key;
    use interface::Table;
    use vec_table::VecTableKey;
    use vec_table::VecTable;

    #[test]
    fn test_vectable_key_same_as() {
        let key_1: Box<dyn Key<Department>> = Box::new(VecTableKey { id: 1 });
        let key_2: Box<dyn Key<Department>> = Box::new(VecTableKey { id: 1 });

        assert!(key_1.same_as(key_2));
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

