use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct User
{
    id: i32,
    name: String,
    password: String,
}

fn database_connection() -> Result<Connection>
{
    let conn = Connection::open("database.db3");
    // let conn = Connection::open_in_memory();
    return conn;
}

pub fn create_database() -> Result<()>
{
    let conn = database_connection()?;
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY,name TEXT NOT NULL,password TEXT NOT NULL)", [],)?;
    Ok(())
}
