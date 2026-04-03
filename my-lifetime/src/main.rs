mod trees;

use trees::{Texture, Tree, Trees};

fn main() {
    let s1 = "Olá, mundo!".to_string();
    let s2 = new("Oi!");

    println!("Largest: {}", largest(&s1, &s2));

    // let trees = Trees {
    //     texture: Texture {
    //         image: "^\n|".to_string(),
    //     },
    //     trees: Vec::new(),
    // };
}

fn largest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}

fn new(s: &str) -> String {
    let n = "aaaaaaaaaaaaaaaaa".to_string() + s;

    n
}
