use std::fs;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub file_path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
      // Extract parameters from command line.
      if args.len() < 3 {
          return Err("not enough arguments!")
      }

      let query = args[1].clone();
      let file_path = args[2].clone();
  
      Ok(Config { query, file_path })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
  let contents = fs::read_to_string(config.file_path)?;

  println!("\nFile content:\n\n{contents}");

  Ok(())
}

// The lifetime parameters specify which argument lifetime is connected to the
// lifetime of the return value. In this case, we indicate that the returned
// vector should contain string slices of `contents`. In other words, we tell
// Rust that the data returned by the `search` function will live as long as
// the data in the `contents` argument.
pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}