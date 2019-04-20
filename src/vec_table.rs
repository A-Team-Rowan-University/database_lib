use std::cmp::Ordering;

use interface::Entry;
use interface::Key;
use interface::QueryType;
use interface::Table;
use interface::Value;
use interface::SortDirection;

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

    fn update(&mut self, key: Self::Key, entry: E) -> Result<(), String> {
        for (k, e) in self.vector.iter_mut() {
            if key.id == *k {
                *e = entry;
                return Ok(());
            }
        }

        Err("Key not found".to_string())
    }

    fn query(
        &self,
        q: QueryType<E>,
        key: Option<Self::Key>,
    ) -> Result<Vec<(Self::Key, E)>, String> {
        match q {
            QueryType::Lookup => {
                if let Some(key) = key {
                    match self.lookup(key) {
                        Some(entry) => Ok(vec![(key, entry)]),
                        None => Err("Key not found".to_string()),
                    }
                } else {
                    Err("Need a key!".to_string())
                }
            }

            QueryType::GetAll(limit, sort_field, sort_dir, page) => {
                // sort, skip lim*(pg-1), take lim

                let mut i = 0;
                let field_index = loop {
                    match E::get_field_names().get(i) {
                        Some(field_name) => if *field_name == sort_field { break i } else { i += 1 },
                        None => return Err("Bad field".to_string()),
                    }
                };

                let mut slice = self.vector.clone();
                slice.sort_by(|(_, a), (_, b)| {
                    match sort_dir {
                        SortDirection::Asc => {
                            a.get_fields()[field_index]
                                .partial_cmp(&b.get_fields()[field_index])
                                .unwrap_or(Ordering::Equal)
                        }

                        SortDirection::Desc => {
                            a.get_fields()[field_index]
                                .partial_cmp(&b.get_fields()[field_index])
                                .unwrap_or(Ordering::Equal)
                                .reverse()
                        }
                    }
                });

                Ok(slice
                   .into_iter()
                   .skip((limit * (page-1)) as usize)
                   .take(limit as usize)
                   .map(|(id, entry)| (VecTableKey { id }, entry))
                   .collect())
            }

            _ => unimplemented!(),
        }
    }

    fn search(
        &self,
        field_name: E::FieldNames,
        field_value: Value,
    ) -> Result<Vec<(Self::Key, E)>, String> {
        let mut good_value: bool = true;
        let temp = self.vector.iter().fold(Vec::new(), |mut v, (id, e)| {
            if let Some(value) = e.get_field(field_name) {
                if value == field_value {
                    v.push((VecTableKey { id: *id }, e.clone()));
                }
            } else {
                good_value = false;
            }
            v
        });
        if good_value == true {
            Ok(temp)
        } else {
            Err("Error converting vec in vectable".to_string())
        }
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

    use interface::QueryType;
    use interface::SortDirection;
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
            )
            .unwrap()
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

    #[test]
    fn test_vectable_update() {
        let mut department_table: VecTable<Department> = VecTable::new();

        let ece_key = department_table.insert(Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        });

        department_table
            .update(
                ece_key,
                Department {
                    name: "Electrical and Computer Engineering Department".to_string(),
                    abreviation: "ECE".to_string(),
                },
            )
            .unwrap();

        let new_ece = department_table.lookup(ece_key).unwrap();

        assert_eq!(
            new_ece.name,
            "Electrical and Computer Engineering Department".to_string()
        );
        assert_eq!(new_ece.abreviation, "ECE".to_string());
    }

    #[test]
    fn test_vectable_query_lookup() {
        let mut department_table: VecTable<Department> = VecTable::new();

        let ece_key = department_table.insert(Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        });

        let mut found: Vec<(VecTableKey, Department)> = department_table
            .query(QueryType::Lookup, Some(ece_key))
            .unwrap();

        assert_eq!(found.len(), 1);

        let (key, entry) = found.pop().unwrap();

        assert_eq!(key, ece_key);
        assert_eq!(
            entry.name,
            "Electrical and Computer Engineering".to_string()
        );
        assert_eq!(entry.abreviation, "ECE".to_string());
    }

    #[test]
    fn test_vectable_query_getall_asc() {
        let mut department_table: VecTable<Department> = VecTable::new();

        let ece_deparment = Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        };

        let me_department = Department {
            name: "Mechanical Engineering".to_string(),
            abreviation: "ME".to_string(),
        };

        let bme_department = Department {
            name: "Biomedical Engineering".to_string(),
            abreviation: "BME".to_string(),
        };

        let ece_key = department_table.insert(ece_deparment);
        let me_key = department_table.insert(me_department);
        let bme_key = department_table.insert(bme_department);

        let mut found: Vec<(VecTableKey, Department)> = department_table
            .query(
                QueryType::GetAll(2, DepartmentFields::Abreviation, SortDirection::Asc, 1),
                None,
            )
            .unwrap();

        assert_eq!(found.len(), 2);

        let (found_bme_key, found_bme_entry) = found.remove(0);
        let (found_ece_key, found_ece_entry) = found.remove(0);

        assert_eq!(found_ece_key, ece_key);
        assert_eq!(found_bme_key, bme_key);

        assert_eq!(found_ece_entry.name, "Electrical and Computer Engineering".to_string());
        assert_eq!(found_ece_entry.abreviation, "ECE".to_string());

        assert_eq!(found_bme_entry.name, "Biomedical Engineering".to_string());
        assert_eq!(found_bme_entry.abreviation, "BME".to_string());

        let mut found_page_2 = department_table
            .query(
                QueryType::GetAll(2, DepartmentFields::Abreviation, SortDirection::Asc, 2),
                None,
            )
            .unwrap();

        assert_eq!(found_page_2.len(), 1);

        let (found_me_key, found_me_entry) = found_page_2.remove(0);

        assert_eq!(found_me_key, me_key);
        assert_eq!(found_me_entry.name, "Mechanical Engineering".to_string());
        assert_eq!(found_me_entry.abreviation, "ME".to_string());
    }

    #[test]
    fn test_vectable_query_getall_desc() {
        let mut department_table: VecTable<Department> = VecTable::new();

        let ece_deparment = Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        };

        let me_department = Department {
            name: "Mechanical Engineering".to_string(),
            abreviation: "ME".to_string(),
        };

        let bme_department = Department {
            name: "Biomedical Engineering".to_string(),
            abreviation: "BME".to_string(),
        };

        let ece_key = department_table.insert(ece_deparment);
        let me_key = department_table.insert(me_department);
        let bme_key = department_table.insert(bme_department);

        let mut found: Vec<(VecTableKey, Department)> = department_table
            .query(
                QueryType::GetAll(2, DepartmentFields::Abreviation, SortDirection::Desc, 1),
                None,
            )
            .unwrap();

        assert_eq!(found.len(), 2);

        let (found_me_key, found_me_entry) = found.remove(0);
        let (found_ece_key, found_ece_entry) = found.remove(0);

        assert_eq!(found_ece_key, ece_key);
        assert_eq!(found_ece_entry.name, "Electrical and Computer Engineering".to_string());
        assert_eq!(found_ece_entry.abreviation, "ECE".to_string());

        assert_eq!(found_me_key, me_key);
        assert_eq!(found_me_entry.name, "Mechanical Engineering".to_string());
        assert_eq!(found_me_entry.abreviation, "ME".to_string());

        let mut found_page_2 = department_table
            .query(
                QueryType::GetAll(2, DepartmentFields::Abreviation, SortDirection::Desc, 2),
                None,
            )
            .unwrap();

        assert_eq!(found_page_2.len(), 1);

        let (found_bme_key, found_bme_entry) = found_page_2.remove(0);

        assert_eq!(found_bme_key, bme_key);

        assert_eq!(found_bme_entry.name, "Biomedical Engineering".to_string());
        assert_eq!(found_bme_entry.abreviation, "BME".to_string());

    }
}
