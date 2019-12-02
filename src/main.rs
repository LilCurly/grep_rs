use std::env;
use std::process;

use grep_rs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error while parsing arguments: {}", err);
        process::exit(1);
    });

    println!("query : {:?}, filename : {:?}", config.query, config.filename);

    if let Err(e) = grep_rs::run(config) {
        println!("Error occured while running the program: {}", e);
        process::exit(1);
    }
}