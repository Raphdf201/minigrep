use std::process;

use clap::Parser;

use searcher_txt::Config;

/// Main function called when the app is run
fn main() {
    let config = Config::parse();

    print!("Searching for \"{}\"", config.query);
    println!(" in \"{}\"", config.filename);

    if let Err(e) = searcher_txt::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
