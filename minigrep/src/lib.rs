use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod error;
use error::CustomError;

pub struct Config {
    pattern: String,
    files_paths: Vec<PathBuf>,
}

struct Match<'a> {
    string: String,
    file_path: &'a PathBuf,
    line: usize,
}

pub fn run(config: &Config) -> Result<(), CustomError> {
    for file_name in &config.files_paths {
        let file = match file_open(file_name) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open {}: {e}", file_name.to_string_lossy());
                continue;
            }
        };

        let reader = BufReader::new(file);

        match find_matches(reader, &config.pattern, file_name) {
            Ok(matches) => print_matches(&matches),
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    Ok(())
}

fn print_matches(matches: &[Match]) {
    let separator = ":".cyan();

    for mat in matches {
        println!(
            "{}{separator}{}{separator}{}",
            mat.file_path.to_string_lossy().purple(),
            mat.line.to_string().green(),
            mat.string
        );
    }
}

fn file_open(name: &PathBuf) -> Result<File, CustomError> {
    let file = File::open(name).map_err(CustomError::Io)?;

    Ok(file)
}

fn find_matches<'a, R: BufRead>(
    reader: R,
    pattern: &str,
    file_path: &'a PathBuf,
) -> Result<Vec<Match<'a>>, CustomError> {
    let mut matches = Vec::new();

    for (line, string) in reader.lines().enumerate() {
        let string = string.map_err(CustomError::Io)?;

        if string.contains(pattern) {
            matches.push(Match {
                string,
                file_path,
                line: line + 1,
            });
        }
    }

    Ok(matches)
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, CustomError> {
        args.next();

        let pattern = match args.next() {
            Some(p) => p,
            None => return Err(CustomError::InvalidPattern),
        };

        let files_paths = args.map(PathBuf::from).collect();

        Ok(Self {
            pattern,
            files_paths,
        })
    }
}
