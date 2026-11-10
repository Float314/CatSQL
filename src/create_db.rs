/* 
 * CatSQL - the low cost SQL Terminal!
 * Copyright (C) 2026, Float314. 
 * 
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

use libsql::{Builder, Connection, Database};
use std::path::PathBuf;

pub fn get_database_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path_to_cfg = dirs::data_local_dir().ok_or("Could Not determine Local Database Directory!")?;
    Ok(path_to_cfg.join("catsql"))
}

/* 
 * Create database. If exist, use the database. 
 * Rust's function for creation or using of Database 
 * has the same function. 
 *
 * To-do: Set a default directory for SQL Creation!
*/
pub async fn create_or_use_db(name: String) -> Result<(Database, Connection),Box<dyn std::error::Error>>{

    let filename = if name.ends_with(".sqlite") { name.clone() } else { name + ".sqlite" };
    let database_dir = get_database_dir()?;
    // SQLite cannot create the database file if its parent directory is
    // missing, which is the case on a fresh install (or after moving the
    // databases out of the current directory).
    std::fs::create_dir_all(&database_dir)?;
    let filepath = database_dir.join(filename).to_string_lossy().into_owned();
    let db = Builder::new_local(filepath)
        .build().await?;
    let connection = db.connect()?;

    Ok((db, connection))
}

/*
 * Load database from memory. If database is disconnected,
 * then we try to open a blank database that is loaded in 
 * memory. 
*/
pub async fn memory_db_load() -> Result<(Database, Connection), Box<dyn std::error::Error>> {
    let db = Builder::new_local(":memory:")
        .build().await?;
    let connection = db.connect()?;
    Ok((db, connection))
}
