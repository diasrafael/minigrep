use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Content to analyze:\n-----------");
    println!("{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Mandatoy arguments not provided: query string and file name.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }
}


