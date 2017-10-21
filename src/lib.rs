#[macro_use]
extern crate indoc;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(indoc!("
                Not enough arguments...correct format is:
                    cargo run <query> <filename>"
                    ));
        }
        Ok(Config {
            query : args[1].clone(),
            filename: args[2].clone()
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

#[cfg(test)]

#[test]
fn test_new_config_happy() {
    let args = [String::from("throw away"), String::from("first"), String::from("second")];
    let result = Config::new(&args).unwrap();
    assert_eq!(result.query, "first");
    assert_eq!(result.filename, "second");
}

#[test]
fn test_new_config_fail() {
    let args = [String::from("first"), String::from("second")];
    let result = Config::new(&args);
    assert!(result.is_err());
}

#[test]
fn test_run_happy() {
    let config : Config = Config { query : String::from("hi"), filename : String::from("poem.txt")};
    let result : () = run(config).unwrap();
    assert_eq!(result, ());
}

#[test]
#[should_panic]
fn test_run_fail() {
    let config : Config = Config { query : String::from("hi"), filename : String::from("notthere.txt")};
    run(config).unwrap();
}