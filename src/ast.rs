/* 
 * CatSQL - the low cost SQL Terminal!
 * Copyright (C) 2026, Float314. 
 * 
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/


use colorize::AnsiColor;
use sqlparser::{
    ast::Statement,
    dialect::{Dialect, GenericDialect, MySqlDialect, PostgreSqlDialect},
    parser::{Parser, ParserError},
};
use std::process;
use tabled::{Table, Tabled};

use crate::configfile::{self, SQLDialect::{self}};

#[derive(Tabled)]
struct HelpCmd {
    name: &'static str,
    description: &'static str,
}

/// Turn CatSQL's configured dialect into the matching sqlparser dialect.
fn parser_dialect(dialect: SQLDialect) -> Box<dyn Dialect> {
    match dialect {
        SQLDialect::MySQLDialect => Box::new(MySqlDialect {}),
        SQLDialect::SQLParserDialect => Box::new(GenericDialect {}),
        SQLDialect::PostgreSQLDialect => Box::new(PostgreSqlDialect {}),
        SQLDialect::AnsiDialect => Box::new(sqlparser::dialect::AnsiDialect {})
    }
}

/* 
 * Generate AST Based on Dialect and Query.
 * query: &str -> The query to feed to sqlparser. 
 * dialect_sql: SQLDialect (enum) -> Dialect. (more details on configfile.rs )
 *
*/
pub fn generate_ast(
    query: &str,
    dialect_sql: SQLDialect,
) -> Result<Vec<Statement>, ParserError> {
    let dialect = parser_dialect(dialect_sql);
    Parser::parse_sql(dialect.as_ref(), query)
}

/*
 * Function to exit program with a cute goodbye message. 
 *
 * Returns only () and nothing else
*/
fn quitcatsql() -> () {
    println!("{}", "Thanks for using CatSQL!".blue().bold());
    process::exit(0);
}

/*
 * Parse Dotcommands. 
 *
 * What are dotcommands - 
 * Dotcommands are a type of `Escape Commands` to escape from SQL and
 * into CatSQL Settings. 
 *
 * Supported Dotcommands - 
 * license, help, dialect, quit, setup, ast [yet to implement]
 *
 * To-do: IMplement `ast` command! [DONE]
 * */
pub fn matchdotcmds(
    mut query: String, /* IT HAS TO BE WITH THE DOTS */
    dialect: &mut SQLDialect,
) {
    query.drain(..1);

    let mut parts = query.split_whitespace();
    let command = parts.next().unwrap_or("");

    match command {
        "license" => {
            // license
            println!("licensew")
        }
        "notlicense" => {
            println!("notlicense ig")
        }
        "help" => {
            println!("This help is only for {}.", "Dot commands".bold().yellow());
            println!(
                "{} Dialect Specific instruction is not implemented {}.",
                "Warning - ".yellow().bold(),
                "yet".italic()
            );
            let help = vec![
                HelpCmd {
                    name: ".license",
                    description: "Show License",
                },
                HelpCmd {
                    name: ".quit | .exit",
                    description: "Exits the program",
                },
                HelpCmd {
                    name: ".ast",
                    description: "Shows AST Generated (Diagnostics)",
                },
                HelpCmd {
                    name: ".dialect",
                    description: "Shows the configured SQL dialect. Use\n\
                                  `.dialect <mysql|postgresql|generic|ansi>` to change it",
                },
                HelpCmd {
                    name: ".setup",
                    description: "Shows the config file location and how to change settings",
                },
                HelpCmd {
                    name: ".clear",
                    description: "Clears the terminal screen (i had to add 20 rust deps to \nimplement it)"
                }
            ];
            let tabeeel = Table::new(help);
            println!("{}", tabeeel);
        }
        "dialect" => {
            let requested: Vec<&str> = parts.collect();

            if requested.is_empty() {
                println!(
                    "This program is set up to use the {}",
                    dialect.display_name().green().bold()
                );
                return;
            }

            let name = requested.join(" ");
            match name.parse::<SQLDialect>() {
                Ok(new_dialect) => match configfile::set_dialect(new_dialect) {
                    Ok(()) => {
                        *dialect = new_dialect;
                        println!(
                            "Dialect changed to the {}",
                            new_dialect.display_name().green().bold()
                        );
                    }
                    Err(err) => println!(
                        "{} Could not save the new dialect: {}",
                        "Error!".bold().red(),
                        err
                    ),
                },
                Err(_) => {
                    let choices = SQLDialect::ALL
                        .iter()
                        .map(SQLDialect::display_name)
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!(
                        "{} Unknown dialect '{}'. Available dialects: {}",
                        "Error!".bold().red(),
                        name,
                        choices
                    );
                }
            }
        }
        "quit" => {
            quitcatsql();
        }
        "exit" => {
            quitcatsql();
        }
        "setup" => match configfile::get_config_file() {
            Ok(path) => {
                println!("Config file: {}", path.display());
                println!("Current dialect: {}", dialect.display_name().green().bold());
                println!("Change it with `.dialect <mysql|postgresql|generic>`.");
            }
            Err(err) => println!(
                "{} Could not locate the config file: {}",
                "Error!".bold().red(),
                err
            ),
        },
        "ast" => {
            // remove the `ast` in the query
            let requested: Vec<&str> = parts.collect();

            let config = configfile::get_config_or_create().unwrap();
            let dialect = config.dialect;

            let query = requested.join(" ");
            let ast_produced = match generate_ast(&query, dialect) {
                Ok(ast) => ast,
                Err(err) => {
                    crate::errordisplay::catch_err_print(&query, &err);
                    return;
                }
            };
            println!("{:?} \n", ast_produced);
            println!(
                "{} The Dialect Selected was {}",
                "AST Generation successful!".bold().green(), 
                dialect.display_name().bold()
            );
        }
        "clear" => {
            clearscreen::clear().expect("Failed to clear terminal! Try Again or read `clearscreen` documentation");
        }
        _ => {
            println!(
                "Unknown Dot Command: {} {}",
                command.to_string().bold(),
                "Check Docs and try again!".cyan().bold()
            );
            println!(
                "{} CatSQL Is still in its {} stage. Some dotcommands could also be unimplemented.",
                "Note".cyan().bold(),
                "initial".italic()
            );
        }
    }
}
