use crate::search;
use crate::Config;
use std::error::Error;
use std::fs;
use colored::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = search(&config.query, &contents, &config.ignore_case);
    print_results(results, &config);
    Ok(())
}

pub fn print_results(results: Vec<&str>, config: &Config) {
    for line in results {
      let highlighted = line.replace(&config.query, &config.query.red().bold().to_string());
      println!("{}", highlighted); 
    }
}