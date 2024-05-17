use std::env;
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let filename: &String = &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents =fs::read_to_string(filename)
        .expect("Cannot read the file. Ensure the title and the file extension are ok");

    println!("With text:\n{}", contents);
}
