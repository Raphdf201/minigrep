use std::error::Error;
use std::fs;

/// Runs a search with the provided config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitivity {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// Parameters of the application
/// query : the string to search
/// filename : the file to search in
/// case_sensitivity : tells if the search needs to be case-sensitive
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitivity: bool,
}

/// Implementation of the Config struct
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Missing 2 arguments");
        } else if args.len() < 4 {
            return Err("Missing 1 argument");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitivity = string_to_bool(args[3].clone());

        Ok(Config {
            query,
            filename,
            case_sensitivity,
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
        println!("An error ocurred");
        eprintln!("Internal error : bad usage of searcher_txt::string_to_bool");
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
