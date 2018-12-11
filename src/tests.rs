//!
//! This module has some test implementations of the traits provided in this crate.
//! This is used by tests in the crate. Otherwise, there would be nothing to use in tests.
//!

use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

use interface::Entry;
use interface::Value;
use interface::FieldName;
use interface::Table;

/**
 *  Test entry with no keys for anything
 */
#[derive(Debug, Clone)]
pub struct Department {
    pub name: String,
    pub abreviation: String,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DepartmentFields {
    Name,
    Abreviation
}

impl FieldName for DepartmentFields { }

impl Display for DepartmentFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DepartmentFields::Name => write!(f, "Name"),
            DepartmentFields::Abreviation => write!(f, "Abreviation"),
        }
    }
}

impl FromStr for DepartmentFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(DepartmentFields::Name),
            "Abreviation" => Ok(DepartmentFields::Abreviation),
            _ => Err("Field does not exist".to_string()),
        }
    }
}

impl<T: Table<Self>> Entry<T> for Department {

    type FieldNames = DepartmentFields;

    fn from_fields(values: &[Value<Self, T>]) -> Result<Self, String> {
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

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![DepartmentFields::Name, DepartmentFields::Abreviation]
    }

    fn get_fields(&self) -> Vec<Value<Self, T>> {
        vec![Value::String(self.name.clone()), Value::String(self.abreviation.clone())]
    }

    fn get_field(&self, field_name: DepartmentFields) -> Value<Self, T> {
        match field_name {
            DepartmentFields::Name => Value::String(self.name.clone()),
            DepartmentFields::Abreviation => Value::String(self.abreviation.clone()),
        }
    }
}

#[cfg(test)]
mod department_tests {

    use std::str::FromStr;

    use tests::Department;
    use tests::DepartmentFields;

    use vec_table::VecTable;

    use interface::Entry;
    use interface::Value;

    #[test]
    fn test_departmentfields_from_str() {
        let name_field = DepartmentFields::from_str("Name");
        assert_eq!(name_field, Ok(DepartmentFields::Name));

        let abreviation_field = DepartmentFields::from_str("Abreviation");
        assert_eq!(abreviation_field, Ok(DepartmentFields::Abreviation));
    }

    #[test]
    fn test_department_from_fields() {
        let fields: [Value<Department<VecTable>, VecTable<Department>>; 2] = [
            Value::String("Electrical and Computer Engineering".to_string()),
            Value::String("ECE".to_string())
        ];

        let department = Department::from_fields(&fields).unwrap();

        assert_eq!(department.name, "Electrical and Computer Engineering".to_string());
        assert_eq!(department.abreviation, "ECE".to_string());
    }

    #[test]
    fn test_department_get_field_names() {
        let field_names = Department::get_field_names();

        assert_eq!(field_names[0], DepartmentFields::Name);
        assert_eq!(field_names[1], DepartmentFields::Abreviation);
        assert_eq!(field_names.len(), 2);
    }

    #[test]
    fn test_department_get_fields() {
        let department = Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        };

        let fields = department.get_fields();

        assert_eq!(fields[0], Value::String("Electrical and Computer Engineering".to_string()));
        assert_eq!(fields[1], Value::String("ECE".to_string()));
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_department_get_field() {
        let department = Department {
            name: "Electrical and Computer Engineering".to_string(),
            abreviation: "ECE".to_string(),
        };

        let first_name = department.get_field(DepartmentFields::Name);

        assert_eq!(first_name, Value::String("Electrical and Computer Engineering".to_string()));
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum UserFields {
    FirstName,
    LastName,
}

impl FieldName for UserFields {}

impl Display for UserFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserFields::FirstName => write!(f, "First Name"),
            UserFields::LastName => write!(f, "Last Name"),
        }
    }
}

impl FromStr for UserFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "First Name" => Ok(UserFields::FirstName),
            "Last Name" => Ok(UserFields::LastName),
            _ => Err("Field does not exist".to_string()),
        }
    }
}

impl<T: Table<Self>> Entry<T> for User {

    type FieldNames = UserFields;

    fn from_fields(values: &[Value<Self, T>]) -> Result<Self, String> {
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

    fn get_field_names() -> Vec<Self::FieldNames> {
        vec![UserFields::FirstName, UserFields::LastName]
    }

    fn get_fields(&self) -> Vec<Value<Self, T>> {
        vec![Value::String(self.first_name.clone()), Value::String(self.last_name.clone())]
    }

    fn get_field(&self, field_name: UserFields) -> Value<Self, T> {
        match field_name {
            UserFields::FirstName => Value::String(self.first_name.clone()),
            UserFields::LastName => Value::String(self.last_name.clone()),
        }
    }
}

#[cfg(test)]
mod user_tests {

    use std::str::FromStr;

    use tests::User;
    use tests::UserFields;

    use interface::Entry;
    use interface::Value;

    #[test]
    fn test_userfields_from_str() {
        let firstname_field = UserFields::from_str("First Name");
        assert_eq!(firstname_field, Ok(UserFields::FirstName));

        let lastname_field = UserFields::from_str("Last Name");
        assert_eq!(lastname_field, Ok(UserFields::LastName));
    }

    #[test]
    fn test_user_from_fields() {
        let fields = [
            Value::String("Tim".to_string()),
            Value::String("Hollabaugh".to_string())
        ];

        let user = User::from_fields(&fields).unwrap();

        assert_eq!(user.first_name, "Tim".to_string());
        assert_eq!(user.last_name, "Hollabaugh".to_string());
    }

    #[test]
    fn test_user_get_field_names() {
        let field_names = User::get_field_names();

        assert_eq!(field_names[0], UserFields::FirstName);
        assert_eq!(field_names[1], UserFields::LastName);
        assert_eq!(field_names.len(), 2);
    }

    #[test]
    fn test_user_get_fields() {
        let user = User {
            first_name: "Tim".to_string(),
            last_name: "Hollabaugh".to_string(),
        };

        let fields = user.get_fields();

        assert_eq!(fields[0], Value::String("Tim".to_string()));
        assert_eq!(fields[1], Value::String("Hollabaugh".to_string()));
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_user_get_field() {
        let user = User {
            first_name: "Tim".to_string(),
            last_name: "Hollabaugh".to_string(),
        };

        let first_name = user.get_field(UserFields::FirstName);

        assert_eq!(first_name, Value::String("Tim".to_string()));
    }
}
