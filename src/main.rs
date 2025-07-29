use std::error::Error;
use std::{fs, process};

use clap::Parser;

use searcher_txt::*;

/// Main function called when the app is run through the CLI
fn main() {
    let config = Config::parse();

    print!("Searching for \"{}\"", config.query);
    println!(" in \"{}\"", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

/// Runs a search with the provided config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.recurse {
        search_recursive(&config)?;
    } else {
        if config.verbose {
            println!("Parsing file {}", config.filename);
        }
        let contents = fs::read_to_string(&config.filename)?;
        if config.whole {
            println!("Contents of search :\n{}\n", contents);
        }

        let results = if config.case {
            if config.verbose {
                println!("Running case-sensitive search");
            }
            search_case_sensitive(&config.query, &contents)
        } else {
            if config.verbose {
                println!("Running case-insensitive search");
            }
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
    }

    Ok(())
}
