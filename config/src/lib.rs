pub mod collect;
pub mod analyze;

use bincode::config::standard;
use bincode::{BorrowDecode, Encode};
use serde::{Deserialize};
use std::fs;

#[derive(BorrowDecode, Debug)]
pub struct HttpConfig {
    pub port: u16,
}

#[derive(Deserialize, Encode, Debug)]
pub struct TomlHttpConfig {
    pub port: u16,
}

#[derive(BorrowDecode, Debug)]
pub struct DatabaseConfig<'a> {
    pub host: &'a str,
    pub user: &'a str,
    pub password: &'a str,
    pub database: &'a str,
    pub batch_size: usize,
}

#[derive(Deserialize, Encode, Debug)]
pub struct TomlDatabaseConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub batch_size: usize,
}

#[derive(BorrowDecode, Debug)]
pub struct Ai<'a> {
    pub engine: &'a str,
    pub api_key: &'a str,
}

#[derive(Deserialize, Encode, Debug)]
pub struct TomlAi {
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

pub fn import<'de, T>(bin: &'de [u8]) -> T
where
    T: BorrowDecode<'de, ()>,
{
    bincode::borrow_decode_from_slice(bin, standard())
        .expect("Failed to decode")
        .0
}
