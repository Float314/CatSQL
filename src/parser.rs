/* We pass query to sqlparser and then to libsql */
use shlex::{self, Shlex};

use crate::create_db;
use crate::ast;

fn splitcommand_shlex(command: String) 
    -> Vec<String> {
        let arg = Shlex::new(&command);

        let vect: Vec<String> = arg.collect();

        return vect;
}

pub async fn match_args(command: String)
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
                Err(err) => {println!("{}", err.to_string()); return Err("".into())}
            }

            for statement in ast_produced.unwrap() {
                match statement {
                    sqlparser::ast::Statement::Query(_) => {
                        println!("query")
                    },
                    sqlparser::ast::Statement::Use(_) => {
                        print!("use")
                    },
                    
                    sqlparser::ast::Statement::CreateDatabase { 
                        db_name, 
                        if_not_exists: _, 
                        location: _, 
                        managed_location: _, 
                        or_replace: _, 
                        transient: _, 
                        clone: _, 
                        data_retention_time_in_days: _, 
                        max_data_extension_time_in_days: _, 
                        external_volume: _, 
                        catalog: _,
                        replace_invalid_characters: _, 
                        default_ddl_collation: _,
                        storage_serialization_policy: _,
                        comment: _,
                        default_charset: _, 
                        default_collation: _, 
                        catalog_sync: _, 
                        catalog_sync_namespace_mode: _, 
                        catalog_sync_namespace_flatten_delimiter: _, 
                        with_tags: _, 
                        with_contacts: _ } => 
                    {
                        create_db::create_db(db_name.to_string()).await; // really, eh? 

                    },
                    _ => {
                        return Err("Error statement".into());
                    }
                }
            }
        }
        Ok(())
}
