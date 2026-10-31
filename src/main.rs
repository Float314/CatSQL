use colorize::*;

use crate::create_db::memory_db_load;

mod parser;
mod create_db;
mod ast;
mod queryfetch;

/*  let dialect = GenericDialect {};

    let sql = "SELECT * FROM users WHERE a > b;";

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("{:?}", ast); */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    println!("{}", "CatSQL v0.1 Alpha.".blue().bold());
    println!("Licensed under {}", "the GNU LGPL v3.".green().bold());

    let mut editorlin = rustyline::DefaultEditor::new()?;
    let mut db = memory_db_load().await.unwrap();

    loop {
        let rl = editorlin.readline("catsql> ");

        match rl {
            Ok(line) => {
                match parser::match_args(line, &mut db).await {
                    Ok(_) => {},
                    Err(_e) => println!("Error occured!")
                };
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("{}", "Ctrl-c".red());
                break;
            }
            Err(_) => {
                println!("Something bad happened!");
                return Err("idk".into());
            }
        }
    }
    Ok(())
}
