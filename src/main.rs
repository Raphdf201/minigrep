use std::env;
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =fs::read_to_string(config.filename)
        .expect("Cannot read the file. Ensure the title and the file extension are ok");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,

}

impl Config {
    fn new(args: &[String]) -> Config {
    let query: String = args[1].clone();
    let filename: String = args[2].clone();

    Config {query, filename}
    }
}
