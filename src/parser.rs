use colorize::AnsiColor;
/* We pass query to sqlparser and then to libsql */
use libsql;
use shlex::{self, Shlex};

fn splitcommand_shlex(command: String) 
    -> Vec<String> {
        let arg = Shlex::new(&command);

        let vect: Vec<String> = arg.collect();

        return vect;
}

pub fn match_args(command: String)
    -> Result<(), Box<dyn std::error::Error>> {
        let argums = splitcommand_shlex(command);

        if argums.is_empty() {
            println!("")
        }
        else if argums[0] == "hi" {
            println!("{}", "Recieved hi".blue().bold())
        }
        else if argums[0] == "license" {
            println!("{} is licensed under the amazing {}", "CarSQL".blue().bold(), "GNU LGPL v3".green().bold())
        } 
        else {
            println!("{} {}", "Unknown Command! ".red().bold(), argums[0])
        }
        Ok(())
}

