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
        if args.len() != 3 {
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
fn test_parse_args() {
    let args = [String::from("throw away"), String::from("first"), String::from("second")];
    match Config::new(&args) {
        Ok(c) => {
            assert_eq!(c.query, "first");
            assert_eq!(c.filename, "second");
        },
        Err(e) => panic!(e)
    };
}