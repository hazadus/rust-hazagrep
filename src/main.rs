use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    println!("hazagrep running");
    println!("Searching for '{}' in file '{}'", query, file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to reaf the file.");

    println!("\nFile content:\n\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    // Get parameters from command line.
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}