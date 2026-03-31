fn largest(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}

fn main() {
    println!("Hello, world!");
}
