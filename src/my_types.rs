extern crate mysql as my;
use interface::Entry;
use interface::Key;
use interface::Table;

#[derive(Debug)]
pub struct mysql_table{
	//names are based on the mysql names
	pub tb_name:String,
	pub db_name: String,
	pub key_name: String,
	pub pool:my::Pool, //The pool that the user is connected to at the time. Use open_mysql(...) to get a Pool
	pub field: Vec<String>, //List of the fields in the tables, excludes key field
	
}


impl <E:Entry+ 'static>Table<E> for mysql_table{
	// functions for insert and lookup
	// These functions insert/lookup from the mysql database, not a local table

	//Defines what type the key is
	type Key = ();

	//Searches the tables for a key
	fn lookup(&self, key: Box<dyn Key<E>>) -> Option<E>{
		let mut con = self.pool.get_conn().unwrap();
		
		let cmd_db = "USE ".to_owned() + &self.db_name;
		con.query(cmd_db).unwrap();
		
		let cmd = "SELECT * FROM ".to_string()+&self.tb_name+ " WHERE "+&self.key_name+ " = " + &key.downcast_ref::<mysql_table_key>().unwrap().id.to_string();
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
		let end_result = Entry::from_mysql(this_result);
		
		Some(end_result)
	}

	//Inserts a new row into the table and returns a key
	//Uses QueryResult.last_insert_id to get a key back
	fn insert(&mut self, entry: E) -> Box<dyn Key<E>>{
		let mut values :String = String::new(); //Create blank strings to hold to the fields and data
		let mut data :String = String::new();
		let entry_string = entry.to_vec_string();//Get the data as a string, must be ordered in the same way as fields
		//Concatinate the fields and data into 2 large strings
		let mut i=0;
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
		//Box the key and send it out
		Box::new(mysql_table_key {
            id: this_key[0].id
        })
			
			

	}
}

#[derive(Debug)]
pub struct mysql_table_key{
	id: usize
}

impl <E:Entry + 'static> Key<E> for mysql_table_key{
	fn same_as(&self, other: Box<dyn Key<E>>) -> bool {
		/*
        if let Some(other_key) = other.downcast_ref::<mysql_table_key>() {
            self.id == other_key.id
        } else {
            false
        }
		*/
		true
    }
}
