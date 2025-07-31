use config::export;
use config::collect::HttpCollectConfig;

fn main() {
    export::<HttpCollectConfig>();
}