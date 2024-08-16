extern crate rusqlite;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()> {
    // Establish a connection to a new or existing SQLite database
    let conn = Connection::open("my_database.db")?;

    // Create a table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL,
                  age INTEGER
                  )",
        [],
    )?;

    // Insert some data
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        params!["Alice", 30],
    )?;
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        params!["Bob", 25],
    )?;

    // Query the data
    let mut stmt = conn.prepare("SELECT id, name, age FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    // Print the results
    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}

