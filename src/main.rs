use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = get_config(&args);

    println!("query : {:?}, filename : {:?}", config.query, config.filename);

    let file_content = fs::read_to_string(&config.filename)
        .expect(format!("failed to read file : '{}'", &config.filename).as_str());

    println!("Content of the file :\n{}", file_content);
}

fn get_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {
        query,
        filename
    }
}

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String
}
