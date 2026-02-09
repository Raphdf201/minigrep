use clap::Parser;
use std::error::Error;
use std::time::SystemTime;
use std::{fs, process};

use searcher_txt::*;

fn main() {
    let now = SystemTime::now();
    let config = Config::parse();

    print!("Searching for \"{}\"", config.query);
    print!(" in \"{}\" ", config.path);
    if config.verbose {
        println!("with arguments");
        println!("query : {}", config.query);
        println!("path : {}", config.path);
        println!("verbose : true");
        println!("recurse : {}", config.recurse);
        println!("case sensitive : {}", config.case);
        println!("whole : {}", config.whole);
    } else { println!("\n"); }

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    print!("Finished in {}ms", now.elapsed().unwrap().as_millis());
}

/// Runs a search with the provided config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let results;
    if config.recurse {
        results = search_recursive(&config);
    } else {
        if config.verbose {
            println!("Parsing file {}", config.path);
        }

        let contents = fs::read_to_string(&config.path)?;

        if config.whole {
            println!("Contents of search :\n{}\n", contents);
        }

        results = if config.case {
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
    }

    if results.is_err() {
        eprintln!("{}", results.err().unwrap());
    } else if results.clone()?.is_empty() {
        println!("Found no line containing {}", config.query);
    } else {
        println!("Lines that contain \"{}\":", config.query);
        for line in results? {
            println!("{}", line);
        }
    }

    Ok(())
}
