use colorize::AnsiColor;
/* We pass query to sqlparser and then to libsql */
use libsql::{self, Statement};
use shlex::{self, Shlex};

use crate::create_db;
use crate::ast;

fn splitcommand_shlex(command: String) 
    -> Vec<String> {
        let arg = Shlex::new(&command);

        let vect: Vec<String> = arg.collect();

        return vect;
}

pub async fn match_args(mut command: String)
    -> Result<(), Box<dyn std::error::Error>> {
        // first detect if its a dotcommand

        let first_char = command.chars().next().unwrap_or(' ');

        if first_char == '.' {
            ast::matchdotcmds(command.clone());
        } else {
            println!("not implement ywt");
        }
        Ok(())
}
