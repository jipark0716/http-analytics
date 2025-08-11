use config::analyze::TomlHttpAnalyzeConfig;
use config::export;

fn main() {
    export::<TomlHttpAnalyzeConfig>();
}