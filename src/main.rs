use std::env;
use std::process;

use rust_hazagrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("hazagrep running");
    println!("Searching for '{}' in file '{}'", config.query, config.file_path);

    if let Err(e) = rust_hazagrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}