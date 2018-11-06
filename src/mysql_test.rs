extern crate mysql as my;
extern crate rouille;
extern crate database_lib as dbl;
extern crate mysql_interface as myi;
extern crate rpassword;
use std::io;
use dbl::interface::Entry;
use dbl::interface::Key;
use dbl::interface::Table;
use myi::my_types;

//The following is an example of how to use the my_types to both send and recieve data from mySQL.
//Because the tables rely on follwing the schema very closely, here is the schema for this example
/*
+--------------------+
| Database           |
+--------------------+
| People             |
+--------------------+
+------------------+
| Tables_in_People |
+------------------+
| Students         |
+------------------+

Columns in Students
+-------+-------------+------+-----+---------+----------------+
| Field | Type        | Null | Key | Default | Extra          |
+-------+-------------+------+-----+---------+----------------+
| id    | int(11)     | NO   | PRI | NULL    | auto_increment |
| name  | varchar(32) | NO   |     | NULL    |                |
| year  | int(11)     | YES  |     | NULL    |                |
+-------+-------------+------+-----+---------+----------------+

*/

//These 2 are for the test functions at the bottom
struct db {
    name:String,
}
struct tb {
    name:String,
}
//Struct to hold the row data
#[derive(Clone, PartialEq, Eq)]
struct student {
    name:String,
    Year:u32,
}
//Defines the entry functions for student
impl Entry for student{
	//Pushes the data into a Vec<String>
	//Must be done in the same order as the mySQL database or else it will panic
	fn to_vec_string(&self) -> Vec<String> {
		let mut data = Vec::new();
		//Strings need single quotes (') around them, so be sure to concatinate them in.
		data.push("\'".to_owned()+&self.name.to_owned()+&"\'".to_owned());
		data.push(self.Year.to_string());
		data//Once the data is all in the Vec, send data
	}
	fn from_mysql(data:&Vec<my::Value>) -> Self{
		//Create a new student based on the generic mySQL values that are sent back.
		//Also must be done in the same as the mySQL database
		//Skip entry 0 because that is the key
		let Student = student {
			name:my::from_value(data[1].to_owned()),
			Year:my::from_value(data[2].to_owned()),
		
		};
		Student
	}

}


fn main (){
	let pool = open_mysql("kluzynick".to_string());//Open mySQL, can be polled to find user instead of typing one into the funtion call
	
	let fieldvec = vec!["name".to_string(),"Year".to_string()];//Create a Vec<String> for the fields
	let mut student_table = my_types::mysql_table {
		tb_name: "Students".to_string(),
		db_name: "People".to_string(),
		key_name: "id".to_string(),
		pool:pool, 
		field: fieldvec,
	};
	//Create a student to send to the database
	let Nick = student{
		name:"Nick".to_string(),
		Year:2019,
	};
	//Insert the student into the database and unwrap the key that it sends back
	let Nick_key = Some(student_table.insert(Nick)).unwrap();
	
	//Fill Nick_2 with the data from the database
	let Nick_2 = student_table.lookup(Nick_key).unwrap();
	assert_eq!(&Nick_2.name,"Nick");
	
	//Should delete when that has been implemented
	
}
//Opens a pooled connection to mySQL and returns the pool used to acess it
fn open_mysql(user: String)-> my::Pool{
	let mut  optbuild = my::OptsBuilder::new();

	println!("{}'s password: ",user);
    let pass = rpassword::read_password().unwrap();
	
	optbuild.ip_or_hostname(Some("localhost"))
		.user(Some(user))
		.pass(Some(pass));

	let optcon = optbuild;
	let p = my::Pool::new(optcon).unwrap();
	p
}
//Test function
fn list_databases(p: &my::Pool)-> Vec<db> {
	let mut con = p.get_conn().unwrap();

	let databases: Vec<db> = con.prep_exec("SHOW DATABASES",())
		.map(|result|{
			result.map(|x| x.unwrap()).map(|row|{
			let name = my::from_row(row);
			db{name:name}
			}).collect()}).unwrap();
			
	databases
}
//Test function
fn list_tables(p: &my::Pool, db_name:&db) -> Vec<tb>{
	let mut con = p.get_conn().unwrap();

	//Switch to selected database
	let cmd = "USE ".to_owned() + &db_name.name;
	con.query(cmd).unwrap();
	//Show tables in that database
	let tables: Vec<tb> = con.prep_exec("SHOW TABLES",())
		.map(|result|{
			result.map(|x| x.unwrap()).map(|row|{
			let name = my::from_row(row);
			tb{name:name}//, db_name:&databases[db_index].name}
			}).collect()
		}).unwrap();

			
	tables
}