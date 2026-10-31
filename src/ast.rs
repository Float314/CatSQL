/* We use sqlparser to generate an AST */

use std::process;
use tabled::{Table, Tabled};
use colorize::AnsiColor;
use sqlparser::{self, ast::Statement, dialect::GenericDialect, parser::Parser};

#[derive(Tabled)]
struct help_cmd {
    name: &'static str,
    description: &'static str
}

pub fn generate_ast(query: String) -> Result<Vec<Statement>, Box<dyn std::error::Error>> {
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, &query);

    match &ast {
        Ok(_) => {
            return Ok(ast.unwrap())
        }
        Err(err) => {
            println!("{} {}", "Error!".bold().red(), dbg!(err)); 
            return Err("Thread Panicked".into());
        }
    }
}

pub fn matchdotcmds(mut query: String/* IT HAS TO BE WITH THE DOTS*/) {
    query.drain(..1);

    if query == "license" {
        // license 
        println!("licensew")
    }
    else if query == "notlicense" {
        println!("notlicense ig")
    }
    else if query == "help" {
        println!("This help is only for {}.", "Dot commands".bold().yellow());
        println!("{} Dialect Specific instruction is not implemented {}.", "Warning - ".yellow().bold(), "yet".italic());
        let help = vec![
            help_cmd {name: ".license", description:"Shlw License"},
            help_cmd {name: ".quit", description:"Exits the program"},
            help_cmd {name: ".ast", description:"Shows AST Generated (Diagnostics)"},
            help_cmd {name: ".dialect", description:
                "Shows Which Dialect this program this is configured \nto (ANSI Dialect, MySQL Dialect, sqlparser generic\nDialect)"
            },
        ];
        let tabeeel = Table::new(help);
        println!("{}", tabeeel);
    }
    else if query == "dialect" {
        println!("This program is set up to the {} ", "ANSI Dialect of SQL".green().bold())
    }
    else if query == "quit" {
        process::exit(0);
    }
    else { 
        println!("Unknown Dot Command: {} {}", query.bold(), "Check Docs and try again!".cyan().bold());
        println!("{} CatSQL Is still in its alpha stage. Some dotcommands could also be unimplemented.", "Note".cyan().bold());
    }
}
