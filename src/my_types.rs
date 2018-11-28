use interface;
use interface::Entry;
use interface::Key;
use interface::Table;
use my;

pub struct MysqlTable<E: Entry>{
	//names are based on the mysql names
	pub tb_name:String,
	pub db_name: String,
	pub key_name: E::FieldNames,
	pub pool: my::Pool, //The pool that the user is connected to at the time. Use open_mysql(...) to get a Pool
	pub field: Vec<E::FieldNames>, //List of the fields in the tables, excludes key field
	
}

impl <E:Entry>Table<E> for MysqlTable<E>{
	// functions for insert, lookup, delete, contains, and search
	// These functions insert/lookup from the mysql database, not a local table

	//Defines what type the key is
	type Key = MysqlTableKey;	

	//Searches the tables for a key
	fn lookup(&self, key: Self::Key) -> Option<E>{
		//Always start with opening mysql
		//Opening mysql will never panic if done with the openmysql function
		let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
		let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
		con.query(cmd_db).unwrap();//Sending known command
		
		let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ " WHERE "+&self.key_name.to_string()+ " = " + &key.id.to_string();
		let vec_result: Vec<Vec<my::Value>> = con.prep_exec(cmd,())
			.map(|result|{
				result.map(|x| x.unwrap()).map(|row|{//Panics if schema is not followed
					//Iterates through each row
					let vec_data = my::Row::unwrap(row); //Returns a vector of my::Values for each row
					//Panics if schema is not followed
					vec_data //The order of the vector depends on the order inside the mySQL database
				}).collect()
			}).unwrap();//Panics if schema is not followed
		let this_result =&vec_result[0]; //Saves the desired entry to a seperate vec
		//Change the my::Value to interface::Value, start by declaring needed variables
		let mut the_result = Vec::new();
		let mut i:usize = 0;
		let mut good_value: bool = true;
		let _err_string = "Failed to convert mySQL Value".to_string();
		//Iterate through this_result and convert each myValue to iValue
		while i < this_result.len() {
			let temp = myvalue_to_ivalue(&this_result[i]);
			//Check the result for an error
			match temp{
				Err(_err_string) => good_value = false,
				_ => the_result.push(temp.unwrap()), //unwraps after checking it's okay, so it won't panic
			};
			i=i+1;
		}

		
		// Make a return based on the presence of an error
		match &good_value {
			true=> Some(Entry::from_fields(&the_result[1..]).unwrap()),//unwraps after checking it's okay, so it won't panic
			_ => None
		}

	}

	//Inserts a new row into the table and returns a key
	//Uses QueryResult.last_insert_id to get a key back
	fn insert(&mut self, entry: E) -> Self::Key{
		//Always start with opening mysql
		//Opening mysql will never panic if done with the openmysql function
		let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
		let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
		con.query(cmd_db).unwrap();//Sending known command
		
		
		let _values :String = String::new(); //Create blank strings to hold to the fields and data
		let _data :String = String::new();
		//Create one big string for all data from Vec<interface:Value>
		let _entry_string = String::new();
		let entry_vec = entry.get_fields();//Get the data as a string, must be ordered in the same way as fields
		let entry_vec_string :Vec<String>= entry_vec.iter().map(|x| {
			ivalue_to_mystring(x)
		}).collect();
		let entry_string = entry_vec_string.join(", ");//Creates one big string from the string vec
		//Repeat entry string but for the values
		let field_iter = self.field.iter();
		let mut value_vec = Vec::new();
		for i in field_iter{
			value_vec.push(i.to_string());
		}
		let values = value_vec.join(", ");
		//Generate the command with mySQL syntax and the 2 previous strings
		let cmd = &("INSERT INTO ".to_string() + &self.tb_name +
			" (" + &self.key_name.to_string() + ", " + &values + 
			") VALUES (NULL, " + &entry_string + ")");


		let qr = con.query(cmd).is_ok();//Send the prepared statement defined earlier and return a bool if it is okay
		//Get last entry in that table, Because we know what exactly what is sent, the unwraps won't panic
		let this_key: Vec<MysqlTableKey> = con.prep_exec("SELECT LAST_INSERT_ID()",())
		.map(|result|{
			result.map(|x| x.unwrap()).map(|row|{
			let id = my::from_row(row);
			MysqlTableKey{id:id, valid:true}
			}).collect()
		}).unwrap();//Sending known command, will not panic
		
		//Return a value based on good results
		if qr{
			this_key[0]
		} else{
			MysqlTableKey{id: 0, valid:false}//Returns 0 and an indicator of a bad key
		}

	}	
	fn search(&self, field_name: E::FieldNames, field_value: interface::Value) -> Result<Vec<(Self::Key, E)>,String>{
		//SELECT * IN tb_name WHERE field_name = field_value
		//Always start with opening mysql
		//Opening mysql will never panic if done with the openmysql function
		let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
		let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
		con.query(cmd_db).unwrap();//Sending known command
		
		let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ " WHERE "+&field_name.to_string()+ " = " + &field_value.to_string();
		let vec_result: Vec<Vec<my::Value>> = con.prep_exec(cmd,())
			.map(|result|{
				result.map(|x| x.unwrap()).map(|row|{
					//Iterates through each row, panics if the schema is not followed
					let vec_data = my::Row::unwrap(row); //Returns a vector of my::Values for each row
					vec_data //The order of the vector depends on the order inside the mySQL database
				}).collect()
			}).unwrap();//Panics if schema is not followed
		let mut final_result: Vec<(Self::Key, E)> = Vec::new();
		let mut j:usize = 0;
		let mut good_value = true;
		let mut err_string = "Failed to convert mySQL Value".to_string();
		while j <vec_result.len(){
			let this_result =&vec_result[j]; //Saves the desired entry to a seperate vec
			//Change the my::Value to interface::Value
			let mut the_result = Vec::new();
			let mut i:usize = 0;
			while i < this_result.len() {
				let temp = myvalue_to_ivalue(&this_result[i]);
				//Check the result for an error
				match temp{
					Err(_err_string) => good_value = false,
					_ => the_result.push(temp.unwrap()), //unwraps after checking it's okay, so it won't panic
				};
				i=i+1;
			}
			// Make a return based on the presence of an error
			match &good_value {
				true=> {
					let my_key= MysqlTableKey{
						id: my::from_value(this_result[0].to_owned()),
						valid:true
					};
					let end_result:Result<E,String> = Entry::from_fields(&the_result[1..]);
					match &end_result{
						Ok(_e) => final_result.push((my_key,end_result.clone().unwrap())),//unwraps after checking it's okay, so it won't panic
						_=> err_string = "Database did not return a valid entry".to_string(),
					};					
				},
				_ => err_string = "Failed to convert mySQL Value".to_string(),
			};
			j=j+1;
		}//End of while loop
		match &good_value {
			true => Ok(final_result),
			_=> Err(err_string),
		}
	}
	
	fn update(&self, key: Self::Key, entry: E)-> Result<(), String>{
		//UPDATE tb_name SET field 1= entry 1, field 2 = entry 2, ... WHERE id = key
		//Always start with opening mysql
		//Opening mysql will never panic if done with the openmysql function
		let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
		let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
		con.query(cmd_db).unwrap();//Sending known command
		
		//Create string for field x = entry x|
		let field_iter = self.field.iter();
		let entry_vec = entry.get_fields();//Get the data as a string, must be ordered in the same way as fields
		let mut entry_iter= entry_vec.iter().map(|x| {
			ivalue_to_mystring(x)
		}); //Converts the values into a string iterator
		let mut set_vec :Vec<String>= Vec::new(); // String that will hold each field x = entry x
		for i in field_iter {
			set_vec.push(i.to_string() + " = " + &entry_iter.next().unwrap()); 
			//Will not panic since ivalue_to_mystring will always return a valid string
		}
		let set = set_vec[..].join(", ");	//Combines the set_vec (as a slice) into one string seperated by ,
		
		//Create string for cmd
		let cmd = "UPDATE ".to_string() + &self.tb_name + " SET " + &set + " WHERE "+ &self.key_name.to_string() + " = " +&key.id.to_string();
		
		//Send cmd and see if it is good
		let qr = con.query(cmd);
		
		let f : Result <(), String>= match qr {
        Ok(_query_result) => Ok(()),
        Err(_error) => Err("There was a problem updating the user, please consult sysadmin".to_string()),
		};
    	f
	}
	
    fn remove(&mut self, key: Self::Key) -> Result<(), String>{
		//DELETE FROM tb_name WHERE key_name = key
		//Always start with opening mysql
		//Opening mysql will never panic if done with the openmysql function
		let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
		let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
		con.query(cmd_db).unwrap();//Sending known command
		
		//let mut cmd = String::new();
		let cmd = "DELETE FROM ".to_string() + &self.tb_name + " WHERE " + &self.key_name.to_string() + " = " +&key.id.to_string();
		let qr = con.query(cmd);
		
		let f : Result <(), String>= match qr {
			Ok(_query_result) => Ok(()),
			Err(_error) => Err("There was a problem deleting the user, please consult sysadmin".to_string()),
		};
    	f
	
	}
	/*
	fn update (&mut self, key: Self::Key) ->Result<(), String>{
		
	
	}
	*/
    fn contains(&self, key: Self::Key) -> bool{
		//Same as lookup but returns a bool if the query result returns anything
		//Always start with opening mysql
		//Opening mysql will never panic if done with the openmysql function
		let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
		let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
		con.query(cmd_db).unwrap();//Sending known command
		
		let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ " WHERE "+&self.key_name.to_string()+ " = " + &key.id.to_string();
		let vec_result: Vec<Vec<my::Value>> = con.prep_exec(cmd,())
			.map(|result|{
				result.map(|x| x.unwrap()).map(|row|{
					//Iterates through each row, will panic if schema is not followed
					let vec_data = my::Row::unwrap(row); //Returns a vector of my::Values for each row
					vec_data //The order of the vector depends on the order inside the mySQL database
				}).collect()
			}).unwrap();//Will panic if schema is not followed
		if vec_result.len() != 0 {
			true
		}else{
			false
		}
	}			
}
fn myvalue_to_ivalue(start:&my::Value)->Result<interface::Value,String>{
	let _temp :interface::Value;
	match start{
		my::Value::Int(_i64) 	=> Ok(interface::Value::Integer	(my::from_value(start.to_owned()))),
		my::Value::Float(_f64)	=> Ok(interface::Value::Float	(my::from_value(start.to_owned()))),
		my::Value::Bytes(_vec)	=> Ok(interface::Value::String	(my::from_value(start.to_owned()))),
		_ => Err("Failed to convert mySQL Value".to_string()),
	}
}
fn ivalue_to_mystring(data: &interface::Value)->String{
	match data{
		interface::Value::Integer(_i32) => data.to_owned().to_string(),
		interface::Value::Float(_f32)   => data.to_owned().to_string(),
		//Strings need quotes around them. This assumes that all other characters have already been escaped
		interface::Value::String(_string) => {
			let temp :Vec<String> = data.to_owned().to_string().split('\'').map({|x|
				x.to_string() //Creates a gap wherever there was a ' (deletes the ')
			}).collect();
			"'".to_string() + &temp.join("\\\'") + "'"//Adds a \' in between each gap
		},
	}
}
	//Opens a pooled connection to mySQL and returns the pool used to acess it
	//This only works when the database is on the same machine that it's being executed on
pub fn open_mysql(user: String, pass:String)-> Result<my::Pool,String>{
	let mut  optbuild = my::OptsBuilder::new();

	optbuild.ip_or_hostname(Some("localhost"))
		.user(Some(user.trim()))
		.pass(Some(pass.trim()));

	let optcon = optbuild;
	let p = my::Pool::new(optcon);
	match p{
		Ok(_) => Ok(p.unwrap()),//unwraps after checking it's okay, so it won't panic
		Err(_) => Err("Username and password do not match".to_string())
	}
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MysqlTableKey{
	pub id: usize,
	pub valid:bool
}

impl <E:Entry> Key<E> for MysqlTableKey{ }
