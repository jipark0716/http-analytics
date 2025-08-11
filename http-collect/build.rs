use config::export;
use config::collect::{TomlHttpCollectConfig};

fn main() {
    export::<TomlHttpCollectConfig>();
}