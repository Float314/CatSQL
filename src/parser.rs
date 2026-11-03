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

use colorize::AnsiColor;
use crate::ast;
use crate::configfile::SQLDialect;
use crate::create_db;
use crate::queryfetch;
use libsql::Connection;
use sqlparser;

/* 
 * Give to SQLparser to produce an ast, and then 
 * match it against statements. 
 *
 * it also detects dotcommands. 
*/
pub async fn match_args(
    command: String,
    db: &mut Connection,
    dialect: &mut SQLDialect,
) -> Result<(), Box<dyn std::error::Error>> {
    // first detect if its a dotcommand

    let first_char = command.chars().next().unwrap_or(' ');

    if first_char == '.' {
        ast::matchdotcmds(command.clone(), dialect);
    } else {
        // generate ast using the dialect selected in the config file
        let ast_produced = match ast::generate_ast(command, *dialect) {
            Ok(ast) => ast,
            Err(err) => return Err(err),
        };

        for statement in ast_produced {
            match statement {
                sqlparser::ast::Statement::Query(q) => {
                    match queryfetch::fetchquery_print(db.clone(), q.to_string()).await {
                        Ok(_) => println!("{}", "Query OK!".green().bold()),
                        Err(e) => println!("Error: {:?}", e),
                    }
                }
                sqlparser::ast::Statement::Use(e) => match e {
                    sqlparser::ast::Use::Object(en) => {
                        let (_db, conn) = create_db::create_or_use_db(en.to_string()).await?;
                        *db = conn;
                        println!("{}", "Database changed.".green().bold());
                    }
                    sqlparser::ast::Use::Database(en) => {
                        let (_db, conn) = create_db::create_or_use_db(en.to_string()).await?;
                        *db = conn;
                        println!("{}", "Database changed.".green().bold());
                    }
                    _ => {
                        println!("{}", "Unimplemented Feature!".bold().red());
                    }
                },
                sqlparser::ast::Statement::CreateDatabase { db_name, .. } => {
                    let (_db, conn) = create_db::create_or_use_db(db_name.to_string()).await?;
                    *db = conn;
                    println!("{}", "Database changed.".green().bold());
                }
                sqlparser::ast::Statement::CreateTable(e) => {
                    db.execute(&e.to_string(), ()).await?;
                    println!("{}", "Query OK!".green().bold());
                }
                sqlparser::ast::Statement::Delete(e) => {
                    db.execute(&e.to_string(), ()).await?;
                    println!("{}", "Query OK!".green().bold());
                }
                sqlparser::ast::Statement::Insert(e) => {
                    db.execute(&e.to_string(), ()).await?;
                    println!("{}", "Query OK!".green().bold());
                }
                sqlparser::ast::Statement::Update(e) => {
                    db.execute(&e.to_string(), ()).await?;
                    println!("{}", "Query OK!".green().bold());
                }
                sqlparser::ast::Statement::Drop { .. } => {
                    db.execute(&statement.to_string(), ()).await?;
                    println!("{}", "Query OK!".green().bold());
                }
                _ => {
                    return Err("Error statement".into());
                }
            }
        }
    }
    Ok(())
}
