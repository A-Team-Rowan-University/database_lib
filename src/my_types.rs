use interface;
use interface::Entry;
use interface::Key;
use interface::Table;
use interface::QueryType;
use my;
use std::marker::PhantomData;
use interface::ITryInto;

pub static MAX_LIMIT : u16 = 100;
pub static DEFAULT_KEY: MysqlTableKey = MysqlTableKey{id: 0, valid: false};

#[derive(Debug, Clone)]
pub struct MysqlTable<E: Entry>{
	//names are based on the mysql names
	pub tb_name:String,
	pub db_name: String,
	pub key_name: String,
	pub pool: my::Pool, //The pool that the user is connected to at the time. Use open_mysql(...) to get a Pool
	pub phantom: PhantomData<E>,
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
		
		let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ " WHERE "+&self.key_name+ " = " + &key.id.to_string();
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
		//INSERT INTO tb_name (fields ) VALUES (values (NULL for auto increment));
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
		let fields = E::get_field_names();
		let field_iter = fields.iter(); //Cuts off the ID
		let mut value_vec = Vec::new();
		for i in field_iter{
			value_vec.push(i.to_string());
		}
		let values = value_vec.join(", ");
		//Generate the command with mySQL syntax and the 2 previous strings
		let cmd = &("INSERT INTO ".to_string() + &self.tb_name +
			" (" + &self.key_name + ", " + &values + 
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
		//SELECT * FROM tb_name WHERE field_name = field_value
		//Always start with opening mysql
		//Opening mysql will never panic if the pool is done with the openmysql function
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
		let fields = E::get_field_names();
		let field_iter = fields.iter();
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
		let cmd = "UPDATE ".to_string() + &self.tb_name + " SET " + &set + " WHERE "+ &self.key_name + " = " +&key.id.to_string();
		
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
		let cmd = "DELETE FROM ".to_string() + &self.tb_name + " WHERE " + &self.key_name + " = " +&key.id.to_string();
		let qr = con.query(cmd);
		
		let f : Result <(), String>= match qr {
			Ok(_query_result) => Ok(()),
			Err(_error) => Err("There was a problem deleting the user, please consult sysadmin".to_string()),
		};
    	f
	
	}
	
    fn contains(&self, key: Self::Key) -> bool{
		//Same as lookup but returns a bool if the query result returns anything
		//Always start with opening mysql
		//Opening mysql will never panic if done with the openmysql function
		let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
		let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
		con.query(cmd_db).unwrap();//Sending known command
		
		let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ " WHERE "+&self.key_name+ " = " + &key.id.to_string();
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
	
	//Make a MysqlTableKey<User>::new({struct data});
	fn query(&self, q: QueryType<E>, key: Option<Self::Key>) -> Result<Vec<(Self::Key, E)>,String>{
		// Uses query type to decide wihch function to use and get the neccessary data
		//Because of the need for Self, the key is taken seperately, but isn't always needed
		match &q{
			QueryType::Lookup=>{
				//Key required
				if key == None {
					return Err("Must have a key".to_string())
				}
				
				let result = self.lookup(key.unwrap()); //Unwraps after checking its okay
				match result {
					Some(this_key) => {
						let mut result_vec: Vec<(Self::Key,E)> = Vec::new();
						result_vec.push((key.unwrap(),this_key)); //Unwraps after checking its okay
						Ok(result_vec)
					},
					None =>{
						Err("Invalid key".to_string())
					}
				}
			
			},
			QueryType::Search(field,val,lim,sort_field,sort_dir, pg)=>{
				//No key required
				//SELECT * FROM tb_name WHERE field_name = field_value
				//Always start with opening mysql
				//Opening mysql will never panic if the pool is done with the openmysql function
				let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
				let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
				con.query(cmd_db).unwrap();//Sending known command
				
				let mut limit = lim;
				if limit > &MAX_LIMIT {
					limit = &MAX_LIMIT;
				}
				let start_limit = limit*(pg-1);
				//Sort and limit string
				let sort_string: String;
				match &sort_dir {
					interface::SortDirection::Asc =>  sort_string = " ASC ".to_string(),
					interface::SortDirection::Desc => sort_string = " DESC ".to_string(),
				}
				let lim_cmd = " ORDER BY ".to_string() + &sort_field.to_string()+&sort_string+"LIMIT "+&start_limit.to_string()+", "+&limit.to_string();

				let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ " WHERE "+&field.to_owned().to_string()+ " = " + &ivalue_to_mystring(&val) + &lim_cmd;
		
				let vec_result: Result<Vec<(MysqlTableKey, E)>,String> = con.prep_exec(cmd,()).map(|result|{
					result.map(|x| x.unwrap()).map(|row|{
						//Iterates through each row, panics if the schema is not followed
						let vec_data = my::Row::unwrap(row);//Returns a vector of my::Values for each row
						let iter_data = vec_data.iter(); 
						let ivec : Vec<interface::Value> = iter_data.map(|r|{
							myvalue_to_ivalue(r).unwrap() //Changes each my::Value to interface::Value
						}).collect();
						let this_entry_result = E::from_fields(&ivec[1..]);
						match this_entry_result {
							Err(string) => return Err(string),
							Ok(_) => ()
						}
						let this_entry = this_entry_result.unwrap(); //Unwraps after checking its okay
						let this_key = MysqlTableKey {
							id: ivec[0].to_owned().itry_into().unwrap(),
							valid: true
						};
						Ok((this_key,this_entry))
					}).collect()
				}).unwrap();//Panics if schema is not followed
					vec_result
				
			},
			interface::QueryType::GetAll(lim,sort_field,sort_dir, pg)=>{
				//No key required
				//Return all of the given table, but does require a limit
				//Limit is checked against MAX_LIMIT
				
				//Always start with opening mysql
				//Opening mysql will never panic if done with the openmysql function
				let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
				let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
				con.query(cmd_db).unwrap();//Sending known command
				
				let mut limit = lim;
				if limit > &MAX_LIMIT {
					limit = &MAX_LIMIT;
				}
				let start_limit = limit*(pg-1);
				//Sort and limit string
				let sort_string: String;
				match &sort_dir {
					interface::SortDirection::Asc =>  sort_string = " ASC ".to_string(),
					interface::SortDirection::Desc => sort_string = " DESC".to_string(),
				}
				let lim_cmd = " ORDER BY ".to_string() + &sort_field.to_string()+&sort_string+"LIMIT "+&start_limit.to_string()+", "+&limit.to_string();
			
				
				let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ &lim_cmd;
				let vec_result: Result<Vec<(MysqlTableKey, E)>,String> = con.prep_exec(cmd,()).map(|result|{
					result.map(|x| x.unwrap()).map(|row|{
						//Iterates through each row, panics if the schema is not followed
						let vec_data = my::Row::unwrap(row);//Returns a vector of my::Values for each row
						let iter_data = vec_data.iter(); 
						let ivec : Vec<interface::Value> = iter_data.map(|r|{
							myvalue_to_ivalue(r).unwrap() //Changes each my::Value to interface::Value
						}).collect();
						let this_entry_result = E::from_fields(&ivec[1..]);
						match this_entry_result {
							Err(string) => return Err(string),
							Ok(_) => ()
						}
						let this_entry = this_entry_result.unwrap(); //Unwraps after checking its okay
						let this_key = MysqlTableKey {
							id: ivec[0].to_owned().itry_into().unwrap(),
							valid: true
						};
						Ok((this_key,this_entry))
					}).collect()
				}).unwrap();//Panics if schema is not followed
					vec_result
			},
			QueryType::PartialSearch(field,val,lim,sort_field,sort_dir, pg)=>{
				//SELECT * FROM tb_name WHERE field_name LIKE *field_value*
				//Always start with opening mysql
				//Opening mysql will never panic if the pool is done with the openmysql function
				let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
				let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
				con.query(cmd_db).unwrap();//Sending known command
				
				//Change the value to a proper search term.
				//Proper searches need '% ... %'
				let mut search_val : String;
				//This is just a modified ivalue_to_mystring
				match val{
					//Change bool to an if true => 1
					interface::Value::Boolean(_i8) =>  search_val = "'%".to_string() + &val.to_owned().to_string()+"%'",
					interface::Value::Integer(_i32) => search_val = "'%".to_string() + &val.to_owned().to_string()+"%'",
					interface::Value::Float(_f32)   => search_val = "'%".to_string() + &val.to_owned().to_string()+"%'",
					//Strings need quotes around them. This assumes that all other characters have already been escaped
					interface::Value::String(_string) => {
						let temp :Vec<String> = val.to_owned().to_string().split('\'').map({|x|
							x.to_string() //Creates a gap wherever there was a ' (deletes the ')
						}).collect();
						search_val ="'%".to_string() + &temp.join("\\\'") + "%'"//Adds a \' in between each gap and adds the %
					},
				}
				let mut limit = lim;
				if limit > &MAX_LIMIT {
					limit = &MAX_LIMIT;
				}
				let start_limit = limit*(pg-1);
				//Sort and limit string
				let sort_string: String;
				match &sort_dir {
					interface::SortDirection::Asc =>  sort_string = " ASC ".to_string(),
					interface::SortDirection::Desc => sort_string = " DESC ".to_string(),
				}
				let lim_cmd = " ORDER BY ".to_string() + &sort_field.to_string()+&sort_string+"LIMIT "+&start_limit.to_string()+", "+&limit.to_string();
				
				//Make and send the command
				let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ " WHERE "+&field.to_owned().to_string()+ " LIKE " + &ivalue_to_mystring(&val) + &lim_cmd;
				let vec_result: Result<Vec<(MysqlTableKey, E)>,String> = con.prep_exec(cmd,()).map(|result|{
					result.map(|x| x.unwrap()).map(|row|{
						//Iterates through each row, panics if the schema is not followed
						let vec_data = my::Row::unwrap(row);//Returns a vector of my::Values for each row
						let iter_data = vec_data.iter(); 
						let ivec : Vec<interface::Value> = iter_data.map(|r|{
							let temp = myvalue_to_ivalue(r).unwrap(); //Changes each my::Value to interface::Value
							temp
						}).collect();
						let this_entry_result = E::from_fields(&ivec[1..]);
						match this_entry_result {
							Err(string) => return Err(string),
							Ok(_) => ()
						}
						let this_entry = this_entry_result.unwrap(); //Unwraps after checking its okay
						let this_key = MysqlTableKey {
							id: ivec[0].to_owned().itry_into().unwrap(),
							valid: true
						};

						Ok((this_key,this_entry))
					}).collect()
				}).unwrap();//Panics if schema is not followed
					vec_result
			
			},
			QueryType::MultiSearch(field_vec, val_vec, lim,sort_field,sort_dir,pg)=>{
				//No key required
				//SELECT * FROM tb_name WHERE field_name[0] = field_value[0] AND field_name[1] = field_value[1]
				//Always start with opening mysql
				//Opening mysql will never panic if the pool is done with the openmysql function
				let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
				let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
				con.query(cmd_db).unwrap();//Sending known command
				
				let mut limit = lim;
				if limit > &MAX_LIMIT {
					limit = &MAX_LIMIT;
				}
				let start_limit = limit*(pg-1);
				//Sort and limit string
				let sort_string: String;
				match &sort_dir {
					interface::SortDirection::Asc =>  sort_string = " ASC ".to_string(),
					interface::SortDirection::Desc => sort_string = " DESC ".to_string(),
				}
				let lim_cmd = " ORDER BY ".to_string() + &sort_field.to_string()+&sort_string+"LIMIT "+&start_limit.to_string()+", "+&limit.to_string();
				if field_vec.len() != val_vec.len(){
					return Err("Field and Value vectors do not pair".to_string())
				}
				//Put together the search
				let mut search_vec = Vec::new();
				let mut i = 0;
				while i < field_vec.len() {
					search_vec.push(field_vec[i].to_string() + " = " + &ivalue_to_mystring(&val_vec[i]));
					i=i+1;
				}
				let search_cmd = search_vec.join(" AND ");
				let cmd = "SELECT * FROM ".to_string() + &self.tb_name+ " WHERE " +&search_cmd + &lim_cmd;
				println!("{}",cmd);
				let vec_result: Result<Vec<(MysqlTableKey, E)>,String> = con.prep_exec(cmd,()).map(|result|{
					result.map(|x| x.unwrap()).map(|row|{
						//Iterates through each row, panics if the schema is not followed
						let vec_data = my::Row::unwrap(row);//Returns a vector of my::Values for each row
						let iter_data = vec_data.iter(); 
						let ivec : Vec<interface::Value> = iter_data.map(|r|{
							myvalue_to_ivalue(r).unwrap() //Changes each my::Value to interface::Value
						}).collect();
						let this_entry_result = E::from_fields(&ivec[1..]);
						match this_entry_result {
							Err(string) => return Err(string),
							Ok(_) => ()
						}
						let this_entry = this_entry_result.unwrap(); //Unwraps after checking its okay
						let this_key = MysqlTableKey {
							id: ivec[0].to_owned().itry_into().unwrap(),
							valid: true
						};
						println!("{:?}",ivec);
						Ok((this_key,this_entry))
					}).collect()
				}).unwrap();//Panics if schema is not followed
					vec_result
				
				
				
			},
		
		}
	
	
	}
}
//New function and supporting functions for query
impl <E:Entry>MysqlTable<E>{
	pub fn new(pool: my::Pool) -> MysqlTable<E>{
		//The pool that the user is connected to at the time. Use open_mysql(...) to get a Pool
		MysqlTable {
			tb_name: "".to_string(),
			db_name: "".to_string(),
			key_name: "".to_string(),
			pool:pool, 
			phantom: PhantomData,
		}
	}
}

//Generic functions for mySQL
fn myvalue_to_ivalue(start:&my::Value)->Result<interface::Value,String>{
	let _temp :interface::Value;
	match start{
		//my::Value::TinyInt(_i64) 	=> Ok(interface::Value::Boolean	(my::from_value(start.to_owned()))),
		my::Value::Int(_i64) 	=> Ok(interface::Value::Integer	(my::from_value(start.to_owned()))),
		my::Value::Float(_f64)	=> Ok(interface::Value::Float	(my::from_value(start.to_owned()))),
		my::Value::Bytes(_vec)	=> Ok(interface::Value::String	(my::from_value(start.to_owned()))),
		//PUT IN BOOLS HERE
		_ => Err("Failed to convert mySQL Value".to_string()),
	}
}
fn ivalue_to_mystring(data: &interface::Value)->String{
	match data{
		interface::Value::Boolean(_i8) => data.to_owned().to_string(),
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



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct MysqlTableKey{
	pub id: i32,
	pub valid:bool
}

impl <E:Entry> Key<E> for MysqlTableKey{ }
