// use rusqlite::{Connection};

// #[derive(Debug)]
// struct Person {
//     id: i32,
//     name: String,
//     data: Option<Vec<u8>>,
// }

#[test]
fn sqlite_test()  {
    // env_logger::init();
    // println!("sqlite demo");
    // let conn = Connection::open_in_memory();
    // let _usize = conn.execute(
    //     "CREATE TABLE person (
    //         id    INTEGER PRIMARY KEY,
    //         name  TEXT NOT NULL,
    //         data  BLOB
    //     )",
    //     (),
    // ).unwarp();
    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };
    // let _usize = conn.execute("INSERT INTO person (name, data) VALUES (?1, ?2)", (&me.name, &me.data)).unwarp();
    // let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwarp();
    // let person_iter = stmt.query_map((), |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // }).unwarp();

    // for p in person_iter {
    //     println!("Found{:?}",p.unwrap());
    // }
}

