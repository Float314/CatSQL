/* We use sqlparser to generate an AST */

use colorize::AnsiColor;
use sqlparser::{
    ast::Statement,
    dialect::{Dialect, GenericDialect, MySqlDialect, PostgreSqlDialect},
    parser::Parser,
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

pub fn generate_ast(
    query: String,
    dialect_sql: SQLDialect,
) -> Result<Vec<Statement>, Box<dyn std::error::Error>> {
    let dialect = parser_dialect(dialect_sql);

    match Parser::parse_sql(dialect.as_ref(), &query) {
        Ok(ast) => Ok(ast),
        Err(err) => {
            println!("{} {}", "Error!".bold().red(), err);
            Err(format!(
                "Could not parse query using the {}",
                dialect_sql.display_name()
            )
            .into())
        }
    }
}

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
                    description: "Shlw License",
                },
                HelpCmd {
                    name: ".quit",
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
            process::exit(0);
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
            println!("{}", "ast command is yet to implement!".blue());
        }
        _ => {
            println!(
                "Unknown Dot Command: {} {}",
                command.to_string().bold(),
                "Check Docs and try again!".cyan().bold()
            );
            println!(
                "{} CatSQL Is still in its alpha stage. Some dotcommands could also be unimplemented.",
                "Note".cyan().bold()
            );
        }
    }
}
