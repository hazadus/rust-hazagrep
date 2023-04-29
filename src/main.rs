use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("hazagrep running");
    println!("Searching for '{}' in file '{}'", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to reaf the file.");

    println!("\nFile content:\n\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // Extract parameters from command line.
        if args.len() < 3 {
            return Err("Not enough arguments!")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}