/* Use db */
use rusqlite::{Connection, Result};
use colorize::{self, AnsiColor};

// we will not include tokenizer cause well, self explanatory :/

pub fn use_db(path: String) -> Result<Connection, Box<dyn std::error::Error>> {
    let path2: String = path.clone() + ".sqlite";
    let db = Connection::open(path2.clone());

    match &db { // all i missing in [RUST] is an [AMPERSAND]
        Ok(c) => println!("{} - Using Database `{}'", "Query OK!".bold().green(), path.clone().bold()),
        Err(f) => println!("{} Cannot open the specified database.", "Error!".bold().red())
    }

    let db_opened = db.unwrap();
    Ok(db_opened) 
}