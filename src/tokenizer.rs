/* Tokenizer in rust */
use shlex;
use std::process;
use colorize::{self, AnsiColor};

fn gen_tokens(inp: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let tokens = shlex::split(&inp)
        .ok_or("invalid shell syntax")?;
    Ok(tokens)
}

// parse tokens and match their current values for now.
fn parse_tokens(gen_tokens: Vec<String>) {
    if gen_tokens[0] == "hi" {
        println!("Recieved `hi`");
    }
    else if gen_tokens[0] == "create" {
        println!("Recieved create");
    }
    else if gen_tokens[0] == "quit" {
        println!("{}", "Quitting!".bold().b_red());
        process::exit(0); //quit successfully
    } else {
        println!(
            "{}, `{}` Check docs and try again.",
            "Unknown Command!".red().bold(),
            gen_tokens[0]
        );
    }
}

pub fn tokenize(inp: String) {
    let gen_tok = gen_tokens(inp).unwrap();
    
    parse_tokens(gen_tok);
}