/* Create Database */
use libsql::{Builder};

pub async fn create_db(name: String) -> Result<(),Box<dyn std::error::Error>>{
    let filename = name + ".sqlite";
    let filepath = "./".to_owned() + &filename;
    let db = Builder::new_local(filepath)
        .build().await?;

    Ok(())
}