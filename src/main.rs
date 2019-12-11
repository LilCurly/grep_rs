use std::env;
use std::process;

use grep_rs::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments: {}", err);
        process::exit(1);
    });

    println!("query : {:?}, filename : {:?} case_sensitive: {:?}", config.query, config.filename, config.case_sensitive);

    if let Err(e) = grep_rs::run(config) {
        eprintln!("Error occured while running the program: {}", e);
        process::exit(1);
    }
}