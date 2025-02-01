use std::env;
use std::process;

use searcher_txt::Config;

/// Main function called when the app is run
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("An error ocurred");
        eprintln!("Problem parsing arguments : {}", err);
        println!("Arguments needed : query, filename, case sensitivity");
        println!("                   word   file/path   boolean");
        process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file \"./{}\"", config.filename);
    println!("Lines that contain \"{}\" :", config.query);

    if let Err(e) = searcher_txt::run(config) {
        println!("An error ocurred");
        eprintln!("Application error : {}", e);
        process::exit(1);
    }
}
