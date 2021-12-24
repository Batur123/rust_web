#[macro_use]
extern crate throw;

use rocket::futures::future::err;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct User {
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
    insert_into_users("Batur".to_string(),"123456".to_string());
    Ok(())
}

pub fn insert_into_users(mut name: String, password: String) -> Result<(), throw::Error<&'static str>>
{
    if name.trim().is_empty()
    {
        throw_new!("oops");
    }

    if password.trim().is_empty()
    {
        throw_new!("oops");
    }

    let firstuser = User
    {
        id: 0,
        name: name.to_string(),
        password: password.to_string(),
    };

    let conn = database_connection()?;
    conn.execute("INSERT INTO person (name, password) VALUES (?1, ?2)", params![firstuser.name, firstuser.password], )?;
    Ok(())
}
