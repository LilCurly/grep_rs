use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("query : {:?}, filename : {:?}", query, filename);

    let file_content = fs::read_to_string(filename)
        .expect(format!("failed to read file : '{}'", filename).as_str());

    println!("Content of the file :\n{}", file_content);
}

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String
}
