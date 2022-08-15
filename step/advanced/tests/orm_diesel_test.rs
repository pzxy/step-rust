use diesel::prelude::*;
use log::info;

#[test]
pub fn establish_connection()  {
    env_logger::init();
    let database_url = String::from("./asd.db");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
   println!("asd")
}