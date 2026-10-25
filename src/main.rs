use colorize::*;
use inquire::*;
use rustyline::{self, DefaultEditor};

mod tokenizer;
mod dbactions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pwd = Password::new("Enter Password")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt();

    println!("{}", "Welcome!".green().bold());

    loop {
        let mut rl = DefaultEditor::new()?;
        let readline = rl.readline("catsql> ");

        /*
        match readline.clone() {
            Ok(str) => println!(""),
            Err(_) => println!("Error!")
        }
        */
        let mut lin = readline.unwrap();

        tokenizer::tokenize(lin);
    }
    Ok(())
}