use std::fs::File;
use std::io::{BufRead, BufReader};

mod error;

use error::CustomError;

pub struct Config {
    pattern: String,
    file_path: String,
}

struct Match<'a> {
    string: String,
    line: usize,
    file_path: &'a str,
}

pub fn run(config: &Config) -> Result<(), CustomError> {
    let file = file_open(&config.file_path)?;

    let reader = BufReader::new(file);

    let matches = find_matches(reader, config)?;

    print_matches(&matches);

    Ok(())
}

fn print_matches(matches: &[Match]) {
    for mat in matches {
        println!("{}:{}: {}", mat.file_path, mat.line, mat.string);
    }
}

fn file_open(name: &str) -> Result<File, CustomError> {
    let file = File::open(name).map_err(CustomError::Io)?;

    Ok(file)
}

fn find_matches<'a, R: BufRead>(
    reader: R,
    config: &'a Config,
) -> Result<Vec<Match<'a>>, CustomError> {
    let mut matches = Vec::new();

    for (line, string) in reader.lines().enumerate() {
        let string = string.map_err(CustomError::Io)?;

        if string.contains(&config.pattern) {
            matches.push(Match {
                string: string,
                file_path: &config.file_path,
                line: line,
            });
        }
    }

    if matches.is_empty() {
        return Err(CustomError::MatchNotFound);
    } else {
        return Ok(matches);
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, CustomError> {
        if args.len() < 3 {
            return Err(CustomError::InvalidNumberOfArgs);
        }

        let pattern = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { pattern, file_path })
    }
}
