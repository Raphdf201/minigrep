use clap::Parser;

use std::error::Error;
use std::fs;

/// Runs a search with the provided config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.verbose {
        println!("Parsing file {}", config.filename)
    }
    let contents = fs::read_to_string(&config.filename)?;
    if config.whole { 
        println!("Contents of search :\n{}", contents)
    }
    let results: Vec<&str> = if config.verbose {
        println!("Checking case sensitivity");
        if config.case {
            println!("Running case-sensitive search");
            search(&config.query, &contents)
        } else {
            println!("Running case-insensitive search");
            search_case_insensitive(&config.query, &contents)
        }
    } else if config.case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    if results.is_empty() {
        println!("Found no line containing {}", config.query);
    } else {
        println!("Lines that contain \"{}\":", config.query);
        for line in results {
            println!("{}", line);
        }
    }
    Ok(())
}

/// Parameters of the application
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// The string to search
    pub query: String,
    /// The file to search in
    pub filename: String,
    /// Case sensitivity (optional)
    #[arg(short, long)]
    pub case: bool,
    /// Verbose logging (optional)
    #[arg(short, long)]
    pub verbose: bool,
    /// Print whole file (optional)
    #[arg(short, long)]
    pub whole: bool,
}

/// Implementation of the Config struct
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let query = args[1].clone();
        let filename = args[2].clone();
        let case = string_to_bool(args[3].clone());
        let verbose = string_to_bool(args[4].clone());
        let whole = string_to_bool(args[5].clone());

        Ok(Config {
            query,
            filename,
            case,
            verbose,
            whole,
        })
    }
}

/// case-sensitive search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn get_whole_file() {
    
}

/// case-insensitive search
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

/// Serializes a string into a boolean
pub fn string_to_bool(string: String) -> bool {
    let mut is_bool = false;
    if string == "true" {
        is_bool = true;
    } else if string == "false" {
        is_bool = false;
    } else {
        eprintln!("An error occurred during a string_to_bool operation");
    }

    is_bool
}

#[cfg(test)]
mod tests {
    use super::{search, search_case_insensitive};

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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
