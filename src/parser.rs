use colorize::AnsiColor;
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

pub async fn match_args(mut command: String)
    -> Result<(), Box<dyn std::error::Error>> {
        // first detect if its a dotcommand

        let first_char = command.chars().next().unwrap_or(' ');

        if first_char == '.' {
            ast::matchdotcmds(command.clone());
        } else {
            // generate ast
            let mut ast_produced = ast::generate_ast(command);

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
                        if_not_exists, 
                        location, 
                        managed_location, 
                        or_replace, 
                        transient, 
                        clone, 
                        data_retention_time_in_days, 
                        max_data_extension_time_in_days, 
                        external_volume, 
                        catalog,
                        replace_invalid_characters, 
                        default_ddl_collation,
                        storage_serialization_policy,
                        comment,
                        default_charset, 
                        default_collation, 
                        catalog_sync, 
                        catalog_sync_namespace_mode, 
                        catalog_sync_namespace_flatten_delimiter, 
                        with_tags, 
                        with_contacts } => 
                    {
                        create_db::create_db(db_name.to_string()); // really, eh? 

                    },
                    _ => {
                        return Err("Error statement".into());
                    }
                }
            }
        }
        Ok(())
}
