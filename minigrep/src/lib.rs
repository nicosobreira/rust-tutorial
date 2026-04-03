use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};

pub struct Config {
    pub pattern: String,
    pub file_path: String,
}
struct Match {
    string: String,
    line: usize,
}

fn run(config: &Config) -> io::Result<()> {
    let file = file_open(&config.file_path);

    let reader = BufReader::new(file);

    let mut _matches: u32 = 0;
    for (n, line) in reader.lines().enumerate() {
        let line = line?;

        if line.contains(&config.pattern) {
            println!("{n}: {line}");
            _matches += 1;
        }
    }

    if _matches == 0 {
        eprintln!("No matches found!");
    }

    Ok(())
}

// FIX: The function wiil not panic if the "name" is a directory
// FIX: It also need to failed if the user doesn't have the permission to acess the file
fn file_open(name: &str) -> File {
    File::open(name).unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => {
            panic!("The file \"{}\" does not exist", name);
        }
        _ => panic!("{error}"),
    })
}

fn find_pattern(reader: &mut BufReader<File>, pattern: &str) -> Option<Vec<Match>> {
    let mut matches = Vec::new();

    for (n, line) in reader.lines().enumerate() {
        let line = line.expect("a");

        if line.contains(pattern) {
            let matc = Match::new(&line, n);
            matches.push(matc);
        }
    }

    if matches.len() == 0 {
        return None;
    } else {
        return Some(matches);
    }
}

impl Config {
    fn parse(args: &[String]) -> Self {
        if args.len() != 3 {
            panic!("This program only supports two arguments!");
        }

        let pattern = args[1].clone();
        let file_path = args[2].clone();

        Self { pattern, file_path }
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
