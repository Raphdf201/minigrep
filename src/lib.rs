mod tests;

use std::fs;
use std::path::Path;

use clap::Parser;

use walkdir::WalkDir;

pub const RECURSE_NOT_DIR: &str = "--recurse flag set but path is not a directory";

/// Parameters (arguments) of the cli program
#[derive(Parser, Debug, PartialEq)]
#[command(version, about = None, long_about = None)]
pub struct Config {
    /// The string to search
    pub query: String,
    /// The file to search in
    pub path: String,
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
        let path = args[2].clone();
        let case = string_to_bool(args[3].clone());
        let verbose = string_to_bool(args[4].clone());
        let whole = string_to_bool(args[5].clone());
        let recurse = string_to_bool(args[6].clone());

        Ok(Config {
            query,
            path,
            case,
            verbose,
            whole,
            recurse,
        })
    }
}

/// Case-sensitive search
/// Returns all the lines of the contents where the query is present
pub fn search_case_sensitive(query: &str, contents: &str) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.to_string());
        }
    }

    Ok(results)
}

/// Case-insensitive search
/// Returns all the lines of the contents where the query is present
pub fn search_case_insensitive(query: &str, contents: &str) -> Result<Vec<String>, String> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.to_string());
        }
    }

    Ok(results)
}

/// Recursive search
/// Returns all the lines of all the files and folders mentioned in the config where the query is present
pub fn search_recursive(config: &Config) -> Result<Vec<String>, String> {
    let path = Path::new(&config.path);
    if !path.is_dir() {
        return Err(String::from(RECURSE_NOT_DIR));
    }

    let mut all_results = Vec::new();

    for entry in WalkDir::new(&config.path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                let results = if config.case {
                    search_case_sensitive(&config.query, &contents)?
                } else {
                    search_case_insensitive(&config.query, &contents)?
                };

                for line in results {
                    all_results.push(line.to_string());
                }
            }
            Err(msg) => {
                return Err(format!(
                    "Failed to read file: {}\n{}",
                    file_path.display(),
                    msg
                ));
            }
        }
    }

    Ok(all_results)
}

/// Serializes a string into a boolean
/// Case insensitive
/// If the input string is not a boolean, eprintln!("An error occurred during a string_to_bool operation")
/// Default return value : false
fn string_to_bool(string: String) -> bool {
    let str = string.to_lowercase();
    let mut is_bool = false;
    if str == "true" {
        is_bool = true;
    } else if str == "false" {
        is_bool = false;
    } else {
        eprintln!("An error occurred during a string_to_bool operation");
    }

    is_bool
}
