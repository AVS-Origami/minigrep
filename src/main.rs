use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("\nProblem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("\nSearching for query: '{}' in file: '{}'", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("\nApplication error: {}", e);
        process::exit(1);
    }
    
}