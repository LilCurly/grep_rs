use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error while parsing arguments: {}", err);
        process::exit(1);
    });

    println!("query : {:?}, filename : {:?}", config.query, config.filename);

    let file_content = fs::read_to_string(&config.filename)
        .expect(format!("failed to read file : '{}'", &config.filename).as_str());

    println!("Content of the file :\n{}", file_content);
}

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            query,
            filename
        })
    }
}