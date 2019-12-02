use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config {
            query,
            filename
        }
    }
}