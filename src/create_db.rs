/* Create Database */
use libsql::{Builder};

pub async fn create_db(name: String) -> Result<(),Box<dyn std::error::Error>>{
    let filename = name + ".sqlite";
    let db = Builder::new_local(filename)
        .build().await?;

    Ok(())
}