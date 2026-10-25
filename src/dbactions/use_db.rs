/* Use db */
use rusqlite::{Connection, Result};

pub fn use_db(path: String) -> Result<Connection, Box<dyn std::error::Error>> {
    Ok(Connection::open(path).unwrap())
}