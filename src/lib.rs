use std::fs;
use std::env;
use std::process;

use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            if args.len() > 1 {
                if args[1].clone() == String::from("-h") {
                    println!("\nminigrep: a tool to search for stuff in files
================================================
Usage: minigrep [query] [filename] [options]

Options:
  -c: Case insensitive search
  -h: Help
================================================
");
                    process::exit(0);
                }
            }
            return Err("not enough arguments.
================================================
Usage: minigrep [query] [filename] [options]

Options:
  -c: Case insensitive search
  -h: Help
================================================");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = if args.len() > 3 {
            if args[3].clone() == String::from("c") {
                true
            } else {
                false
            }
        } else {
            env::var("CASE_INSENSITIVE").is_err()
        };

        return Ok(Config {query, filename, case_sensitive});
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in &results {
        println!("{}", line);
    }

    if results.len() == 0 as usize {
        println!("Query not found in file.");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
