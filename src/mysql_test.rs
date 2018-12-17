#[cfg(test)]
mod mysql_test {
    extern crate mysql as my;
    extern crate rpassword;
    use interface;
    use interface::Entry;
    use interface::ITryInto;
    use interface::Table;
    use my_types;
    use std::fmt;
    use std::fmt::Display;
    use std::io;
    use std::marker::PhantomData;
    use std::str::FromStr;

    //The following is an example of how to use the my_types to both send and recieve data from mySQL.
    //Because the tables rely on follwing the schema very closely, here is the schema for this example
    /*
    +--------------------+
    | Database           |
    +--------------------+
    | dbTest             |
    +--------------------+
    +------------------+
    | Tables_in_People |
    +------------------+
    | User             |
    +------------------+

    Columns in User
    +-----------+-------------+------+-----+---------+----------------+
    | Field     | Type        | Null | Key | Default | Extra          |
    +-----------+-------------+------+-----+---------+----------------+
    | userID    | int(11)     | NO   | PRI | NULL    | auto_increment |
    | firstname | varchar(32) | NO   |     | NULL    |                |
    | lastname  | varchar(32) | NO   |     | NULL    |                |
    | email     | varchar(64) | NO   |     | NULL    |                |
    | bannerID  | bigint(20)  | NO   |     | NULL    |                |
    +-----------+-------------+------+-----+---------+----------------+

    */

    //Struct to hold the row data
    #[derive(Clone, PartialEq, Eq)]
    struct User {
        firstname: String,
        lastname: String,
        email: String,
        bannerID: i32,
    }

    #[derive(PartialEq, Clone, Copy, Debug)]
    enum UserFields {
        firstname,
        lastname,
        email,
        bannerID,
    }
    impl interface::FieldName for UserFields {}

    impl FromStr for UserFields {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "firstname" => Ok(UserFields::firstname),
                "lastname" => Ok(UserFields::lastname),
                "email" => Ok(UserFields::email),
                "bannerID" => Ok(UserFields::bannerID),
                _ => Err("Field does not exist".to_string()),
            }
        }
    }
    impl Display for UserFields {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                UserFields::firstname => write!(f, "firstname"),
                UserFields::lastname => write!(f, "lastname"),
                UserFields::email => write!(f, "email"),
                UserFields::bannerID => write!(f, "bannerID"),
            }
        }
    }
    //Defines the entry functions for student
    impl Entry for User {
        type FieldNames = UserFields;

        fn from_fields(values: &[interface::Value]) -> Result<Self, String> {
            let length = values.len();
            let len_err = "Wrong vector for creating User, expecting length of 4, found "
                .to_string()
                + &length.to_string();
            let this_result: Result<User, String> = match &length {
                4 => {
                    //Must assign non strings to buffers first to transfer the data from interface::Value to actual data
                    let banner: Result<i32, String> = values[3].to_owned().itry_into();
                    match banner {
                        Ok(_i32) => Ok(User {
                            firstname: values[0].to_string().clone(),
                            lastname: values[1].to_string().clone(),
                            email: values[2].to_string().clone(),
                            bannerID: banner.unwrap(),
                        }),
                        _ => Err("Not proper values for user".to_string()),
                    }
                }
                _ => Err(len_err),
            };
            this_result
        }
        fn get_field_names() -> Vec<Self::FieldNames> {
            vec![
                UserFields::firstname,
                UserFields::lastname,
                UserFields::email,
                UserFields::bannerID,
            ]
        }
        fn get_fields(&self) -> Vec<interface::Value> {
            vec![
                interface::Value::String(self.firstname.clone()),
                interface::Value::String(self.lastname.clone()),
                interface::Value::String(self.email.clone()),
                interface::Value::Integer(self.bannerID.clone()),
            ]
        }
        fn get_field(&self, field_name: Self::FieldNames) -> Option<interface::Value> {
            match field_name {
                UserFields::firstname => Some(interface::Value::String(self.firstname.clone())),
                UserFields::lastname => Some(interface::Value::String(self.lastname.clone())),
                UserFields::email => Some(interface::Value::String(self.email.clone())),
                UserFields::bannerID => Some(interface::Value::Integer(self.bannerID.clone())),
                _ => None,
            }
        }
    }

    #[test]
    #[ignore]
    fn full_mysql_test() {
        println!("enter username: ");
        let mut user = String::new();
        io::stdin()
            .read_line(&mut user)
            .expect("Failed to read line");
        user = user.trim().to_string();
        println!("{}'s password: ", user);
        let pass = rpassword::read_password().unwrap().trim().to_string();
        let pool = my_types::open_mysql(user, pass).unwrap(); //Open mySQL

        let mut user_table: my_types::MysqlTable<User> = my_types::MysqlTable {
            tb_name: "User".to_string(),
            db_name: "dbTest".to_string(),
            key_name: "userID".to_string(),
            pool: pool,
            phantom: PhantomData,
        };

        //Create a student to send to the database
        let nick_kz = User {
            firstname: "Nick".to_string(),
            lastname: "Kluzynski".to_string(),
            email: "kluzynskn6@students.rowan.edu".to_string(),
            bannerID: 916181533,
        };
        //Create a student to update with
        let nick_update = User {
            firstname: "Nicholas".to_string(),
            lastname: "Kluzynski".to_string(),
            email: "kluzynskn6@students.rowan.edu".to_string(),
            bannerID: 916181533,
        };
        //Testing basic functions
        let nick_key: my_types::MysqlTableKey = Some(user_table.insert(nick_kz)).unwrap();
        assert!(nick_key.valid);

        let nick_bool = user_table.contains(nick_key);
        assert!(nick_bool);

        let nick_2 = user_table.lookup(nick_key).unwrap();
        assert_eq!(nick_2.firstname, "Nick");

        let _nick_up_check = user_table.update(nick_key, nick_update).unwrap();

        // 												Create a generic value containing the string 'Nick'
        let nick_3 = user_table
            .search(
                UserFields::firstname,
                interface::Value::String("\'Nicholas\'".to_string()),
            )
            .unwrap()[0]
            .to_owned()
            .1; //Only saves the entry of the first result
        assert_eq!(nick_3.lastname, "Kluzynski");

        //Testing query
        let q_nick = user_table
            .query(interface::QueryType::Lookup, Some(nick_key))
            .unwrap();
        assert_eq!(q_nick[0].1.firstname, "Nicholas");

        let q_parsearch = user_table
            .query(
                interface::QueryType::PartialSearch(
                    UserFields::email,
                    interface::Value::String("@".to_string()),
                    2,
                    UserFields::firstname,
                    interface::SortDirection::Asc,
                    1,
                ),
                None,
            )
            .unwrap();
        assert_eq!(q_parsearch.len(), 2);

        let q_all = user_table
            .query(
                interface::QueryType::GetAll(
                    3,
                    UserFields::firstname,
                    interface::SortDirection::Asc,
                    1,
                ),
                None,
            )
            .unwrap();
        assert_eq!(q_all.len(), 3);

        let field_vec = vec![UserFields::firstname, UserFields::lastname];
        let val_vec = vec![
            interface::Value::String("Nick".to_string()),
            interface::Value::String("Kluzynski".to_string()),
        ];
        let q_multi = user_table
            .query(
                interface::QueryType::MultiSearch(
                    field_vec,
                    val_vec,
                    2,
                    UserFields::bannerID,
                    interface::SortDirection::Asc,
                    1,
                ),
                None,
            )
            .unwrap();
        assert_eq!(q_multi[0].1.firstname, "Nick");

        let _nick_del = user_table.remove(nick_key).unwrap(); //Delete Nick from db so it doesn't get clogged
    }

}
