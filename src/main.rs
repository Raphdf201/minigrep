use std::env;
use std::process;

use searcher_txt::run;
use searcher_txt::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments : {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error : {}", e);
        process::exit(1);
    }
}
