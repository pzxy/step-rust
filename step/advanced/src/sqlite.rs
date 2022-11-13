use log::info;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[test]
fn sqlite_test() -> Result<()> {
    env_logger::init();
    info!("sqlite demo");
    let conn = Connection::open_in_memory()?;
    let _usize = conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (),
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    let _usize = conn.execute("INSERT INTO person (name, data) VALUES (?1, ?2)", (&me.name, &me.data))?;
    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map((), |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for p in person_iter {
        info!("Found{:?}",p.unwrap());
    }
    Ok(())
}

