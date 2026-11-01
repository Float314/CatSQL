use colorize::AnsiColor;
/* We pass query to sqlparser and then to libsql */
use shlex::{self, Shlex};
use libsql::{Connection};
use sqlparser::ast::ObjectName;
use sqlparser;
use crate::create_db;
use crate::ast;
use crate::queryfetch;

fn splitcommand_shlex(command: String) 
    -> Vec<String> {
        let arg = Shlex::new(&command);

        let vect: Vec<String> = arg.collect();

        return vect;
}

pub async fn match_args(command: String, db: &mut Connection)
    -> Result<(), Box<dyn std::error::Error>> {
        // first detect if its a dotcommand

        let first_char = command.chars().next().unwrap_or(' ');

        if first_char == '.' {
            ast::matchdotcmds(command.clone());
        } else {
            // generate ast
            let ast_produced = ast::generate_ast(command);

            match &ast_produced {
                Ok(_) => {}
                Err(err) => { println!("{}", err.to_string()); return Err("".into()) }
            }

            for statement in ast_produced.unwrap() {
                match statement {
                    sqlparser::ast::Statement::Query(q) => {
                        match queryfetch::fetchquery_print(db.clone(), q.to_string()).await {
                            Ok(_) => println!("{}", "Query OK!".green().bold()),
                            Err(e) => println!("Error: {:?}", e)
                        }
                    },
                    sqlparser::ast::Statement::Use(e) => {
                        match e {
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
                        }
                    },
                    sqlparser::ast::Statement::CreateDatabase { 
                        db_name, 
                        ..
                    } => 
                    {
                        let (_db, conn) = create_db::create_or_use_db(db_name.to_string()).await?;
                        *db = conn;
                        println!("{}", "Database changed.".green().bold());
                    },
                    sqlparser::ast::Statement::CreateTable(e) => {
                        db.execute(&e.to_string(), ()).await?;
                        println!("{}", "Query OK!".green().bold());
                    },
                    sqlparser::ast::Statement::Delete(e) => {
                        db.execute(&e.to_string(), ()).await?;
                        println!("{}", "Query OK!".green().bold());
                    },
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
