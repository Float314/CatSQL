use colorize::*;

use crate::create_db::memory_db_load;

mod ast;
mod configfile;
mod create_db;
mod parser;
mod queryfetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "CatSQL v0.3 Alpha.".blue().bold());
    println!("Licensed under {}", "the GNU LGPL v3.".green().bold());

    let config = configfile::get_config_or_create()?;
    let mut dialect = config.dialect;

    println!("Type .help for help related to CatSQL. Dialect");
    println!("selected is {}", dialect.display_name().green().bold());

    let mut editorlin = rustyline::DefaultEditor::new()?;
    let (_db_obj, mut db) = memory_db_load().await.unwrap();

    loop {
        let rl = editorlin.readline("catsql> ");

        match rl {
            Ok(line) => {
                match parser::match_args(line, &mut db, &mut dialect).await {
                    Ok(_) => {}
                    Err(_e) => println!("Error occured!"),
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
