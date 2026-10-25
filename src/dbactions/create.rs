/*
 * Create DB Using rusqlite
 * 
 */

use rusqlite::{self, Connection};
use colorize::{self, AnsiColor};

struct field_table {
    name: String,
    type_f: String,
    len: String
}

// ----------------------- Database ------------------------------------------
pub fn create_db(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let path_unix: String= "~/.catsql_db".into();
    let path_windows: String = "C:/.catsql_db".into();

    if cfg!(target_os = "windows") {
        let _ = std::fs::create_dir_all(path_windows);
        create_db_rn(name).expect("Failed to create database");
    } else {
        let _ = std::fs::create_dir_all(path_unix);
        create_db_rn(name).expect("Failed to create database");
    }

    Ok(())
}

pub fn create_db_rn(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let name2: String = name + ".sqlite" ;

    let _ = rusqlite::Connection::open(name2);

    Ok(())
}

// --------------------- Table ----------------------------------------------

pub fn create_table(queri: Vec<String>, database: Connection)
{
    // son why exec()? 

    let exec = database.execute("", ());

    match exec {
        Ok(_) => println!("{}", "Query OK!".green().bold()),
        Err(e) => println!("{} Details: {}", "Error Called!".red().bold(), dbg!(e))
    }

}