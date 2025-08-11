use crate::{DatabaseConfig, HttpConfig, TomlDatabaseConfig, TomlHttpConfig};
use bincode::{BorrowDecode, Encode};
use serde::{Deserialize};

#[derive(BorrowDecode, Debug)]
pub struct HttpCollectConfig<'de> {
    pub http: HttpConfig,
    pub clickhouse: DatabaseConfig<'de>,
}

#[derive(Deserialize, Encode, Debug)]
pub struct TomlHttpCollectConfig {
    pub http: TomlHttpConfig,
    pub clickhouse: TomlDatabaseConfig,
}