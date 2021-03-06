use std::{fs, error::Error};
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;


    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("Result found: {}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(&query.to_lowercase()))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        
        //bypassing first parameter, i.e., the name of the program
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() -> () {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn it_founds_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn it_creates_new_config() {
        
        let c = Config::new(&vec![
            "path".to_string(), String::from("query"), String::from("filename")]
        ).unwrap();

        assert_eq!("query", c.query);
        assert_eq!("filename", c.filename);
    }

    #[test]
    #[should_panic(expected = "Mandatoy arguments not provided: query string and file name.")]
    fn it_fails_to_create_with_no_args() {
        let _c: Config = Config::new(&vec!["path".to_string()]).unwrap();
    }
}