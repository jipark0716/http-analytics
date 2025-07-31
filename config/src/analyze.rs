use std::sync::Arc;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::{Ai, DatabaseConfig, HttpConfig};

#[derive(Deserialize, Serialize, Decode, Encode, Debug)]
pub struct HttpAnalyzeConfig {
    pub http: HttpConfig,

    #[serde(bound(deserialize = "DatabaseConfig: Deserialize<'de>"))]
    pub clickhouse: Arc<DatabaseConfig>,

    #[serde(bound(deserialize = "Ai: Deserialize<'de>"))]
    pub ai: Arc<Ai>,
}