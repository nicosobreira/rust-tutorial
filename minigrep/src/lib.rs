use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod error;

use error::CustomError;

pub struct Config {
    pattern: String,
    file_path: String,
}

struct Match {
    string: String,
    line: usize,
}

pub fn run(config: &Config) -> Result<(), CustomError> {
    let file = file_open(&config.file_path)?;

    let mut reader = BufReader::new(file);

    let matches = find_matches(&mut reader, &config.pattern)?;

    print_matches(&matches);

    Ok(())
}

fn print_matches(matches: &[Match]) {
    for mat in matches {
        println!("{}: {}", mat.line, mat.string);
    }
}

fn file_open(name: &str) -> Result<File, CustomError> {
    if Path::new(name).is_dir() {
        return Err(CustomError::IsDirectory(name.to_string()));
    }

    let file = File::open(name).map_err(CustomError::Io)?;

    Ok(file)
}

fn find_matches(reader: &mut BufReader<File>, pattern: &str) -> Result<Vec<Match>, CustomError> {
    let mut matches = Vec::new();

    for (line, string) in reader.lines().enumerate() {
        let string = string.expect("Failed to read line");

        if string.contains(pattern) {
            let mat = Match::new(&string, line);
            matches.push(mat);
        }
    }

    if matches.len() == 0 {
        return Err(CustomError::MatchNotFound);
    } else {
        return Ok(matches);
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, CustomError> {
        if args.len() != 3 {
            return Err(CustomError::InvalidNumberOfArgs);
        }

        let pattern = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { pattern, file_path })
    }
}

impl Match {
    fn new(string: &str, line: usize) -> Self {
        Self {
            string: String::from(string),
            line: line,
        }
    }
}
