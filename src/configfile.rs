use toml;
use serde::{Deserialize, Serialize, self};
use sqlparser::dialect;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub enum SQLDialect {
    MySQLDialect,
    SQLParserDialect,
    PostgreSQLDialect
} // for now only three dialect

#[derive(Debug, Serialize, Deserialize)]
pub struct Configfile {
    Dialect: SQLDialect,
    ComputerNetworkOwner: String
}

pub fn load_config() -> Result<Configfile, Box<dyn std::io::Error>> {
    
};