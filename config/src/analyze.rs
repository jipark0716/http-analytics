use crate::{Ai, DatabaseConfig, HttpConfig, TomlAi, TomlDatabaseConfig, TomlHttpConfig};
use bincode::{BorrowDecode, Encode};
use serde::Deserialize;

#[derive(BorrowDecode, Debug)]
pub struct HttpAnalyzeConfig<'de> {
    pub http: HttpConfig,

    pub clickhouse: DatabaseConfig<'de>,
    pub ai: Ai<'de>,
}

#[derive(Deserialize, Encode, Debug)]
pub struct TomlHttpAnalyzeConfig {
    pub http: TomlHttpConfig,
    pub clickhouse: TomlDatabaseConfig,
    pub ai: TomlAi,
}