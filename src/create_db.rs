/* Create Database */
use libsql::{Builder, Connection, Database};

pub async fn create_or_use_db(name: String) -> Result<(Database, Connection),Box<dyn std::error::Error>>{
    let filename = if name.ends_with(".sqlite") { name.clone() } else { name + ".sqlite" };
    let filepath = "./".to_owned() + &filename;
    let db = Builder::new_local(filepath)
        .build().await?;
    let connection = db.connect()?;

    Ok((db, connection))
}

pub async fn memory_db_load() -> Result<(Database, Connection), Box<dyn std::error::Error>> {
    let db = Builder::new_local(":memory:")
        .build().await?;
    let connection = db.connect()?;
    Ok((db, connection))
}