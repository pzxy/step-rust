#[macro_use]
extern crate diesel;

use diesel::prelude::*;

pub mod schema;
pub mod models;

pub fn establish_connection() -> SqliteConnection {
    let database_url = String::from("./demo.db");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}