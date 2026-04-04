use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Error: {error}");
        process::exit(1);
    });

    if let Err(error) = run(&config) {
        eprintln!("Error: {error}!");
        process::exit(2);
    };
}
