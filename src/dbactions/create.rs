/*
 * Create DB Using rusqlite
 *
 */

use rusqlite::Connection;
use colorize::*;

// ----------------------- Database ------------------------------------------
pub fn create_db(name: String) -> Result<(), Box<dyn std::error::Error>> {
    match create_db_rn(name) {
        Ok(_) => {
            println!("{}", "Database created successfully.".green().bold());
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn create_db_rn(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let name2: String = name + ".sqlite";

    // Opening a SQLite database creates the file if it does not exist.
    Connection::open(name2)?;

    Ok(())
}

// --------------------- Table ----------------------------------------------

pub fn create_table(queri: &[String], database: &Connection) ->
Result<(), Box<dyn std::error::Error>>
{
    // The tokens come from shlex::split, so joining them with spaces restores
    // the SQL. Do NOT use shlex::try_join here: it quotes characters such as
    // '(' and ')' and turns valid SQL into invalid SQL.
    let query = queri.join(" ");

    database.execute(&query, ())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_table_uses_valid_sql() {
        let db = Connection::open_in_memory().unwrap();
        let tokens: Vec<String> = [
            "create", "table", "users", "(id", "integer", "primary", "key,", "name", "text)",
        ]
        .iter()
        .map(|token| token.to_string())
        .collect();

        create_table(&tokens, &db).unwrap();

        let count: i64 = db
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'users'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 1);
    }
}
