use rusqlite::{self, Connection, fallible_iterator::Unwrap};
/* Tokenizer in rust */
use shlex;
use std::process;
use colorize::{self, AnsiColor};

use crate::dbactions::{self, create, use_db};

fn gen_tokens(inp: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let tokens = shlex::split(&inp)
        .ok_or("invalid shell syntax")?;
    Ok(tokens)
}

// parse tokens and match their current values for now.
fn parse_tokens(gen_tokens: Vec<String>, conn: Connection) {
    let mut current_db = conn;
    if gen_tokens[0] == "hi" {
        println!("Recieved `hi`");
    }
    else if gen_tokens[0] == "create" {
        if gen_tokens.len() <= 1 {
            println!("{} - Provide an argument!", "Error!".red().bold())
        }
        else if gen_tokens[1] == "database" {
            create::create_db(<String as Clone>::clone(&gen_tokens[2]))
                .expect("Error! Could not create Database!");
        } else if gen_tokens[1] == "table" {
            let result = create::create_table(gen_tokens, current_db);
            match result {
                Ok(_) => {
                    println!("{} - Table created successfully", "Success!");
                }
                Err(e) => {
                    println!("{} - {}", "Error!".red().bold(), e);
                }
            }
        } else {
            println!("{} Check docs and retry!", "Error! ");
        }
    }
    else if gen_tokens[0] == "quit" {
        println!("{}", "Quitting!".bold().b_red());
        process::exit(0); //quit successfully
    } 
    else if gen_tokens[0] == "use" {
        current_db = use_db::use_db(<String as Clone>::clone(&gen_tokens[1])).unwrap();
    }
    else {
        println!(
            "{} - `{}` Check docs and try again.",
            "Unknown Command!".red().bold(),
            gen_tokens[0]
        );
    }
}

pub fn tokenize(inp: String, db: rusqlite::Connection) {
    let gen_tok = gen_tokens(inp).unwrap();
    
    parse_tokens(gen_tok, db);
}