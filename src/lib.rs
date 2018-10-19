
use std::collections::hash_map::HashMap;
use std::hash::Hash;

trait Key: Copy + Eq + Hash {
    fn new() -> Self;
}

trait Entry {
    type PrimaryKey: Key;
    //fn get_fields(&self) -> ;
}

trait Table<E: Entry> {
    type Key;
    fn insert(&mut self, entry: E) -> E::PrimaryKey;
    fn lookup(&self, key: E::PrimaryKey) -> Option<&E>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UserKey;
impl Key for UserKey {
    fn new() -> UserKey {
        UserKey { }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DepartmentKey;
impl Key for DepartmentKey {
    fn new() -> DepartmentKey {
        DepartmentKey { }
    }
}

#[derive(Debug, PartialEq)]
struct User {
    first_name: String,
    last_name: String,
    department: DepartmentKey,
}

impl Entry for User {
    type PrimaryKey = UserKey;
}

struct HashTable<E: Entry> {
    map: HashMap<E::PrimaryKey, E>
}

impl<E: Entry> HashTable<E> {
    fn new() -> HashTable<E> {
        HashTable {
            map: HashMap::new()
        }
    }
}

impl<E: Entry> Table<E> for HashTable<E> {
    type Key = ();
    fn insert(&mut self, entry: E) -> E::PrimaryKey {
        let key = E::PrimaryKey::new();
        self.map.insert(key, entry);
        key
    }

    fn lookup(&self, key: E::PrimaryKey) -> Option<&E> {
        self.map.get(&key)
    }
}

#[test]
fn test_hashtable() {

    let mut table: HashTable<User> = HashTable::new();

    let key = table.insert(
        User {
            first_name: "Tim".to_owned(),
            last_name: "Hollabaugh".to_owned(),
            department: DepartmentKey { },
        }
    );

    let user = table.lookup(key).unwrap();

    assert_eq!(
        *user,
        User {
            first_name: "Tim".to_owned(),
            last_name: "Hollabaugh".to_owned(),
            department: DepartmentKey { },
        }
    );
}

