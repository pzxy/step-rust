use std::env;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub mod schema;
pub mod models;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    // 从数据库中拿到环境变量
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


// sqlite
// #[macro_use]
// extern crate diesel;
//
// use diesel::prelude::*;
//
// pub mod schema;
// pub mod models;
//
// pub fn establish_connection() -> SqliteConnection {
//     let database_url = String::from("./demo.db");
//     SqliteConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }