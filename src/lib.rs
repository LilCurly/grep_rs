use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.filename)?;
    
    for line in search(&config.query, &file_content) {
        println!("{}", line);
    }

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_case_sensitive() {
        let query = "his";
        let content = "\
His palms are sweaty, knees weak, arms are heavy
There's vomit on his sweater already, mom's spaghetti
He's nervous, but on the surface he looks calm and ready";

        assert_eq!(vec!["There's vomit on his sweater already, mom's spaghetti"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "his";
        let content = "\
His palms are sweaty, knees weak, arms are heavy
There's vomit on his sweater already, mom's spaghetti
He's nervous, but on the surface he looks calm and ready";

        assert_eq!(vec!["His palms are sweaty, knees weak, arms are heavy", "There's vomit on his sweater already, mom's spaghetti"],
                        search_case_insensitive(query, content));
    }
}