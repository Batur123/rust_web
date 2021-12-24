use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct User
{
    id: i32,
    name: String,
    password: String,
}

fn create_database() -> Result<()>
{
    let conn = Connection::open_in_memory()?;
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY,name TEXT NOT NULL,password TEXT NOT NULL)", [],)?;
    Ok(())
}