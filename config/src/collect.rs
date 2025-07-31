use std::sync::Arc;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::{DatabaseConfig, HttpConfig};

#[derive(Deserialize, Serialize, Decode, Encode, Debug)]
pub struct HttpCollectConfig {
    pub http: HttpConfig,
    #[serde(bound(deserialize = "DatabaseConfig: Deserialize<'de>"))]
    pub clickhouse: Arc<DatabaseConfig>,
}