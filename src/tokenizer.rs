use rusqlite::Connection;
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
fn parse_tokens(mut gen_tokens: Vec<String>, current_db: &mut Option<Connection>)
    -> Result<(), Box<dyn std::error::Error>>
    {
    if gen_tokens.is_empty() {
        return Err("No agument provided".into());
    }

    if gen_tokens.last().unwrap().chars().last().unwrap() != ';' {
        return Err("Expected Semicolon at last character!".into());
    } 
    if gen_tokens[0] == "hi" {
        println!("Recieved `hi`");
    }
    else if gen_tokens[0] == "create" {
        if gen_tokens.len() <= 1 {
            println!("{} - Provide an argument!", "Error!".red().bold())
        }
        else if gen_tokens[1] == "database" {
            if gen_tokens.len() <= 2 {
                return Err("Provide a database name".into());
            }
            create::create_db(<String as Clone>::clone(&gen_tokens[2]))
                .expect("Error! Could not create Database!");
        } else if gen_tokens[1] == "table" {
            if gen_tokens.len() <= 2 {
                return Err("Provide a table definition".into());
            }

            let database = current_db
                .as_ref()
                .ok_or("No database selected. Run `use <database>` first.")?;

            let result = create::create_table(&gen_tokens, database);
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
    else if gen_tokens[0] == "quit" || gen_tokens[0] == "exit" || gen_tokens[0] == "break" {
        println!("{}", "Quitting!".bold().b_red());
        print!("{}", "The cat says goodbye to you :P \n".blue());
        process::exit(0); //quit successfully
    } 
    else if gen_tokens[0] == "use" {
        if gen_tokens.len() <= 1 {
            return Err("Provide a database name".into());
        }
        *current_db = Some(use_db::use_db(<String as Clone>::clone(&gen_tokens[1]))?);
    }
    else if gen_tokens[0] == "license" {
        println!("This software is licensed under the {}. ", "GNU Lessr General Public License (GPL) v3".green().bold());
        println!("Copyright (C) 2026, Float314; Permission to use, modify and redistribute is granted");
        println!("Under specific conditions. Read <https://www.gnu.org/licenses/lgpl-3.0.html> for more");
        println!("info. \n{}", "The cat is free to anyone adopt and any child to play!. ".blue().bold())
    }
    else if gen_tokens[0] == "disconnect" {
        *current_db = Some(Connection::open_in_memory().unwrap());
        println!(
            "{} {} Using blank database loaded in memory",
            "Query OK!".green().bold(), 
            "Database disconnected.".yellow()
        );
    }
    else {
        println!(
            "{} - `{}` Check docs and try again.",
            "Unknown Command!".red().bold(),
            gen_tokens[0]
        );
    }
    Ok(())
}

pub fn tokenize(inp: String, db: &mut Option<Connection>) {
    let gen_tok = match gen_tokens(inp) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{} {}", "Error! ".red().bold(), e);
            return;
        }
    };

    match parse_tokens(gen_tok, db) {
        Ok(()) => {},
        Err(e) => println!("{} {}", "Error! ".red().bold(), dbg!(e))
    };
}
