use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

mod guess;

use guess::Guess;

fn read_username_from_file(name: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(name)?;

    // let mut username_file = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() -> Result<(), Box<dyn Error>> {
    let name = "hello.txt";
    /*
        let _greeting = match File::open(name) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(name) {
                    Ok(new_file) => new_file,
                    Err(e) => panic!("Problem while creating file: {e}"),
                },
                _ => {
                    panic!("Problem opening the file: {error}");
                }
            },
        };
    */

    let _greeting = File::open(name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(name).unwrap_or_else(|error| {
                panic!("Problem while creating file: {error}");
            })
        } else {
            panic!("Problem opening the file: {error}");
        }
    });

    read_username_from_file(name)?;

    loop {
        println!("Enter a number guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = match Guess::new(guess) {
            Ok(guess) => guess,
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };

        println!("Your guess: {}", guess.value());

        break;
    }

    Ok(())
}
