use colorize::*;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

mod parser;
mod create_db;
mod ast;

/*  let dialect = GenericDialect {};

    let sql = "SELECT * FROM users WHERE a > b;";

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("{:?}", ast); */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    println!("{}", "CatSQL v0.1 Alpha.".blue().bold());
    println!("Licensed under {}", "the GNU LGPL v3.".green().bold());

    let dialect = GenericDialect{};

    let astig = Parser::parse_sql(&dialect, "
        SELECT * FROM students WHERE rollno == 10 OR marks >= 59"
    ).unwrap();

    println!("{:?}",astig);

    let mut editorlin = rustyline::DefaultEditor::new()?;

    loop {
        let rl = editorlin.readline("catsql> ");

        match rl {
            Ok(line) => {
                parser::match_args(line).await?;
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
