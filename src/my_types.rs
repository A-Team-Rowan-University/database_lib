use interface;
use interface::Entry;
use interface::Key;
use interface::Table;
use interface::FieldName;
use my;

pub struct mysql_table{
	//names are based on the mysql names
	pub tb_name:String,
	pub db_name: String,
	pub key_name: String,
	pub pool: my::Pool, //The pool that the user is connected to at the time. Use open_mysql(...) to get a Pool
	pub field: Vec<String>, //List of the fields in the tables, excludes key field
	
}

impl <E:Entry>Table<E> for mysql_table{
	// functions for insert, lookup, delete, contains, and search
	// These functions insert/lookup from the mysql database, not a local table

	//Defines what type the key is
	type Key = mysql_table_key;

	//Searches the tables for a key
	fn lookup(&self, key: Self::Key) -> Option<E>{
		let mut con = self.pool.get_conn().unwrap();
		
		let cmd_db = "USE ".to_owned() + &self.db_name;
		con.query(cmd_db).unwrap();
		
		let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ " WHERE "+&self.key_name+ " = " + &key.id.to_string();
		println!("{}",cmd);
		let vec_result: Vec<Vec<my::Value>> = con.prep_exec(cmd,())
			.map(|result|{
				result.map(|x| x.unwrap()).map(|row|{
					//Iterates through each row
					let vec_data = my::Row::unwrap(row); //Returns a vector of my::Values for each row
					vec_data //The order of the vector depends on the order inside the mySQL database
				}).collect()
			}).unwrap();
		let this_result =&vec_result[0]; //Saves the desired entry to a seperate vec
		//Change the my::Value to interface::Value
		let mut the_result = Vec::new();
		let mut i:usize = 0;
		while i < this_result.len() {
			let temp = myValue_to_iValue(this_result[i]);
			the_result.push(temp);
			i=i+1;
		}
		let end_result = Entry::from_fields(&the_result[..]).unwrap();
		
		Some(end_result)
	}

	//Inserts a new row into the table and returns a key
	//Uses QueryResult.last_insert_id to get a key back
	fn insert(&mut self, entry: E) -> Self::Key{
		let mut values :String = String::new(); //Create blank strings to hold to the fields and data
		let mut data :String = String::new();
		let mut entry_string:Vec<String> = Vec::new();
		let entry_vec = entry.get_fields();//Get the data as a string, must be ordered in the same way as fields
		let mut i:usize = 0;
		while i < entry_vec.len(){
			entry_string[i] =entry_vec[i].to_string();
			i=i+1;
		}
		//Concatinate the fields and data into 2 large strings
		i=0;
		while i < self.field.len(){
			values = values.to_owned() + ", "+&self.field[i] ;
			data   = data.to_owned()   + ", "+&entry_string[i];
			i=i+1;
		}
		//Generate the command with mySQL syntax and the 2 previous strings
		let cmd = &("INSERT INTO ".to_string() + &self.tb_name +
			" (" + &self.key_name + &values + 
			") VALUES (NULL" + &data + ")");
		
		//println!("{}",cmd);//Uncomment if you want to check what you just sent
		let mut con = self.pool.get_conn().unwrap();//Open connection to mySQL
		
		let cmd_db = "USE ".to_owned() + &self.db_name;//Open the proper database
		con.query(cmd_db).unwrap();
		
		con.prep_exec(cmd,()).unwrap();//Send the prepared statement defined earlier
		//Get last entry in that table
		let this_key: Vec<mysql_table_key> = con.prep_exec("SELECT LAST_INSERT_ID()",())
			.map(|result|{
				result.map(|x| x.unwrap()).map(|row|{
				let id = my::from_row(row);
				mysql_table_key{id:id}
				}).collect()
			}).unwrap();
        this_key[0]
	}	
	fn search(&self, field_name: E::FieldNames, field_value: interface::Value) -> Vec<(Self::Key, E)>{
	//SELECT * IN tb_name WHERE field_name = field_value
	
	}
    fn remove(&mut self, key: Self::Key) -> Result<(), String>{
	//REMOVE FROM tb_name WHERE key_name = key
	
	}
    fn contains(&self, key: Self::Key) -> bool{
	//Same as lookup but returns a bool if the query result returns anything
	
	}			
}
fn myValue_to_iValue(start:my::Value)->interface::Value{
	let mut temp :interface::Value;
	match start{
		my::Value::Int(i64) 	=> temp = interface::Value::Integer	(my::from_value(start)),
		my::Value::Float(f64)	=> temp = interface::Value::Float	(my::from_value(start)),
		my::Value::Bytes(Vec)	=> temp = interface::Value::String	(my::from_value(start)),
		_ => (),
	};
	temp
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct mysql_table_key{
	id: usize
}

impl <E:Entry> Key<E> for mysql_table_key{ }
