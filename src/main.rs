use std::{env, process};
mod lib;

fn main() {
     let config = lib::Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}' in file '{}'", config.query, config.filename);
    if let Err(e) = lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };

}

