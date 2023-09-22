use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TomlConfig {
    pub http: HttpConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
    pub trace: TraceConfig,
}

#[derive(Deserialize, Debug)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LogConfig {
    pub directory: String,
    pub file_name_prefix: String,
}

#[derive(Deserialize, Debug)]
pub struct TraceConfig {
    pub rolling_file: RollingFileConfig,
    pub console: ConsoleConfig
}

#[derive(Deserialize, Debug)]
pub struct RollingFileConfig {
    pub directory: String,
    pub file_name_prefix: String,
    pub rotation: String,
    pub app_only: bool,
    pub with_max_level: String,
    pub with_file: bool,
    pub with_line_number: bool,
    pub with_target: bool,
}

#[derive(Deserialize, Debug)]
pub struct ConsoleConfig {
    pub app_only: bool,
    pub with_max_level: String,
    pub with_file: bool,
    pub with_line_number: bool,
    pub with_target: bool,
}

impl TomlConfig {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let config = toml::from_str::<TomlConfig>(&contents)?;
        Ok(config)
    }
}
