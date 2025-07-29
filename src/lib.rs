use std::error::Error;
use std::fs;
use std::path::Path;

use clap::Parser;

use walkdir::WalkDir;

/// Parameters (arguments) of the cli program
#[derive(Parser, Debug)]
#[command(version, about = None, long_about = None)]
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
    /// Search a folder recursively (optional)
    #[arg(short, long)]
    pub recurse: bool,
}

/// Implementation of the Config struct
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let query = args[1].clone();
        let filename = args[2].clone();
        let case = string_to_bool(args[3].clone());
        let verbose = string_to_bool(args[4].clone());
        let whole = string_to_bool(args[5].clone());
        let recurse = string_to_bool(args[6].clone());

        Ok(Config {
            query,
            filename,
            case,
            verbose,
            whole,
            recurse,
        })
    }
}

/// Case-sensitive search
/// Returns all the lines of the contents where the query is present
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Case-insensitive search
/// Returns all the lines of the contents where the query is present
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

/// Recursive search
/// Returns all the lines of all the files and folders mentioned in the config where the query is present
pub fn search_recursive(config: &Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.filename);
    if !path.is_dir() {
        eprintln!("Error: --recurse flag set but path is not a directory");
        return Err("Invalid directory".into());
    }

    for entry in WalkDir::new(&config.filename)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        if config.verbose {
            println!("Searching in file: {}", file_path.display());
        }

        let contents = fs::read_to_string(file_path);
        if config.whole {
            println!("Contents of search :\n{:?}\n", contents);
        }

        if let Ok(contents) = contents {
            let results = if config.case {
                search_case_sensitive(&config.query, &contents)
            } else {
                search_case_insensitive(&config.query, &contents)
            };

            if !results.is_empty() {
                println!("\nIn file {}:", file_path.display());
                for line in results {
                    println!("{}", line);
                }
            }
        } else if config.verbose {
            eprintln!("Failed to read file: {}", file_path.display());
        }
    }

    Ok(())
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
    use super::{search_case_sensitive, search_case_insensitive};

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
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

    #[test]
    fn whole_file() {
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
