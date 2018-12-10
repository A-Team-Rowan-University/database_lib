use interface::Entry;
use interface::Key;
use interface::Table;
use interface::Value;

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

impl<E: Entry> Key<E> for VecTableKey {}

/**
 *  A table implemented as a vector. Inserting will add to the end of the vector, and keys are the
 *  index. Removing will probably do nothing, so the vec will keep expanding but never shrink.
 *  Intended for testing and example purposes only.
*/
#[derive(Default)]
pub struct VecTable<E: Entry> {
    vector: Vec<(usize, E)>,
    next_key: usize,
}

impl<E: Entry> VecTable<E> {
    /**
     *  Gives a new `VecTable` with an empty vector
     */
    pub fn new() -> VecTable<E> {
        VecTable {
            vector: Vec::new(),
            next_key: 0,
        }
    }
}

impl<E: Entry> Table<E> for VecTable<E> {
    type Key = VecTableKey;

    fn insert(&mut self, entry: E) -> Self::Key {
        self.vector.push((self.next_key, entry));
        let key = VecTableKey { id: self.next_key };
        self.next_key += 1;
        key
    }

    fn lookup(&self, key: Self::Key) -> Option<E> {
        for (k, e) in &self.vector {
            if key.id == *k {
                return Some(e.clone());
            }
        }

        None
    }

    fn update(&self, _key: Self::Key, _entry: E) -> Result<(), String> {
        unimplemented!();
    }

    fn search(
        &self,
        field_name: E::FieldNames,
        field_value: Value,
    ) -> Result<Vec<(Self::Key, E)>, String> {
        let temp = self.vector.iter().fold(Vec::new(), |mut v, (id, e)| {
            if e.get_field(field_name) == field_value {
                v.push((VecTableKey { id: *id }, e.clone()));
            }
            v
        });
        Ok(temp)
    }

    fn remove(&mut self, key: Self::Key) -> Result<(), String> {
        let index = self.vector.iter().fold(
            None,
            |i, (id, _e)| {
                if *id == key.id {
                    Some(*id)
                } else {
                    i
                }
            },
        );

        if let Some(index) = index {
            self.vector.remove(index);
            Ok(())
        } else {
            Err("Key not in table".to_string())
        }
    }

    fn contains(&self, key: Self::Key) -> bool {
        self.vector.iter().any(|(k, _e)| *k == key.id)
    }
}

#[cfg(test)]
mod tests {

    use interface::Table;
    use interface::Value;
    use tests::Department;
    use tests::DepartmentFields;
    use vec_table::VecTable;
    use vec_table::VecTableKey;

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

        assert_eq!(
            ece_department_after.name,
            "Electrical and Computer Engineering".to_string()
        );
        assert_eq!(ece_department_after.abreviation, "ECE".to_string());
    }

    #[test]
    fn test_search() {
        let mut department_table: VecTable<Department> = VecTable::new();

        let ece_key = department_table.insert(Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        });

        let _me_key = department_table.insert(Department {
            name: "Mechanical Engineering".to_string(),
            abreviation: "ME".to_string(),
        });

        let (found_key, _found_entry) = department_table
            .search(
                DepartmentFields::Abreviation,
                Value::String("ECE".to_string()),
            ).unwrap()
            .pop()
            .unwrap();

        assert_eq!(ece_key, found_key);
    }

    #[test]
    fn test_vectable_contains() {
        let mut department_table: VecTable<Department> = VecTable::new();

        let ece_key = department_table.insert(Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        });

        let me_key = department_table.insert(Department {
            name: "Mechanical Engineering".to_string(),
            abreviation: "ME".to_string(),
        });

        assert!(department_table.contains(ece_key));
        assert!(department_table.contains(me_key));
    }

    #[test]
    fn test_vectable_remove() {
        let mut department_table: VecTable<Department> = VecTable::new();

        let ece_key = department_table.insert(Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        });

        let me_key = department_table.insert(Department {
            name: "Mechanical Engineering".to_string(),
            abreviation: "ME".to_string(),
        });

        let result = department_table.remove(ece_key);

        assert_eq!(result, Ok(()));
        assert!(!department_table.contains(ece_key));
        assert!(department_table.contains(me_key));
    }
}
