use std::{env, process};
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = lib::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);git 
    });

    println!("Searching for '{}' in file '{}'", config.query, config.filename);
    if let Err(e) = lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };

}

