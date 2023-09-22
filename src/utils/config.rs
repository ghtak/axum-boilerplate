use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TomlConfig {
    pub http: Http,
    pub database: Database,
    pub log: Log
}

#[derive(Deserialize, Debug)]
pub struct Http {
    pub host: String,
    pub port: u16
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Log {
    pub directory: String,
    pub file_name_prefix: String
}

impl TomlConfig {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let config = toml::from_str::<TomlConfig>(&contents)?;
        Ok(config)
    }
}
