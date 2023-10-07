use std::{fs, net::SocketAddr};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct TomlConfig {
    pub(crate) http: HttpConfig,
    pub(crate) database: DatabaseConfig,
    pub(crate) tracing: TracingConfig,
}

#[derive(Deserialize, Debug)]
pub(crate) struct HttpConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) static_directory: String,
}

impl HttpConfig {
    pub(crate) fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        let s = format!("{}:{}", self.host, self.port).parse()?;
        Ok(s)
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct DatabaseConfig {
    pub(crate) url: String,
    pub(crate) max_connection: u32,
    pub(crate) with_migrations: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct TracingConfig {
    pub(crate) rolling_file: RollingFileConfig,
    pub(crate) console: ConsoleConfig,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RollingFileConfig {
    pub(crate) directory: String,
    pub(crate) file_name_prefix: String,
    pub(crate) rotation: String,
    pub(crate) app_only: bool,
    pub(crate) with_max_level: String,
    pub(crate) with_file: bool,
    pub(crate) with_line_number: bool,
    pub(crate) with_target: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ConsoleConfig {
    pub(crate) app_only: bool,
    pub(crate) with_max_level: String,
    pub(crate) with_file: bool,
    pub(crate) with_line_number: bool,
    pub(crate) with_target: bool,
}

impl TomlConfig {
    pub(crate) fn from_file(filename: &str) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let config = toml::from_str::<TomlConfig>(&contents)?;
        Ok(config)
    }
}
