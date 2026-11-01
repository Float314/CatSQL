use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

pub const CONFIG_FILENAME: &str = "config.toml";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SQLDialect {
    #[serde(rename = "mysql", alias = "MySQLDialect", alias = "MySQL")]
    MySQLDialect,
    #[serde(
        rename = "generic",
        alias = "SQLParserDialect",
        alias = "GenericDialect",
        alias = "Generic"
    )]
    SQLParserDialect,
    #[serde(
        rename = "postgresql",
        alias = "PostgreSQLDialect",
        alias = "Postgres",
        alias = "postgres"
    )]
    PostgreSQLDialect,
}

impl Default for SQLDialect {
    fn default() -> Self {
        Self::MySQLDialect
    }
}

impl SQLDialect {
    /// Every dialect CatSQL can parse.
    pub const ALL: [SQLDialect; 3] = [
        SQLDialect::MySQLDialect,
        SQLDialect::SQLParserDialect,
        SQLDialect::PostgreSQLDialect,
    ];

    /// Name shown to the user by the `.dialect` dot command.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MySQLDialect => "MySQL Dialect",
            Self::SQLParserDialect => "Generic (sqlparser) Dialect",
            Self::PostgreSQLDialect => "PostgreSQL Dialect",
        }
    }
}

impl FromStr for SQLDialect {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "mysql" | "mariadb" | "mysqldialect" => Ok(Self::MySQLDialect),
            "generic" | "ansi" | "sqlparser" | "sqlparserdialect" => Ok(Self::SQLParserDialect),
            "postgres" | "postgresql" | "pg" | "postgresqldialect" => Ok(Self::PostgreSQLDialect),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Configfile {
    #[serde(rename = "Dialect", alias = "dialect")]
    pub dialect: SQLDialect,
    #[serde(rename = "ComputerNetworkOwner", alias = "computer_network_owner")]
    pub computer_network_owner: String,
}

impl Default for Configfile {
    fn default() -> Self {
        Self {
            dialect: SQLDialect::default(),
            computer_network_owner: "Admin <noreply@localhost>".into(),
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cfgdir = dirs::config_dir().ok_or("Could not determine config directory")?;
    Ok(cfgdir.join("catsql"))
}

pub fn get_config_file() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(get_config_dir()?.join(CONFIG_FILENAME))
}

pub fn save_config(config: &Configfile) -> Result<(), Box<dyn std::error::Error>> {
    let cfgdir = get_config_dir()?;
    std::fs::create_dir_all(&cfgdir)?;

    let content = toml::to_string_pretty(config)?;
    std::fs::write(cfgdir.join(CONFIG_FILENAME), content)?;
    Ok(())
}

pub fn create_default_dir() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(get_config_dir()?)?;
    Ok(())
}

pub fn create_default_config() -> Result<(), Box<dyn std::error::Error>> {
    create_default_dir()?;
    save_config(&Configfile::default())
}

pub fn fetch_config_file() -> Result<Configfile, Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string(get_config_file()?)?;
    let config_serialised: Configfile = toml::from_str(&config)?;

    Ok(config_serialised)
}

pub fn check_config_file() -> Result<(), Box<dyn std::error::Error>> {
    if !get_config_file()?.exists() {
        create_default_config()?;
    }

    Ok(())
}

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
