use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

pub struct Config {
    pattern: String,
    file_path: String,
}

struct Match {
    string: String,
    line: usize,
}

pub fn run(config: &Config) -> Result<(), &'static str> {
    let file = file_open(&config.file_path);

    let mut reader = BufReader::new(file);

    let matches = find_matches(&mut reader, &config.pattern);

    match matches {
        Some(matches) => print_matches(&matches),
        None => return Err("No matches found!"),
    }

    Ok(())
}

fn print_matches(matches: &[Match]) {
    for matc in matches {
        println!("{}: {}", matc.line, matc.string);
    }
}

// FIX: The function wiil not panic if the "name" is a directory
// FIX: It also need to failed if the user doesn't have the permission to acess the file
fn file_open(name: &str) -> File {
    File::open(name).unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => panic!("The file \"{}\" does not exist!", name),
        ErrorKind::PermissionDenied => panic!("Read permission denied on file \"{name}\"!"),
        ErrorKind::IsADirectory => panic!("This \"{name}\" is a directory!"),
        _ => panic!("{error}"),
    })
}

fn find_matches(reader: &mut BufReader<File>, pattern: &str) -> Option<Vec<Match>> {
    let mut matches = Vec::new();

    for (n, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");

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
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("The number of arguments must be exactly three");
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
