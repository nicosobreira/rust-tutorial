use std::env;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse(&args);

    run()?
}
