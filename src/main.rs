use std::process;

use clap::Parser;

use searcher_txt::Config;

/// Main function called when the app is run
fn main() {
    let config = Config::parse();

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);
    println!("Lines that contain \"{}\":", config.query);

    if let Err(e) = searcher_txt::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
