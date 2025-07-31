pub mod collect;
pub mod analyze;

use bincode::config::standard;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;

#[derive(Deserialize, Serialize, Decode, Encode, Debug)]
pub struct HttpConfig {
    pub port: u16,
}

#[derive(Deserialize, Serialize, Decode, Encode, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub batch_size: usize,
}

#[derive(Deserialize, Serialize, Decode, Encode, Debug)]
pub struct Ai {
    pub engine: String,
    pub api_key: String,
}

pub fn export<T>()
where
    T: for<'de> Deserialize<'de> + Encode,
{
    let profile = if std::env::var("CARGO_FEATURE_PRODUCTION").is_ok() {
        "production"
    } else if std::env::var("CARGO_FEATURE_STAGING").is_ok() {
        "staging"
    } else if std::env::var("CARGO_FEATURE_DEVELOPMENT").is_ok() {
        "development"
    } else {
        panic!("알수 없는 features");
    };

    let in_path = format!("./config/{}.toml", profile);
    let config_toml =
        fs::read_to_string(&in_path).expect(format!("{} is not found", in_path).as_str());
    let config: T =
        toml::from_str(&config_toml).expect(format!("fail parse config {}", config_toml).as_str());
    let bin = bincode::encode_to_vec(&config, standard()).expect("serialize failed");

    let out_path = format!("./config/{}.bin", profile);
    fs::write(&out_path, bin).expect("write failed");
}

pub fn import<T>(bin: &[u8]) -> Arc<T>
where
    T: Decode<()>,
{
    Arc::new(bincode::decode_from_slice(bin, standard())
        .expect("Failed to decode")
        .0)
}
