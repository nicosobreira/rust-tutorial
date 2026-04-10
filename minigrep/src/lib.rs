use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod error;
pub use error::CustomError;

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub files_paths: Vec<PathBuf>,
}

pub fn run(config: &Config) -> Result<(), CustomError> {
    for file_path in &config.files_paths {
        if let Err(e) = process_file(file_path, &config.pattern) {
            eprintln!("Failed to process {}: {e}", file_path.to_string_lossy());
        }
    }

    Ok(())
}

fn process_file(file_path: &PathBuf, pattern: &str) -> Result<(), CustomError> {
    let file = File::open(file_path).map_err(CustomError::Io)?;

    let reader = BufReader::new(file);

    find_and_print_matches(reader, pattern, file_path)?;

    Ok(())
}

fn find_and_print_matches<'a, R: BufRead>(
    reader: R,
    pattern: &str,
    file_path: &'a PathBuf,
) -> Result<(), CustomError> {
    let separator = ":".cyan();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.map_err(CustomError::Io)?;

        if line.contains(pattern) {
            println!(
                "{}{separator}{}{separator}{}",
                file_path.to_string_lossy().purple(),
                line_number.to_string().green(),
                line
            );
        }
    }

    Ok(())
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, CustomError> {
        args.next();

        let pattern = match args.next() {
            Some(p) => p,
            None => return Err(CustomError::InvalidPattern),
        };

        let files_paths: Vec<PathBuf> = args.map(PathBuf::from).collect();

        if files_paths.is_empty() {
            return Err(CustomError::InvalidFiles);
        }

        Ok(Self {
            pattern,
            files_paths,
        })
    }
}
