use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("This program only supports two arguments!");
    }

    let pattern = args.get(1).expect("Plese, enter a match!");
    let file_name = args.get(2).expect("Please, enter a file name!");

    let file = file_open(file_name);

    let reader = BufReader::new(file);

    let mut _matches: u32 = 0;
    for (n, line) in reader.lines().enumerate() {
        let line = line?;

        if line.contains(pattern) {
            println!("{n}: {line}");
            _matches += 1;
        }
    }

    if _matches == 0 {
        eprintln!("No matches found!");
    }

    Ok(())
}
