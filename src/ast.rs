/* We use sqlparser to generate an AST */

use std::{f64::consts::E, process};

use sqlparser::{self, ast::Statement, dialect::{self, GenericDialect}, parser::Parser};

pub fn generate_ast(query: String) -> Result<Vec<Statement>, Box<dyn std::error::Error>> {
    let dialect = GenericDialect{};
    let ast = Parser::parse_sql(&dialect, &query).unwrap();

    Ok(ast)
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
    else if query == "quit" {
        process::exit(0);
    }
}
