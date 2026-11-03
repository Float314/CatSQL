/* 
 * CatSQL - the low cost SQL Terminal!
 * Copyright (C) 2026, Float314. 
 * 
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

use colorize::*;

// crate import
mod ast;
mod configfile;
mod create_db;
mod parser;
mod queryfetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // welcome messages
    println!("{}", "CatSQL v0.3 Alpha.".blue().bold());
    println!("Licensed under {}", "the GNU LGPL v3.".green().bold());

    // configs and dialects
    let config = configfile::get_config_or_create()?;
    let mut dialect = config.dialect;

    println!("Type .help for help related to CatSQL. Dialect");
    println!("selected is {}", dialect.display_name().green().bold());

    // rustyline editor load
    let mut editorlin = rustyline::DefaultEditor::new()?;

    // load in-memory database by default.
    let (_db_obj, mut db) = create_db::memory_db_load().await.unwrap();

    // loop for terminal feeling
    loop {
        let rl = editorlin.readline("catsql> ");

        match rl {
            Ok(line) => {
                match parser::match_args(line, &mut db, &mut dialect).await {
                    Ok(_) => {}
                    Err(_e) => println!("Error occured!"),
                };
            }
            /* ctrl-c */
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("{}", "Ctrl-c".red());
                break;
            }
            Err(_) => {
                println!("Something bad happened!");
                return Err("idk".into());
            }
        }
    }
    Ok(())
}
