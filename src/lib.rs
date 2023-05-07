use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  // This usage of the impl Trait syntax we discussed in the “Traits as Parameters”
  // section of Chapter 10 means that args can be any type that implements the
  // Iterator type and returns String items.
  //
  // Because we’re taking ownership of args and we’ll be mutating args by iterating
  // over it, we can add the mut keyword into the specification of the args parameter
  // to make it mutable.
  pub fn build(
    mut args: impl Iterator<Item = String>,
  ) -> Result<Config, &'static str> {
      // First agrument is program's name, we want to skip it
      args.next();

      let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Did't get a query string"),
      };

      let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Did't get a file path"),
      };

      // We don’t care about the value of the environment variable, 
      // just whether it’s set or unset, so we’re checking is_ok
      // rather than using unwrap, expect
      let ignore_case = env::var("IGNORE_CASE").is_ok();

      Ok(Config { 
        query, 
        file_path, 
        ignore_case, 
      })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
  let contents = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in results {
    println!("{line}");
  }

  Ok(())
}

// The lifetime parameters specify which argument lifetime is connected to the
// lifetime of the return value. In this case, we indicate that the returned
// vector should contain string slices of `contents`. In other words, we tell
// Rust that the data returned by the `search` function will live as long as
// the data in the `contents` argument.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }
}