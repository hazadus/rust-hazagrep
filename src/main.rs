use std::env;
use std::process;

use rust_hazagrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("hazagrep running");
    println!("Searching for '{}' in file '{}'", config.query, config.file_path);

    if let Err(e) = rust_hazagrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}