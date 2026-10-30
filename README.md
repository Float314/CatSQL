# CatSQL - The low cost SQL Command line 
> Because cats are low maintainance animals :D 

CatSQL is a low-effort SQL Command line which implements 
- SQL Parsing (Thrugh sqlparser-rs)
- Preety Result and error appearance (through colorize)
- sqlite backend (libsql for rust) which is preety damn fast
- its damn cheap
- Single binary o' a few megabytes 

Liberate yourself brothas of the world! Cats are more less maintainence
animals than dolphins (gewt the refs, dolphins are still cute tho :D)

# License
GNU LGPL v3. May change to GNU AGPL but ill announce if thata the case :P

<!--
To know which type of SQL statement is being executed when using **`sqlparser-rs`**, you need to inspect the top-level **`Statement` enum** provided by the library's AST.

When you parse a SQL string, `sqlparser-rs` returns a vector of `Statement` variants (e.g., `Vec<Statement>`). You can use Rust's pattern matching (`match` statement or `if let`) to check which variant the statement belongs to.

---

### How to Check the Statement Type in Rust

Here is a practical code example showing how to inspect and match different statement types:

```rust
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::Statement;

fn main() {
    let sql = "USE school_db; DROP TABLE students; SELECT * FROM marks;";
    let dialect = GenericDialect {};

    // Parse the SQL string into a vector of Statements
    let statements = Parser::parse_sql(&dialect, sql).unwrap();

    for statement in statements {
        match statement {
            Statement::Query(_) => {
                println!("Found a Query statement (like SELECT, INSERT/UPDATE queries)");
            }
            Statement::Use(_) => {
                println!("Found a USE statement");
            }
            Statement::Drop { .. } => {
                println!("Found a DROP statement");
            }
            Statement::CreateTable { .. } => {
                println!("Found a CREATE TABLE statement");
            }
            _ => {
                println!("Found another type of SQL statement");
            }
        }
    }
}

```

### Key Variants inside the `Statement` Enum

`sqlparser-rs` handles dozens of different SQL commands by wrapping them in the `Statement` enum. Some of the most common ones include:

* **`Statement::Query(Query)`**: Covers `SELECT` queries, as well as `WITH`, `VALUES`, and subquery blocks.
* **`Statement::Use(Use)`**: Covers database context switching (`USE db_name`).
* **`Statement::Drop { ... }`**: Covers removal of database objects (`DROP TABLE`, `DROP VIEW`, etc.).
* **`Statement::CreateTable { ... }`**: Covers schema definitions.
* **`Statement::Insert { ... }`**, **`Update { ... }`**, **`Delete { ... }`**: Covers data manipulation language (DML) statements.

-->