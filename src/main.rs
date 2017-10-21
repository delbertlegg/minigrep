extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let args : Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(|err| {
        println!("{}",  err);
        process::exit(1);
    });
}

