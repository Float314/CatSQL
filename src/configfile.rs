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

// imports ig...
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

// filename for config file: config.toml
pub const CONFIG_FILENAME: &str = "config.toml";

/* 
 * SQLDialect. We use Serde to serialize names 
 *
 * Code is actually self expalanatory actually. Easy
 * to look isntead of burning your eyes. 
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SQLDialect {
    /* MySQL Dialect */
    #[serde(
        rename = "mysql",
        alias = "MySQLDialect", 
        alias = "MySQL"
    )]
    MySQLDialect,
    /*Generic/SQLParser Dialect. 
     * Not to be confused by ANSI Dialect
    */
    #[serde(
        rename = "generic",
        alias = "SQLParserDialect",
        alias = "GenericDialect",
        alias = "Generic"
    )]
    SQLParserDialect,
    /* PostgreSQL Dialect*/
    #[serde(
        rename = "postgresql",
        alias = "PostgreSQLDialect",
        alias = "Postgres",
        alias = "postgres"
    )]
    PostgreSQLDialect,
    /* ANSI Dialect - the standard one */
    #[serde(
        rename = "ansi",
        alias = "AnsiDialect",
        alias = "ANSI",
        alias = "ansi"
    )]
    AnsiDialect
}

/*
 * I actually dont know what the function does 
 * so please anyone contributing figure out ig...
*/
impl Default for SQLDialect {
    fn default() -> Self {
        Self::MySQLDialect
    }
}

/*
 * Map out SQLDialects.
*/
impl SQLDialect {
    /// Every dialect CatSQL can parse.
    pub const ALL: [SQLDialect; 4] = [
        SQLDialect::MySQLDialect,
        SQLDialect::SQLParserDialect,
        SQLDialect::PostgreSQLDialect,
        SQLDialect::AnsiDialect
    ];

    /// Name shown to the user by the `.dialect` dot command.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MySQLDialect => "MySQL Dialect",
            Self::SQLParserDialect => "Generic (sqlparser) Dialect",
            Self::PostgreSQLDialect => "PostgreSQL Dialect",
            Self::AnsiDialect => "ANSI Dialect"
        }
    }
}

/* Parse SQL Dialects from given string values */
impl FromStr for SQLDialect {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() { 
            "mysql"    | "mariadb"    | "mysqldialect"                          => Ok(Self::MySQLDialect),
            "generic"  | "sqlparser"  | "sqlparserdialect"                      => Ok(Self::SQLParserDialect),
            "postgres" | "postgresql" | "pg"  | "postgresqldialect"             => Ok(Self::PostgreSQLDialect),
            "ansi" | "ansisql"                                                  => Ok(Self::AnsiDialect),
            /* other */_                                                        => Err(()),
        }
    }
}

/*
 * Congfigfile contents. 
 *
 * dialect : SQL DIalect (type -> SQLDIalect (enum) )
 * computer_network_owner: Idk i just added it. Will 
 *              gonna edit it if required ...
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Configfile {
    #[serde(rename = "Dialect", alias = "dialect")]
    pub dialect: SQLDialect,
    #[serde(rename = "ComputerNetworkOwner", alias = "computer_network_owner")]
    pub computer_network_owner: String,
}

/*
 * Default settings for configfile enum. 
 * If it doesnt exists (config.toml) it will write 
 * these values to config.toml as default value.
*/
impl Default for Configfile {
    fn default() -> Self {
        Self {
            dialect: SQLDialect::default(),
            computer_network_owner: "Admin <noreply@localhost>".into(),
        }
    }
}

/// get config dir (string:Std::error::error)
pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cfgdir = dirs::config_dir().ok_or("Could not determine config directory")?;
    Ok(cfgdir.join("catsql"))
}

/*
 * Get Config file (get_config_dir.join(CONFIG_FILENAME and thats it))
*/
pub fn get_config_file() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(get_config_dir()?.join(CONFIG_FILENAME))
}

/// save config to desired location.
pub fn save_config(config: &Configfile) -> Result<(), Box<dyn std::error::Error>> {
    let cfgdir = get_config_dir()?;
    std::fs::create_dir_all(&cfgdir)?;

    let content = toml::to_string_pretty(config)?;
    std::fs::write(cfgdir.join(CONFIG_FILENAME), content)?;
    Ok(())
}

/// create default directory for config
pub fn create_default_dir() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(get_config_dir()?)?;
    Ok(())
}

/// create default config
pub fn create_default_config() -> Result<(), Box<dyn std::error::Error>> {
    create_default_dir()?;
    save_config(&Configfile::default())
}

/// fetch config file and serialize it.
pub fn fetch_config_file() -> Result<Configfile, Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string(get_config_file()?)?;
    let config_serialised: Configfile = toml::from_str(&config)?;

    Ok(config_serialised)
}

/// check if config file exist. if not std::error::Error.
pub fn check_config_file() -> Result<(), Box<dyn std::error::Error>> {
    if !get_config_file()?.exists() {
        create_default_config()?;
    }

    Ok(())
}

/// Get config or create
pub fn get_config_or_create() -> Result<Configfile, Box<dyn std::error::Error>> {
    check_config_file()?;
    fetch_config_file()
}

/// Persist a new dialect while keeping the rest of the user's config intact.
pub fn set_dialect(dialect: SQLDialect) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = get_config_or_create()?;
    config.dialect = dialect;
    save_config(&config)
}

/// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_dialect_names() {
        assert_eq!("mysql".parse(), Ok(SQLDialect::MySQLDialect));
        assert_eq!("PostgreSQL".parse(), Ok(SQLDialect::PostgreSQLDialect));
        assert_eq!("ansi".parse(), Ok(SQLDialect::SQLParserDialect));
        assert!("not-a-dialect".parse::<SQLDialect>().is_err());
    }

    #[test]
    fn reads_old_and_new_config_keys() {
        let old: Configfile = toml::from_str(
            "Dialect = \"MySQLDialect\"\nComputerNetworkOwner = \"Admin <noreply@localhost>\"",
        )
        .unwrap();
        assert_eq!(old.dialect, SQLDialect::MySQLDialect);

        let new: Configfile =
            toml::from_str("dialect = \"postgresql\"\ncomputer_network_owner = \"me\"").unwrap();
        assert_eq!(new.dialect, SQLDialect::PostgreSQLDialect);
        assert_eq!(new.computer_network_owner, "me");
    }

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let config: Configfile = toml::from_str("Dialect = \"generic\"").unwrap();
        assert_eq!(config.dialect, SQLDialect::SQLParserDialect);
        assert_eq!(
            config.computer_network_owner,
            Configfile::default().computer_network_owner
        );
    }
}
