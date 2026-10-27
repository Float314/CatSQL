use colorize::*;
use inquire::*;
use rusqlite::Connection;
use rustyline::{self, DefaultEditor};

mod tokenizer;
mod dbactions;
mod tokenidentifier;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pwd = Password::new("Enter Password")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt();

    println!("{}", "Welcome!".green().bold());

    // Keep one connection for the whole session. `use` replaces it with the
    // selected database, so later commands run against the same database.
    let mut db: Option<Connection> = None;

    loop {
        let mut rl = DefaultEditor::new()?;
        let readline = rl.readline("catsql> ");

        let lin = match readline {
            Ok(line) => line,
            Err(_) => break,
        };

        tokenizer::tokenize(lin, &mut db);
    }
    Ok(())
}