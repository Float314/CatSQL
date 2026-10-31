/* Create Database */
use libsql::{Builder, Connection};

pub async fn create_or_use_db(name: String) -> Result<Connection,Box<dyn std::error::Error>>{
    let filename = name + ".sqlite";
    let filepath = "./".to_owned() + &filename;
    let db = Builder::new_local(filepath)
        .build().await?;
    let _connection = db.connect()?;

    Ok(_connection)
}

pub async fn memory_db_load() -> Result<Connection, Box<dyn std::error::Error>> {
    let db = Builder::new_local(":memory:")
        .build().await?;
    let connection = db.connect()?;
    Ok(connection)
}